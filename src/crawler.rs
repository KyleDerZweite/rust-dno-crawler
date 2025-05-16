use reqwest::Client;
use scraper::{Html, Selector};
use spider::tokio;
use spider::website::Website;
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
};
use tokio::sync::Semaphore;

#[tokio::main]
pub async fn old_main() -> anyhow::Result<()> {
    let client = Client::builder().user_agent("MyRustCrawler/1.0").build()?;

    let max_concurrent = Arc::new(Semaphore::new(20));
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back("https://www.netze-bw.de/unternehmen/veroeffentlichungen#3-1".to_string());

    while let Some(url) = queue.pop_front() {
        if !visited.insert(url.clone()) {
            continue;
        }
        let permit = Arc::clone(&max_concurrent).acquire_owned().await?;
        let client = client.clone();
        tokio::spawn(async move {
            let resp = client.get(&url).send().await;
            drop(permit);
            println!("{:?}", resp);
            if let Ok(resp) = resp {
                if let Ok(text) = resp.text().await {
                    let document = Html::parse_document(&text);
                    println!("Crawled: {:?}", document);
                    let selector = Selector::parse("a[href]").unwrap();
                    for elem in document.select(&selector) {
                        if let Some(link) = elem.value().attr("href") {
                            println!("Crawled: {}", link);
                            // absolutisieren, filtern, etc.
                            // neue URLs in die Queue pushen (via Channel oder Mutex)
                        }
                    }
                    // Extrahiere Daten, speichere PDF-Links usw.
                    println!("Crawled: {}", text);
                }
            }
        });
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let mut website: Website = Website::new("https://spider.cloud");

    website.crawl().await;

    let links = website.get_links();

    for link in links {
        println!("- {:?}", link.as_ref());
    }
}
