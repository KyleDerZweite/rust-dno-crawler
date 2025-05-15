## Project: rust-dno-crawler

**Description**

`rust-dno-crawler` is a high-performance, asynchronous web crawler for collecting and standardizing data from German Distribution Network Operators (DNOs). It uses Tokio for concurrency, Reqwest for HTTP requests, Scraper for HTML parsing, and Serde for data serialization. Integration with a local Ollama instance allows downstream AI-based interpretation and normalization of unstructured data.

---

### Features

* Asynchronous, concurrent crawling with configurable limits.
* Directed brute-crawl strategy with prioritized URL queuing.
* Automatic scheduling (yearly, interval-based) via configuration.
* Configurable file-pattern matching for PDF and data extraction.
* JSON-based data store (`data_store.json`) matching existing schema.
* HTTP interface to Ollama for AI-based data interpretation.

---

### Roadmap / Next Steps

1. **Project Initialization**: Create Cargo.toml, src scaffolding.
2. **Configuration Module**: Implement Serde structs for `data_store.json`.
3. **Crawler Core**: Tokio + Reqwest + Scraper + Semaphore-based concurrency.
4. **Data Extraction**: Pattern-based discovery and download.
5. **Ollama Client**: HTTP wrapper for prompt-based analysis.
6. **CLI & Scheduler**: Command-line options and auto-crawl intervals.
7. **Testing & Documentation**: Unit tests, integration tests, full documentation.

---

### Usage

1. Clone repository:

   ```bash
   git clone https://github.com/your-org/rust-dno-crawler.git
   cd rust-dno-crawler
   ```

2. Configure your `data_store.json` in the project root (template provided):

   ```json
   {
     "example-dno": {
       "metadata": { /* ... */ },
       "crawl": { /* ... */ },
       "data": { /* ... */ }
     }
   }
   ```

3. Build and run:

   ```bash
   cargo build --release
   ./target/release/rust-dno-crawler --config data_store.json
   ```

---

### Configuration Format (`data_store.json`)

Structure must follow the existing schema:

```json
{
  "<dno-key>": {
    "metadata": { /* name, description, region */ },
    "crawl": { /* type, source URLs, file patterns, settings */ },
    "data": { /* year-indexed results */ }
  }
}
```

Refer to `/config/template_data_store.json` for a full example.

---

### Folder Structure

```
rust-dno-crawler/
├── src/
│   ├── main.rs
│   ├── crawler.rs
│   ├── config.rs
│   ├── ollama_client.rs
│   └── data_store.rs
├── config/
│   └── template_data_store.json
├── assets/       # downloaded files/PDFs
├── README.md
└── Cargo.toml
```

---

### License

MIT © 2025
