#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Terms() -> Element {
    let current_date = chrono::Utc::now().format("%B %d, %Y").to_string();

    rsx! {
        div { class: "max-w-4xl mx-auto",
            div { class: "bg-white shadow rounded-lg",
                div { class: "px-4 py-5 sm:p-6",
                    h1 { class: "text-3xl font-bold text-gray-900 mb-6",
                        "Terms of Service - DNO Crawler"
                    }
                    div { class: "prose max-w-none",
                        p { class: "text-gray-600 mb-4",
                            "Last updated: {current_date}"
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Acceptance of Terms"
                        }
                        p { class: "text-gray-700 mb-4",
                            "By accessing and using the DNO Crawler service, you accept and agree to be bound by the terms and provision of this agreement."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Service Description"
                        }
                        p { class: "text-gray-700 mb-4",
                            "DNO Crawler provides automated data retrieval services from German Distribution Network Operators. The service is designed for legitimate data collection purposes only."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Use License"
                        }
                        p { class: "text-gray-700 mb-4",
                            "Permission is granted to use the DNO Crawler service for lawful data collection activities. Users must comply with all applicable laws and DNO terms of service."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Disclaimer"
                        }
                        p { class: "text-gray-700 mb-4",
                            "The DNO Crawler service is provided on an 'as is' basis. We make no warranties regarding the accuracy, completeness, or availability of retrieved DNO data."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Limitation of Liability"
                        }
                        p { class: "text-gray-700 mb-4",
                            "Users are responsible for ensuring their use of retrieved DNO data complies with all applicable regulations and the terms of service of the respective Distribution Network Operators."
                        }
                    }
                }
            }
        }
    }
}