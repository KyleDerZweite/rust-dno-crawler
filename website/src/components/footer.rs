#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-gray-900/80 backdrop-blur-lg border-t border-white/10 text-white py-8 mt-auto",
            div {
                class: "max-w-6xl mx-auto px-4 text-center",
                
                div {
                    class: "flex justify-center items-center mb-4",
                    div {
                        class: "w-8 h-8 bg-gradient-to-br from-emerald-500 to-emerald-600 rounded-xl flex items-center justify-center mr-3 shadow-lg",
                        span { class: "text-white font-bold text-sm", "K" }
                    }
                    h3 { class: "text-xl font-bold", "DNO Crawler" }
                }
                
                p { class: "text-gray-400 mb-4", "Open Source Web Crawler - Part of KyleHub" }
                
                div {
                    class: "flex justify-center space-x-6 mb-6 text-sm",
                    a { href: "/dashboard", class: "text-gray-300 hover:text-emerald-400", "Dashboard" }
                    a { href: "/privacy", class: "text-gray-300 hover:text-emerald-400", "Privacy" }
                    a { href: "/terms", class: "text-gray-300 hover:text-emerald-400", "Terms" }
                    a { href: "/contact", class: "text-gray-300 hover:text-emerald-400", "Contact" }
                }
                
                p { class: "text-gray-500 text-xs", "© 2025 Kyle. Built with Rust + Dioxus + Freyr" }
            }
        }
    }
}
