pub mod components;
pub mod pages;
pub mod hooks;
pub mod services;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/search")]
    Search {},
    #[route("/dnos")]
    Dnos {},
    #[route("/crawl")]
    Crawl {},
    #[route("/settings")]
    Settings {},
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-stone-900 via-stone-800 to-stone-900",
            components::layout::Header {}
            main { class: "container mx-auto px-4 py-8",
                div { class: "text-center mb-12",
                    h1 { class: "text-5xl font-bold text-green-400 mb-4",
                        "🌿 DNO Data Gatherer"
                    }
                    p { class: "text-xl text-stone-300 max-w-2xl mx-auto",
                        "Automatically gather and process data from German Distribution Network Operators with AI-powered analysis."
                    }
                }
                
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-6xl mx-auto",
                    components::home::FeatureCard {
                        icon: "🔍",
                        title: "Smart Search",
                        description: "Search across multiple DNO websites with privacy-respecting SearXNG integration.",
                        link: "/search"
                    }
                    components::home::FeatureCard {
                        icon: "🕷️",
                        title: "Web Crawling",
                        description: "Automated crawling of DNO websites to extract structured data.",
                        link: "/crawl"
                    }
                    components::home::FeatureCard {
                        icon: "🤖",
                        title: "AI Processing",
                        description: "Local Ollama integration for intelligent data extraction and analysis.",
                        link: "/dnos"
                    }
                }
            }
            components::layout::Footer {}
        }
    }
}

#[component]
fn Search() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-stone-900 via-stone-800 to-stone-900",
            components::layout::Header {}
            main { class: "container mx-auto px-4 py-8",
                h1 { class: "text-3xl font-bold text-green-400 mb-8 text-center",
                    "🔍 Search DNO Data"
                }
                components::search::SearchForm {}
                components::search::SearchResults {}
            }
            components::layout::Footer {}
        }
    }
}

#[component]
fn Dnos() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-stone-900 via-stone-800 to-stone-900",
            components::layout::Header {}
            main { class: "container mx-auto px-4 py-8",
                h1 { class: "text-3xl font-bold text-green-400 mb-8 text-center",
                    "⚡ Distribution Network Operators"
                }
                components::dnos::DnoList {}
            }
            components::layout::Footer {}
        }
    }
}

#[component]
fn Crawl() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-stone-900 via-stone-800 to-stone-900",
            components::layout::Header {}
            main { class: "container mx-auto px-4 py-8",
                h1 { class: "text-3xl font-bold text-green-400 mb-8 text-center",
                    "🕷️ Web Crawler"
                }
                components::crawl::CrawlForm {}
                components::crawl::CrawlJobs {}
            }
            components::layout::Footer {}
        }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gradient-to-br from-stone-900 via-stone-800 to-stone-900",
            components::layout::Header {}
            main { class: "container mx-auto px-4 py-8",
                h1 { class: "text-3xl font-bold text-green-400 mb-8 text-center",
                    "⚙️ Settings"
                }
                components::settings::SettingsForm {}
            }
            components::layout::Footer {}
        }
    }
}
