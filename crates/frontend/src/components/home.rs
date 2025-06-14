use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[component]
pub fn FeatureCard(icon: String, title: String, description: String, link: String) -> Element {
    rsx! {
        div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-6 border border-stone-700 hover:border-green-500/50 transition-all duration-300 hover:transform hover:scale-105",
            div { class: "text-center",
                div { class: "text-4xl mb-4", "{icon}" }
                h3 { class: "text-xl font-semibold text-green-400 mb-3", "{title}" }
                p { class: "text-stone-300 mb-6 leading-relaxed", "{description}" }
                Link {
                    to: match link.as_str() {
                        "/search" => Route::Search {},
                        "/dnos" => Route::Dnos {},
                        "/crawl" => Route::Crawl {},
                        "/settings" => Route::Settings {},
                        _ => Route::Home {},
                    },
                    class: "inline-flex items-center px-4 py-2 bg-green-600 hover:bg-green-500 text-white rounded-lg transition-colors font-medium",
                    "Learn More →"
                }
            }
        }
    }
}