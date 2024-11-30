use page_rank_from_scratch::{crawler::WebCrawler, page_rank::PageRanker};

#[tokio::main]
async fn main() {
    let page_registry =
        WebCrawler::load("1000sites.json").expect("Failed to load page registry from file");
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
}
