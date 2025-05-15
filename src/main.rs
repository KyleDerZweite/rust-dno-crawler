mod crawler;
mod ollama_client;
mod data_store;
mod config;

fn main() {
    let result = crawler::main();
    match result {
        Ok(_) => println!("Crawling completed successfully. {result:?}"),
        Err(e) => eprintln!("Error during crawling: {}", e),
    }
}
