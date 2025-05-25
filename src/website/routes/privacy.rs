#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Privacy() -> Element {
    let current_date = chrono::Utc::now().format("%B %d, %Y").to_string();

    rsx! {
        div { class: "max-w-4xl mx-auto",
            div { class: "bg-white shadow rounded-lg",
                div { class: "px-4 py-5 sm:p-6",
                    h1 { class: "text-3xl font-bold text-gray-900 mb-6",
                        "Privacy Policy - DNO Crawler"
                    }
                    div { class: "prose max-w-none",
                        p { class: "text-gray-600 mb-4",
                            "Last updated: {current_date}"
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Information We Collect"
                        }
                        p { class: "text-gray-700 mb-4",
                            "We collect information you provide directly to us, such as when you create an account, use our DNO crawler services, or contact us for support. This includes data retrieved from German Distribution Network Operators."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Data Usage and Processing"
                        }
                        p { class: "text-gray-700 mb-4",
                            "The DNO Crawler processes data from various German Distribution Network Operators in compliance with applicable data protection regulations. We use this information to provide automated data retrieval services."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "How We Use Your Information"
                        }
                        p { class: "text-gray-700 mb-4",
                            "We use the information we collect to provide, maintain, and improve our crawler services, process data requests, and communicate with you about DNO-related updates."
                        }
                        
                        h2 { class: "text-xl font-semibold text-gray-900 mt-6 mb-3",
                            "Contact Us"
                        }
                        p { class: "text-gray-700 mb-4",
                            "If you have any questions about this Privacy Policy or our DNO data processing, please contact us at privacy@dno-crawler.com"
                        }
                    }
                }
            }
        }
    }
}