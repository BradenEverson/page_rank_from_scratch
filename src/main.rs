use std::io::{self, Write};

use page_rank_from_scratch::{crawler::WebCrawler, page_rank::PageRanker};

fn main() {
    let page_registry = WebCrawler::load("100_sites_with_roots.json")
        .expect("Failed to load page registry from file");
    let pageranker = PageRanker::from_registry(page_registry);

    loop {
        print!("Search: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input to buffer");

        let cleaned = input.trim();

        if let Some(rankings) = pageranker.search(cleaned) {
            println!("Top results for \"{cleaned}\":");
            for site in rankings.iter() {
                println!("\n{}\n\t{}\n", site.title, site.url)
            }
        } else {
            println!("Unable to generate top results for query, it's likely that our database isn't yet large enough :(");
        }
    }
}
