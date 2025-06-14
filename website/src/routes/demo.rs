#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold text-white mb-6", "Component Demo" }
            p { class: "text-gray-300 mb-8", "Simplified demo page without complex components." }
            
            div { class: "bg-gray-900/80 backdrop-blur-lg border border-white/10 p-8 rounded-xl shadow-2xl",
                h2 { class: "text-2xl font-bold text-emerald-400 mb-4", "Simple Card" }
                p { class: "text-gray-300", "This is a basic card component using simple CSS classes." }
            }
        }
    }
}