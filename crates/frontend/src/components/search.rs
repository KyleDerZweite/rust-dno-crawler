use dioxus::prelude::*;

#[component]
pub fn SearchForm() -> Element {
    let mut query = use_signal(|| String::new());
    
    rsx! {
        div { class: "max-w-2xl mx-auto mb-8",
            div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-6 border border-stone-700",
                div { class: "space-y-4",
                    input {
                        r#type: "text",
                        placeholder: "Search for DNO data...",
                        class: "w-full px-4 py-3 bg-stone-700 border border-stone-600 rounded-lg text-white placeholder-stone-400 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500",
                        value: "{query}",
                        oninput: move |evt| query.set(evt.value())
                    }
                    button {
                        class: "w-full px-6 py-3 bg-green-600 hover:bg-green-500 text-white rounded-lg transition-colors font-medium",
                        onclick: move |_| {
                            // TODO: Implement search functionality
                            tracing::info!("Searching for: {}", query());
                        },
                        "🔍 Search"
                    }
                }
            }
        }
    }
}

#[component]
pub fn SearchResults() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto",
            div { class: "text-center text-stone-400 py-12",
                p { "Enter a search query to find DNO data" }
            }
        }
    }
}