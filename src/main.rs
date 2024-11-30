use page_rank_from_scratch::crawler::WebCrawler;

/// How many sites to scrape for our fake internet
pub const SITES_TO_SCRAPE: usize = 100;

#[tokio::main]
async fn main() {
    let mut crawler = WebCrawler::default();
    crawler.enqueue("https://en.wikipedia.org/wiki/Main_Page");

    for _ in 0..SITES_TO_SCRAPE {
        if crawler.crawl().await.is_none() {
            panic!("Oops ran out of entries");
        }
    }

    crawler.save("100sites.json").expect("Failed to save");
}
