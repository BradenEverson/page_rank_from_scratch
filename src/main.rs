use page_rank_from_scratch::{crawler::WebCrawler, page_rank::PageRanker};

/*
#[tokio::main]
async fn main() {
    let page_registry = WebCrawler::load("100_sites_with_roots.json")
        .expect("Failed to load page registry from file");
    let pageranker = PageRanker::from_registry(page_registry);

    let search_term = "wiki";

    if let Some(rankings) = pageranker.search(search_term) {
        println!("Top results for \"{search_term}\":");
        for site in rankings.iter() {
            println!("\n{}\n\t{}\n", site.title, site.url)
        }
    } else {
        println!("Unable to generate top results for query, it's likely that our database isn't yet large enough :(");
    }
}*/

/// How many sites to scrape for our fake internet
pub const SITES_TO_SCRAPE: usize = 100;

#[tokio::main]
async fn main() {
    let mut crawler = WebCrawler::default();
    crawler.enqueue("https://en.wikipedia.org/wiki/Main_Page");

    for _ in 0..SITES_TO_SCRAPE {
        let _ = crawler.crawl().await;
    }

    crawler
        .save("100_sites_with_roots.json")
        .expect("Failed to save");
}
