//! Web crawler for collecting site information and sites linked to from this site

use std::collections::{HashMap, VecDeque};

use slotmap::{new_key_type, SlotMap};

new_key_type! {pub struct SiteKey;}

/// A webcrawling agent that parses a site's metadata and adds all links found within to a queue to
/// do the same to
#[derive(Default, Debug)]
pub struct WebCrawler {
    pub site_pool: SlotMap<SiteKey, SiteLog>,
    pub url_to_keys: HashMap<String, SiteKey>,
    pub site_queue: VecDeque<String>,
}

impl WebCrawler {
    /// Adds a URL to the crawling queue
    pub fn enqueue<S: Into<String>>(&mut self, input: S) {
        self.site_queue.push_back(input.into());
    }
    /// Crawls through the site queue, adding sites to the site pool and
    pub async fn crawl(&mut self) -> Option<()> {
        if let Some(url) = self.site_queue.pop_front() {
            self.parse_site(url).await
        } else {
            None
        }
    }

    pub fn urls_within_site(text: &str) -> Option<Vec<String>> {
        let mut hrefs = vec![];

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

                hrefs.push(url);
            } else {
                remaining.pop();
            }
        }

        Some(hrefs)
    }

    pub async fn parse_site<S: AsRef<str>>(&mut self, url: S) -> Option<()> {
        let response = reqwest::get(url.as_ref()).await.ok()?;
        let url_str = url.as_ref().to_string();
        let html = response.text().await.ok()?;

        let site_key = if !self.url_to_keys.contains_key(url.as_ref()) {
            let key = self.site_pool.insert(SiteLog::default());
            self.url_to_keys.insert(url_str, key);
            key
        } else {
            self.url_to_keys[url.as_ref()]
        };

        let site_entry = &mut self.site_pool[site_key];
        site_entry.url = url.as_ref().to_string();

        // TODO: Parse site, store title and all outgoing connections
        let hrefs = WebCrawler::urls_within_site(&html)?;

        for href in hrefs {
            todo!("Parse href by creating new empty SiteLog and enqueing it. Add key created from slotmap to the current SiteEntry's connections");
        }

        Some(())
    }
}

/// Tracked information about a site
#[derive(Default, Debug, PartialEq)]
pub struct SiteLog {
    pub url: String,
    pub title: String,
    pub connections: Vec<SiteKey>,
}
