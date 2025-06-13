#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container mx-auto py-12 px-4 sm:px-6 lg:px-8",
            div { class: "max-w-4xl mx-auto text-center",
                h1 { class: "text-4xl font-black text-light-beige-200 mb-6",
                    "Welcome to DNO Crawler"
                }
                p { class: "text-xl font-normal text-light-beige-400 mb-8",
                    "Your comprehensive tool for German Distribution Network Operator data analysis"
                }
                
                div { class: "grid md:grid-cols-3 gap-8 mt-12",
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-bold mb-4 text-forest-green-400",
                                "🌐 Web Crawling"
                            }
                            p { class: "text-light-beige-400",
                                "Save Time with Automated Data Collection. Effortlessly gather the latest data from all German DNO websites without manual work, so you can focus on analysis."
                            }
                        }
                    }
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-bold mb-4 text-forest-green-400",
                                "📊 Data Analysis"
                            }
                            p { class: "text-light-beige-400",
                                "Discover Insights Instantly. Use our powerful built-in tools and customizable filters to immediately process raw data into actionable intelligence."
                            }
                        }
                    }
                    Card {
                        variant: CardVariant::Elevated,
                        padding: CardPadding::Medium,
                        CardContent {
                            h3 { class: "text-xl font-bold mb-4 text-vibrant-orange-400",
                                "🤖 AI Integration"
                            }
                            p { class: "text-light-beige-400",
                                "Unlock Deeper Insights with AI. Leverage integrated AI, powered by Ollama, to intelligently parse unstructured content and reveal trends you would otherwise miss."
                            }
                        }
                    }
                }
                
                div { class: "mt-12",
                    div { class: "inline-flex space-x-4",
                        PrimaryButton {
                            href: "/dashboard".to_string(),
                            size: ButtonSize::Large,
                            "Start Crawling Now"
                        }
                        SecondaryButton {
                            href: "/contact".to_string(),
                            size: ButtonSize::Large,
                            "Explore Features"
                        }
                    }
                }
            }
        }
    }
}