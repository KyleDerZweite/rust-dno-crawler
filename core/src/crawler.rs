use spider::tokio;
use spider::website::Website;
use std::collections::HashSet; // To keep track of unique PDF links
use scraper::{Html, Selector};

use crate::ollama_client::call_ollama;

const KEYWORD_WEIGHTS: [(&str, f32); 10] = [
    ("veröffentlichungen", 0.85), // Publications - very strong indicator
    ("veroeffentlichungen", 0.85), // Alternate spelling
    ("netzentgelte", 0.7),       // Network tariffs - strong indicator
    ("hochlastzeitfenster", 0.7),// Peak load time windows - strong indicator
    ("preisblätter", 0.65),       // Price sheets - strong indicator (preisblaetter)
    ("preisblaetter", 0.65),     // Price sheets - strong indicator
    ("entgelte", 0.6),           // Charges/tariffs - strong indicator
    ("downloads", 0.4),          // Downloads section - common location
    ("dokumente", 0.3),          // Documents section - common location
    ("netzanschluss", 0.2),      // Network connection - sometimes relevant
];

const NEGATIVE_KEYWORDS: [(&str, f32); 10] = [
    ("kontakt", 1.0), // Contact
    ("impressum", 1.0), // Legal notice
    ("datenschutz", 1.0), // Data privacy
    ("login", 1.0),     // Login page
    ("karriere", 0.8),  // Career
    ("presse", 0.5),    // Press
    ("news", 0.3),      // News (can sometimes be relevant, but often not for specific docs)
    ("blog", 0.5),
    ("?id=", 0.9),      // Generic ID parameters often lead to non-primary pages
    ("&id=", 0.9),      // Generic ID parameters
];

const FILE_KEYWORDS: [(&str, f32); 12] = [
    ("netzentgelte", 0.9),
    ("hochlastzeitfenster", 0.9),
    ("preisblatt", 0.8),
    ("preisblätter", 0.8),
    ("entgelte", 0.75),
    ("tarife", 0.7),
    ("netz", 0.5), // "network" - general but can be part of relevant file names
    ("strom", 0.4), // "electricity"
    ("gas", 0.4), // "gas"
    ("regulierung", 0.6),
    ("bedingungen", 0.5), // "conditions"
    ("veröffentlichung", 0.6), // "publication"
];

const PDF_SCORE_THRESHOLD: f32 = 0.7; // Minimum score for a PDF to be considered relevant
const OLLAMA_MODEL_NAME: &str = "deepseek-r1:7b"; // Replace with your actual Deepseek model name

const DISCOVERY_CRAWL_DEPTH: u32 = 2; // Depth for initial crawl to find promising pages/sections
const FOCUSED_PDF_CRAWL_DEPTH: u32 = 3; // Depth to crawl from a promising page to find PDFs
const PAGE_SCORE_THRESHOLD_FOR_FOCUSED_CRAWL: f32 = 0.5; // Min page score to trigger focused PDF crawl

fn rank_url(url_str: &str) -> f32 {
    let lower_url = url_str.to_lowercase();
    let mut score = 0.1; // Start with a small base score to avoid immediate 0.0

    // Penalize for negative keywords
    for (neg_keyword, penalty) in NEGATIVE_KEYWORDS.iter() {
        if lower_url.contains(neg_keyword) {
            score -= penalty;
        }
    }

    // If score is already very low after penalties, no need to check positive keywords
    if score <= 0.0 {
        return 0.0;
    }

    // Add points for positive keywords
    for (keyword, weight) in KEYWORD_WEIGHTS.iter() {
        if lower_url.contains(&keyword.to_lowercase()) {
            score += weight;
        }
    }

    // Penalize URLs that look like they have generic database IDs as primary identifiers
    // This is a simple check; more sophisticated regex could be used.
    if (lower_url.contains("?id=") || lower_url.contains("&id=")) && (lower_url.contains("catid=") || lower_url.contains("itemid=")) {
        // Check if these are the dominant parts of the query string
        if let Some(query_string) = url_str.split('?').nth(1) {
            if query_string.matches('&').count() >= 1 && query_string.len() > 30 { // Arbitrary length check
                 score *= 0.5; // Reduce score if it seems to be a complex ID-based query
            }
        }
    }
    
    // Ensure score is within [0.0, 1.0]
    score.max(0.0).min(1.0)
}

