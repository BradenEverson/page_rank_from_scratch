use indicatif::ProgressBar;
use page_rank_from_scratch::crawler::WebCrawler;

/// How many sites to scrape for our fake internet
pub const SITES_TO_SCRAPE: usize = 100_000;

#[tokio::main]
async fn main() {
    println!("Starting Crawler");
    let mut crawler = WebCrawler::default();
    crawler.enqueue("https://www.wikipedia.org/");

    let pb = ProgressBar::new(SITES_TO_SCRAPE as u64);

    for _ in 0..SITES_TO_SCRAPE {
        let _ = crawler.crawl().await;
        pb.inc(1);
    }

    crawler.save("100_000_wiki_entries.json");
    println!("Saved!");
}
