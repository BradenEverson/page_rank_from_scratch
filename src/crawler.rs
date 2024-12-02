//! Web crawler for collecting site information and sites linked to from this site

use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, SlotMap};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

new_key_type! {pub struct SiteKey;}

/// A webcrawling agent that parses a site's metadata and adds all links found within to a queue to
/// do the same to
#[derive(Debug)]
pub struct WebCrawler {
    pub site_pool: SlotMap<SiteKey, SiteLog>,
    pub site_queue: UnboundedReceiver<SiteKey>,
    pub site_queue_sender: UnboundedSender<SiteKey>,
    pub visited: HashSet<String>,
}

impl Default for WebCrawler {
    fn default() -> Self {
        let (sender, receiver) = unbounded_channel();
        Self {
            site_queue: receiver,
            site_queue_sender: sender,
            site_pool: SlotMap::default(),
            visited: HashSet::new(),
        }
    }
}

impl WebCrawler {
    /// Adds a URL to the crawling queue
    pub fn enqueue<S: Into<String>>(&mut self, input: S) -> SiteKey {
        let site_log = SiteLog {
            url: input.into(),
            ..Default::default()
        };
        let inserted = self.site_pool.insert(site_log);
        let _ = self.site_queue_sender.send(inserted);

        inserted
    }

    /// Saves the site_pool slotmap as a JSON file
    pub fn save<P: Into<PathBuf>>(&mut self, file: P) -> Option<()> {
        let mut file = File::create_new(file.into()).ok()?;
        file.write_all(serde_json::to_string(&self.site_pool).ok()?.as_bytes())
            .ok()?;

        Some(())
    }

    /// Loads a site slotmap from a JSON file
    pub fn load<P: Into<PathBuf>>(file: P) -> Option<SlotMap<SiteKey, SiteLog>> {
        let mut file = File::open(file.into()).ok()?;
        let mut buf = String::new();

        file.read_to_string(&mut buf).ok()?;
        serde_json::from_str(&buf).ok()
    }

    /// Crawls through the site queue, adding sites to the site pool and
    pub async fn crawl(&mut self) -> Option<()> {
        if let Some(url) = self.site_queue.recv().await {
            self.parse_site(url).await
        } else {
            None
        }
    }

    pub fn urls_and_title_within_site(text: &str, root_url: &str) -> Option<(String, Vec<String>)> {
        let mut hrefs = HashSet::new();
        let mut name = String::new();

        let mut remaining = text.chars().rev().collect::<String>();
        while !remaining.is_empty() {
            if remaining.ends_with("=ferh") {
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();

                let open_quote = remaining.pop()?;
                let mut url = String::new();
                while let Some(character) = remaining.pop() {
                    if character == open_quote {
                        break;
                    }
                    url.push(character);
                }

                if !url.starts_with("http") {
                    url = format!("{root_url}{url}");
                }

                if let Some((normalized, _)) = url.split_once('?') {
                    hrefs.insert(normalized.to_string());
                } else if let Some((normalized, _)) = url.split_once('#') {
                    hrefs.insert(normalized.to_string());
                } else {
                    hrefs.insert(url);
                }
            } else if remaining.ends_with(">eltit<") {
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();
                remaining.pop();

                while let Some(character) = remaining.pop() {
                    if remaining.ends_with("/<") {
                        break;
                    }

                    name.push(character);
                }
            } else {
                remaining.pop();
            }
        }

        Some((name.trim().to_string(), hrefs.into_iter().collect()))
    }

    pub async fn parse_site(&mut self, url: SiteKey) -> Option<()> {
        let site = &mut self.site_pool[url];
        let response = reqwest::get(&site.url).await.ok()?;
        self.visited.insert(site.url.clone());

        let html = response.text().await.ok()?;

        let mut root_url = String::new();
        let mut remaining = site.url.chars().rev().collect::<String>();

        while !remaining.is_empty() {
            if remaining.ends_with("//") {
                remaining.pop();
                remaining.pop();

                root_url.push('/');
                root_url.push('/');
            }
            if let Some(character) = remaining.pop() {
                if character == '/' {
                    break;
                }
                root_url.push(character);
            }
        }

        let (title, hrefs) = WebCrawler::urls_and_title_within_site(&html, &root_url)?;

        let hrefs: Vec<_> = hrefs
            .into_iter()
            .filter_map(|href| {
                if href.starts_with("http")
                    && !self.visited.contains(&href)
                    && self
                        .site_pool
                        .iter()
                        .filter(|(_, log)| log.url == href)
                        .count()
                        == 0
                {
                    Some(self.enqueue(href))
                } else {
                    None
                }
            })
            .collect();

        hrefs.iter().for_each(|key| {
            let _ = self.site_queue_sender.send(*key);
        });
        self.site_pool[url].connections.extend(hrefs);

        // Add self connection
        self.site_pool[url].connections.push(url);
        self.site_pool[url].title = title;

        Some(())
    }
}

/// Tracked information about a site
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteLog {
    pub url: String,
    pub title: String,
    pub connections: Vec<SiteKey>,
}

#[cfg(test)]
mod tests {
    use crate::crawler::WebCrawler;

    #[test]
    fn url_dupes_spotted() {
        let text = r#"<a href="https://example.com/path">Link</a>
              <a href="https://example.com/path?query=value">Link</a>
              <a href="https://example.com/path#section">Link</a>"#;

        let root_url = "https://example.com";
        let result = WebCrawler::urls_and_title_within_site(text, root_url);
        assert_eq!(
            result,
            Some(("".to_string(), vec!["https://example.com/path".to_string()]))
        );
    }
}