fn rank_pdf_file(pdf_url_str: &str) -> f32 {
    let lower_url = pdf_url_str.to_lowercase();
    // Try to extract filename
    let filename = lower_url.split('/').last().unwrap_or(&lower_url);
    
    let mut score = 0.0;

    for (keyword, weight) in FILE_KEYWORDS.iter() {
        if filename.contains(&keyword.to_lowercase()) {
            score += weight;
        }
    }
    // Simple check for year, might indicate relevance (e.g., "netzentgelte_2023.pdf")
    if filename.contains("2023") || filename.contains("2024") || filename.contains("2025") {
        score += 0.1;
    }

    score.min(1.0) // Cap score at 1.0
}


pub async fn crawl_website(website_url: &str) -> anyhow::Result<()> {
    println!("Starting multi-stage PDF search for {}...", website_url);

    let mut found_high_ranking_pdfs: Vec<(String, f32)> = Vec::new();
    let mut all_pages_crawled_for_ai_context: HashSet<String> = HashSet::new();
    let mut processed_focused_crawl_starts: HashSet<String> = HashSet::new();

    // Phase 1: Discovery Crawl
    println!("\nPhase 1: Discovery Crawl (Depth: {}) from {}", DISCOVERY_CRAWL_DEPTH, website_url);
    let mut initial_website = Website::new(website_url);
    initial_website.configuration.depth = DISCOVERY_CRAWL_DEPTH as usize;
    initial_website.configuration.delay = 200; // Politeness delay
    initial_website.crawl().await;

    let discovered_links = initial_website.get_links();
    let mut promising_page_urls_to_scan: Vec<(String, f32)> = Vec::new();

    if discovered_links.is_empty() {
        println!("No links found during initial discovery crawl.");
    } else {
        println!("Found {} links in discovery phase. Ranking pages...", discovered_links.len());
        for link in discovered_links {
            let url_str = link.as_ref().to_string();
            all_pages_crawled_for_ai_context.insert(url_str.clone()); // Add to AI context

            if !url_str.to_lowercase().ends_with(".pdf") { // We only rank pages here, not PDFs directly
                let page_rank = rank_url(&url_str);
                if page_rank >= PAGE_SCORE_THRESHOLD_FOR_FOCUSED_CRAWL {
                    promising_page_urls_to_scan.push((url_str, page_rank));
                }
            } else {
                // Directly check PDFs found in discovery phase
                let pdf_rank = rank_pdf_file(&url_str);
                if pdf_rank >= PDF_SCORE_THRESHOLD {
                     if !found_high_ranking_pdfs.iter().any(|(u, _)| u == &url_str) {
                        println!("Found potential PDF during discovery: {} (Score: {:.2})", url_str, pdf_rank);
                        found_high_ranking_pdfs.push((url_str.clone(), pdf_rank));
                    }
                }
            }
        }
    }
    
    promising_page_urls_to_scan.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    if !promising_page_urls_to_scan.is_empty() {
        println!("\nFound {} promising pages/sections to perform focused PDF crawls:", promising_page_urls_to_scan.len());
        for (url, rank) in promising_page_urls_to_scan.iter().take(10) { // Show top 10
            println!("- {:.2} - {}", rank, url);
        }
    } else {
        println!("\nNo promising pages found meeting threshold {:.2} in discovery phase.", PAGE_SCORE_THRESHOLD_FOR_FOCUSED_CRAWL);
    }

    // Phase 2: Focused PDF Crawl
    if !promising_page_urls_to_scan.is_empty() {
        println!("\nPhase 2: Focused PDF Crawl (Depth: {} from each promising page)", FOCUSED_PDF_CRAWL_DEPTH);
        for (page_url_to_scan, page_rank) in promising_page_urls_to_scan {
            if processed_focused_crawl_starts.contains(&page_url_to_scan) {
                continue; // Skip if already processed this as a starting point
            }
            processed_focused_crawl_starts.insert(page_url_to_scan.clone());

            println!("\nStarting focused crawl from: {} (Initial Page Score: {:.2})", page_url_to_scan, page_rank);
            let mut focused_website = Website::new(&page_url_to_scan);
            focused_website.configuration.depth = FOCUSED_PDF_CRAWL_DEPTH as usize;
            focused_website.configuration.delay = 200;
            focused_website.crawl().await;

            let focused_links = focused_website.get_links();
            if focused_links.is_empty() {
                println!("No links found during focused crawl from {}", page_url_to_scan);
                continue;
            }
            println!("Found {} links from {}. Filtering for PDFs...", focused_links.len(), page_url_to_scan);

            for link in focused_links {
                let url_str = link.as_ref().to_string();
                all_pages_crawled_for_ai_context.insert(url_str.clone()); // Add to AI context

                if url_str.to_lowercase().ends_with(".pdf") {
                    let pdf_rank = rank_pdf_file(&url_str);
                    if pdf_rank >= PDF_SCORE_THRESHOLD {
                        if !found_high_ranking_pdfs.iter().any(|(u, _)| u == &url_str) {
                            println!("Found potential PDF: {} (Score: {:.2})", url_str, pdf_rank);
                            found_high_ranking_pdfs.push((url_str.clone(), pdf_rank));
                        }
                    }
                }
            }
        }
    }


    if !found_high_ranking_pdfs.is_empty() {
        found_high_ranking_pdfs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        println!("\n--- Highly Ranked PDFs Found (Score >= {:.2}) ---", PDF_SCORE_THRESHOLD);
        for (link, rank) in &found_high_ranking_pdfs {
            println!("- {:.2} - {}", rank, link);
        }
    } else {
        println!("\nNo PDFs found meeting the threshold of {:.2} after discovery and focused crawls.", PDF_SCORE_THRESHOLD);
        println!("Ranking all crawled non-PDF pages to ask AI for assistance...");

        let mut ranked_pages_for_ai: Vec<(String, f32)> = all_pages_crawled_for_ai_context
            .iter()
            .filter(|url| !url.to_lowercase().ends_with(".pdf"))
            .map(|url_str| (url_str.clone(), rank_url(url_str)))
            .filter(|(_,rank)| *rank > 0.0) // Only consider pages with some relevance
            .collect();
        
        ranked_pages_for_ai.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if ranked_pages_for_ai.is_empty() {
            println!("No relevant non-PDF pages were crawled to provide context to AI.");
            return Ok(());
        }
        
        println!("\nTop 10 Ranked Non-PDF Pages (for AI context from all crawl phases):");
        for (link, rank) in ranked_pages_for_ai.iter().take(10) {
            println!("- {:.2} - {}", rank, link);
        }


        println!("\nAsking AI for assistance to find relevant PDFs...");
        
        let mut prompt_urls_context = String::new();
        for (link, rank) in ranked_pages_for_ai.iter().take(15) { // Send up to 15 pages to AI
            prompt_urls_context.push_str(&format!("- {} (Page Score: {:.2})\n", link, rank));
        }

        let prompt = format!(
            "I have performed a multi-stage crawl on the website '{}'.\n\
            First, a discovery crawl (depth {}) to find promising sections. \
            Then, focused crawls (depth {} each) from pages scoring above {} to find PDFs.\n\
            Despite this, I did not find PDF files that meet my criteria (score >= {}) for containing \"Netzentgelte\" (network tariffs/charges) and \"Hochlastzeitfenster\" (peak load time windows) for German DSOs. \
            The typical keywords I look for in PDF filenames are: {}.\n\
            Here are some of the top-ranked pages found across all crawl phases on the site, which might provide context:\n{}\n\
            Based on these pages and your knowledge, can you suggest:\n\
            1. Specific sub-sections or paths on '{}' that are most likely to host these types of PDF documents (e.g., 'Downloads', 'Veröffentlichungen', 'Netzinformationen', 'Preisblätter')?\n\
            2. Any specific search terms I could use on an internal site search, if available?\n\
            3. The most promising URL from the list above that might lead to or contain links to the required PDFs.\n\
            Please provide concise, actionable advice.",
            website_url, DISCOVERY_CRAWL_DEPTH, FOCUSED_PDF_CRAWL_DEPTH, PAGE_SCORE_THRESHOLD_FOR_FOCUSED_CRAWL, PDF_SCORE_THRESHOLD,
            FILE_KEYWORDS.iter().map(|(k,_)| *k).collect::<Vec<&str>>().join(", "), 
            prompt_urls_context, 
            website_url
        );

        match call_ollama(OLLAMA_MODEL_NAME, &prompt).await {
            Ok(ai_response) => {
                println!("\nDeepseek AI Suggestion:\n{}", ai_response);
            }
            Err(e) => {
                eprintln!("\nError calling Deepseek AI: {}", e);
            }
        }
    }
    Ok(())
}


