mod crawler;
mod ollama_client;
mod data_store;
mod config;
mod web_search;


fn main() {
    let _ = web_search::search_xng("RheinNetz GmbH (RNG)").unwrap_or_else(
        |err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    );
}
