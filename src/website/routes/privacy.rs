#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::theme::{use_theme, Theme};

#[component]
pub fn Privacy() -> Element {
    let theme = use_theme();
    let current_date = "May 28, 2025"; // Using your current date

    // Theme-aware classes
    let container_bg = match theme() {
        Theme::Light => "bg-white shadow-lg border border-gray-200",
        Theme::Dark => "bg-neutral-800 shadow-xl border border-neutral-700",
    };

    let heading_color = match theme() {
        Theme::Light => "text-neutral-800",
        Theme::Dark => "text-neutral-100",
    };

    let subheading_color = match theme() {
        Theme::Light => "text-neutral-700",
        Theme::Dark => "text-neutral-200",
    };

    let text_color = match theme() {
        Theme::Light => "text-neutral-600",
        Theme::Dark => "text-neutral-400",
    };

    let date_color = match theme() {
        Theme::Light => "text-neutral-500",
        Theme::Dark => "text-neutral-500",
    };

    rsx! {
        div {
            class: "flex-grow px-4 py-8",
            div {
                class: "max-w-4xl mx-auto",
                div {
                    class: format!("{} rounded-2xl", container_bg),
                    div {
                        class: "px-6 py-8 sm:p-10",
                        h1 {
                            class: format!("text-4xl font-bold {} mb-6", heading_color),
                            "Privacy Policy - DNO Crawler"
                        }
                        div {
                            class: "prose max-w-none",
                            p {
                                class: format!("{} mb-6 text-sm", date_color),
                                "Last updated: {current_date}"
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Information We Collect"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "We collect information you provide directly to us, such as when you create an account, use our DNO crawler services, or contact us for support. This includes data retrieved from German Distribution Network Operators."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Data Usage and Processing"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "The DNO Crawler processes data from various German Distribution Network Operators in compliance with applicable data protection regulations. We use this information to provide automated data retrieval services."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "How We Use Your Information"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "We use the information we collect to provide, maintain, and improve our crawler services, process data requests, and communicate with you about DNO-related updates."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Contact Us"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "If you have any questions about this Privacy Policy or our DNO data processing, please contact us at "
                                a {
                                    href: "mailto:privacy@kylehub.dev",
                                    class: "text-green-500 hover:text-green-600 transition-colors underline",
                                    "privacy@kylehub.dev"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}