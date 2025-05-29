#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::theme::{use_theme, Theme};

#[component]
pub fn Terms() -> Element {
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
            style: "padding-top: 6rem;",

            div {
                class: "max-w-4xl mx-auto",
                div {
                    class: format!("{} rounded-2xl", container_bg),
                    div {
                        class: "px-6 py-8 sm:p-10",
                        h1 {
                            class: format!("text-4xl font-bold {} mb-6", heading_color),
                            "Terms of Service - DNO Crawler"
                        }
                        div {
                            class: "prose max-w-none",
                            p {
                                class: format!("{} mb-6 text-sm", date_color),
                                "Last updated: {current_date}"
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Acceptance of Terms"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "By accessing and using the DNO Crawler service, you accept and agree to be bound by the terms and provision of this agreement."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Service Description"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "DNO Crawler provides automated data retrieval services from German Distribution Network Operators. The service is designed for legitimate data collection purposes only."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Use License"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "Permission is granted to use the DNO Crawler service for lawful data collection activities. Users must comply with all applicable laws and DNO terms of service."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Disclaimer"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "The DNO Crawler service is provided on an 'as is' basis. We make no warranties regarding the accuracy, completeness, or availability of retrieved DNO data."
                            }

                            h2 {
                                class: format!("text-2xl font-semibold {} mt-8 mb-4", subheading_color),
                                "Limitation of Liability"
                            }
                            p {
                                class: format!("{} mb-6 leading-relaxed", text_color),
                                "Users are responsible for ensuring their use of retrieved DNO data complies with all applicable regulations and the terms of service of the respective Distribution Network Operators."
                            }
                        }
                    }
                }
            }
        }
    }
}