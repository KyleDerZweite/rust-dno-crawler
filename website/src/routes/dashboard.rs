#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold text-white mb-6", "Dashboard" }
            p { class: "text-gray-300 mb-8", "Welcome to your DNO Crawler dashboard." }
            
            div { class: "grid md:grid-cols-2 lg:grid-cols-3 gap-6",
                div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                    h2 { class: "text-xl font-bold text-emerald-400 mb-4", "Crawler Status" }
                    p { class: "text-gray-300", "All systems operational" }
                    div { class: "mt-4",
                        button { 
                            class: "bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-medium py-2 px-6 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                            "Start Crawl" 
                        }
                    }
                }
                div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                    h2 { class: "text-xl font-bold text-amber-400 mb-4", "Recent Data" }
                    p { class: "text-gray-300", "Last updated 2 hours ago" }
                    div { class: "mt-4",
                        button { 
                            class: "bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-600 hover:to-amber-700 text-white font-medium py-2 px-6 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                            "View Data" 
                        }
                    }
                }
                div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                    h2 { class: "text-xl font-bold text-blue-400 mb-4", "Settings" }
                    p { class: "text-gray-300", "Configure your crawler" }
                    div { class: "mt-4",
                        button { 
                            class: "bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-medium py-2 px-6 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                            "Settings" 
                        }
                    }
                }
            }
        }
    }
}