pub async fn parse_and_filter_netzentgelte(website_url: &str) {
    let mut website = Website::new(website_url);

    // Configure the crawl
    website.configuration.depth = 1; // Only crawl the specified URL
    website.configuration.delay = 200; // Politeness delay
    website.configuration.respect_robots_txt = true; // Respect robots.txt

    println!("Starting crawl of {}", website_url);
    website.crawl().await;
    println!("Crawl finished.");

    // Get the crawled pages. With depth=1, this should contain only the initial page if successful.
    let pages = website.get_pages();

    if pages.expect("REASON").is_empty() {
        println!("No pages were crawled successfully for {}", website_url);
        return;
    }

    // Process the HTML content of the first (and likely only) page
    if let Some(page) = pages.expect("REASON").get(0) {
        let html_content = page.get_html();

        if html_content.is_empty() {
            println!("No HTML content found for {}", website_url);
            return;
        }

        println!("Successfully retrieved HTML content from {}", website_url);

        // Filter and extract information
        find_netzentgelte_downloads(&*html_content, website_url).await;

    } else {
        println!("Could not retrieve the content for the specified URL from the crawled pages.");
    }
}

async fn find_netzentgelte_downloads(html_content: &str, base_url: &str) {
    // Use scraper to parse the HTML
    let document = Html::parse_document(html_content);

    // Define a selector for links that might contain "Netzentgelte" or indicate a download.
    // This selector looks for <a> tags that:
    // 1. Contain the text "Netzentgelte" (case-insensitive search might be better in a real app)
    // 2. Or have a class like "download" or "btn-download" (based on common patterns and the image)
    // 3. Or have an href that likely points to a document (like .pdf, .doc, .xlsx, etc.)
    // We'll combine these for a broader search, then filter more precisely.

    // A robust selector targeting links potentially containing the keyword or indicating download
    let selector = Selector::parse("a:contains(\"Netzentgelte\"), a[class*=\"download\"], a[href$=\".pdf\"], a[href$=\".doc\"], a[href$=\".docx\"], a[href$=\".xls\"], a[href$=\".xlsx\"]")
        .expect("Failed to create selector");

    let mut found_downloads = Vec::new();

    for element in document.select(&selector) {
        let link_text = element.text().collect::<String>();
        let href = element.value().attr("href");

        if let Some(url) = href {
            // Basic check if the link text contains the keyword "Netzentgelte"
            // A more sophisticated check might involve checking surrounding text or parent elements
            if link_text.contains("Netzentgelte") || url.contains("Netzentgelte") {
                // Construct the absolute URL if it's relative
                let absolute_url = if url.starts_with("http") || url.starts_with("//") {
                    url.to_string()
                } else {
                    // Simple relative path joining; a proper URL join library is recommended for production
                    format!("{}/{}", base_url.trim_end_matches('/'), url.trim_start_matches('/'))
                };
                found_downloads.push((link_text.trim().to_string(), absolute_url));
            } else {
                // If the link text doesn't contain the keyword, but the selector matched (e.g., by class or file extension),
                // we could add additional checks here if needed to confirm relevance.
                // For now, we'll primarily rely on the keyword being present in text or URL.
            }
        }
    }

    if found_downloads.is_empty() {
        println!("No potential 'Netzentgelte' download links found.");
    } else {
        println!("Found potential 'Netzentgelte' download links:");
        for (text, url) in found_downloads {
            println!("- Text: '{}', URL: '{}'", text, url);
        }
    }
}