use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "bg-stone-800/50 backdrop-blur-sm border-b border-stone-700",
            nav { class: "container mx-auto px-4 py-4",
                div { class: "flex items-center justify-between",
                    Link { to: Route::Home {}, class: "flex items-center space-x-2 text-green-400 hover:text-green-300 transition-colors",
                        span { class: "text-2xl", "🌿" }
                        span { class: "text-xl font-bold", "DNO Gatherer" }
                    }
                    
                    div { class: "flex items-center space-x-6",
                        NavLink { to: Route::Search {}, "Search" }
                        NavLink { to: Route::Dnos {}, "DNOs" }
                        NavLink { to: Route::Crawl {}, "Crawler" }
                        NavLink { to: Route::Settings {}, "Settings" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-stone-800/30 border-t border-stone-700 mt-auto",
            div { class: "container mx-auto px-4 py-8",
                div { class: "text-center text-stone-400",
                    p { class: "mb-2",
                        "Built with ❤️ for KyleDerZweite and German friends"
                    }
                    p { class: "text-sm",
                        "Powered by Rust 🦀 • Dioxus • Axum • SearXNG • Ollama"
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(to: Route, children: Element) -> Element {
    rsx! {
        Link {
            to: to,
            class: "text-stone-300 hover:text-green-400 transition-colors px-3 py-2 rounded-md hover:bg-stone-700/50",
            {children}
        }
    }
}