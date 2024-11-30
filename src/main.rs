use page_rank_from_scratch::crawler::WebCrawler;

/// How many sites to scrape for our fake internet
pub const SITES_TO_SCRAPE: usize = 10_000;

#[tokio::main]
async fn main() {
    let mut crawler = WebCrawler::default();
    crawler.enqueue("https://en.wikipedia.org/wiki/Main_Page");

    for i in 0..SITES_TO_SCRAPE {
        if crawler.crawl().await.is_none() {
            println!("Oops ran out of entries on entry {i}");
        }
    }

    crawler.save("10000sites.json").expect("Failed to save");
}
