#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            div { class: "max-w-4xl mx-auto text-center",
                h1 { class: "text-4xl font-bold text-light-beige-200 mb-6",
                    "Welcome to DNO Crawler"
                }
                p { class: "text-xl text-light-beige-400 mb-8",
                    "Your comprehensive tool for German Distribution Network Operator data analysis"
                }
                
                div { class: "grid md:grid-cols-3 gap-8 mt-12",
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-semibold mb-4 text-forest-green-400",
                                "🌐 Web Crawling"
                            }
                            p { class: "text-light-beige-400",
                                "Automated collection of data from German DNO websites with intelligent parsing and extraction."
                            }
                        }
                    }
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-semibold mb-4 text-forest-green-400",
                                "📊 Data Analysis"
                            }
                            p { class: "text-light-beige-400",
                                "Process and analyze collected data with built-in tools and customizable filters."
                            }
                        }
                    }
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-semibold mb-4 text-amber-brown-400",
                                "🤖 AI Integration"
                            }
                            p { class: "text-light-beige-400",
                                "Powered by Ollama for intelligent content processing and insights."
                            }
                        }
                    }
                }
                
                div { class: "mt-12",
                    div { class: "inline-flex space-x-4",
                        PrimaryButton {
                            href: "/dashboard".to_string(),
                            size: ButtonSize::Large,
                            "Get Started"
                        }
                        SecondaryButton {
                            href: "/contact".to_string(),
                            size: ButtonSize::Large,
                            "Learn More"
                        }
                    }
                }
            }
        }
    }
}