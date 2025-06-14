#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto py-12 px-4",
            div { class: "max-w-4xl mx-auto text-center",
                h1 { class: "text-4xl font-bold text-white mb-6",
                    "Welcome to DNO Crawler"
                }
                p { class: "text-xl text-gray-300 mb-8",
                    "Your comprehensive tool for German Distribution Network Operator data analysis"
                }
                
                div { class: "grid md:grid-cols-3 gap-8 mt-12",
                    div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                        h3 { class: "text-xl font-bold mb-4 text-emerald-400",
                            "🌐 Web Crawling"
                        }
                        p { class: "text-gray-300",
                            "Save Time with Automated Data Collection. Effortlessly gather the latest data from all German DNO websites without manual work."
                        }
                    }
                    div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                        h3 { class: "text-xl font-bold mb-4 text-emerald-400",
                            "📊 Data Analysis"
                        }
                        p { class: "text-gray-300",
                            "Discover Insights Instantly. Use our powerful built-in tools and customizable filters to immediately process raw data."
                        }
                    }
                    div { class: "bg-gray-900/50 backdrop-blur-sm border border-white/10 p-6 rounded-xl shadow-xl hover:shadow-2xl transition-all duration-300 hover:scale-[1.02]",
                        h3 { class: "text-xl font-bold mb-4 text-amber-400",
                            "🤖 AI Integration"
                        }
                        p { class: "text-gray-300",
                            "Unlock Deeper Insights with AI. Leverage integrated AI, powered by Ollama, to intelligently parse unstructured content."
                        }
                    }
                }
                
                div { class: "mt-12",
                    div { class: "inline-flex space-x-4",
                        a {
                            href: "/dashboard",
                            class: "bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-semibold py-3 px-8 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                            "Start Crawling Now"
                        }
                        a {
                            href: "/contact",
                            class: "bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-600 hover:to-amber-700 text-white font-semibold py-3 px-8 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                            "Explore Features"
                        }
                    }
                }
            }
        }
    }
}