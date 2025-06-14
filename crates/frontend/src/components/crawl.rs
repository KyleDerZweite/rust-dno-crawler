use dioxus::prelude::*;

#[component]
pub fn CrawlForm() -> Element {
    let mut url = use_signal(|| String::new());
    
    rsx! {
        div { class: "max-w-2xl mx-auto mb-8",
            div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-6 border border-stone-700",
                h2 { class: "text-xl font-semibold text-green-400 mb-4", "Start New Crawl Job" }
                div { class: "space-y-4",
                    input {
                        r#type: "url",
                        placeholder: "https://example-dno.de",
                        class: "w-full px-4 py-3 bg-stone-700 border border-stone-600 rounded-lg text-white placeholder-stone-400 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500",
                        value: "{url}",
                        oninput: move |evt| url.set(evt.value())
                    }
                    button {
                        class: "w-full px-6 py-3 bg-green-600 hover:bg-green-500 text-white rounded-lg transition-colors font-medium",
                        onclick: move |_| {
                            // TODO: Implement crawl job creation
                            tracing::info!("Creating crawl job for: {}", url());
                        },
                        "🕷️ Start Crawling"
                    }
                }
            }
        }
    }
}

#[component]
pub fn CrawlJobs() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto",
            h2 { class: "text-xl font-semibold text-green-400 mb-6", "Recent Crawl Jobs" }
            div { class: "space-y-4",
                CrawlJobCard {
                    url: "https://avacon-netz.de",
                    status: "Completed",
                    started: "2 minutes ago"
                }
                CrawlJobCard {
                    url: "https://bayernwerk-netz.de",
                    status: "In Progress",
                    started: "5 minutes ago"
                }
                CrawlJobCard {
                    url: "https://eon-netz.de",
                    status: "Pending",
                    started: "10 minutes ago"
                }
            }
        }
    }
}

#[component]
fn CrawlJobCard(url: String, status: String, started: String) -> Element {
    let status_color = match status.as_str() {
        "Completed" => "text-green-400",
        "In Progress" => "text-amber-400",
        "Failed" => "text-red-400",
        _ => "text-stone-400",
    };
    
    rsx! {
        div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-4 border border-stone-700",
            div { class: "flex items-center justify-between",
                div { class: "flex-1",
                    p { class: "text-white font-medium mb-1", "{url}" }
                    p { class: "text-stone-400 text-sm", "Started {started}" }
                }
                div { class: "text-right",
                    span { class: "px-3 py-1 rounded-full text-sm font-medium {status_color} bg-stone-700",
                        "{status}"
                    }
                }
            }
        }
    }
}