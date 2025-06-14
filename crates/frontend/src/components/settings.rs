use dioxus::prelude::*;

#[component]
pub fn SettingsForm() -> Element {
    let mut api_url = use_signal(|| "http://localhost:8080".to_string());
    let mut searxng_url = use_signal(|| "http://localhost:8888".to_string());
    let mut ollama_url = use_signal(|| "http://localhost:11434".to_string());
    
    rsx! {
        div { class: "max-w-2xl mx-auto",
            div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-6 border border-stone-700",
                h2 { class: "text-xl font-semibold text-green-400 mb-6", "Configuration" }
                
                div { class: "space-y-6",
                    div {
                        label { class: "block text-stone-300 text-sm font-medium mb-2",
                            "API Server URL"
                        }
                        input {
                            r#type: "url",
                            class: "w-full px-4 py-3 bg-stone-700 border border-stone-600 rounded-lg text-white placeholder-stone-400 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500",
                            value: "{api_url}",
                            oninput: move |evt| api_url.set(evt.value())
                        }
                    }
                    
                    div {
                        label { class: "block text-stone-300 text-sm font-medium mb-2",
                            "SearXNG URL"
                        }
                        input {
                            r#type: "url",
                            class: "w-full px-4 py-3 bg-stone-700 border border-stone-600 rounded-lg text-white placeholder-stone-400 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500",
                            value: "{searxng_url}",
                            oninput: move |evt| searxng_url.set(evt.value())
                        }
                    }
                    
                    div {
                        label { class: "block text-stone-300 text-sm font-medium mb-2",
                            "Ollama URL"
                        }
                        input {
                            r#type: "url",
                            class: "w-full px-4 py-3 bg-stone-700 border border-stone-600 rounded-lg text-white placeholder-stone-400 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500",
                            value: "{ollama_url}",
                            oninput: move |evt| ollama_url.set(evt.value())
                        }
                    }
                    
                    button {
                        class: "w-full px-6 py-3 bg-green-600 hover:bg-green-500 text-white rounded-lg transition-colors font-medium",
                        onclick: move |_| {
                            // TODO: Save settings
                            tracing::info!("Saving settings");
                        },
                        "Save Settings"
                    }
                }
            }
        }
    }
}