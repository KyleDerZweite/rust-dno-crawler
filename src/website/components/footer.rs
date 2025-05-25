#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-gray-800 text-white",
            div { class: "max-w-7xl mx-auto py-8 px-4 sm:px-6 lg:px-8",
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    div {
                        h3 { class: "text-lg font-semibold mb-4", "Auth System" }
                        p { class: "text-gray-300 text-sm",
                            "A Crawler for automated retrieving of data from various German-DNO's"
                        }
                    }
                    div {
                        h4 { class: "text-md font-semibold mb-4", "Quick Links" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "/dashboard",
                                    class: "text-gray-300 hover:text-white text-sm",
                                    "Dashboard"
                                }
                            }
                            li {
                                a {
                                    href: "/contact",
                                    class: "text-gray-300 hover:text-white text-sm",
                                    "Contact"
                                }
                            }
                        }
                    }
                    div {
                        h4 { class: "text-md font-semibold mb-4", "Legal" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "/privacy",
                                    class: "text-gray-300 hover:text-white text-sm",
                                    "Privacy Policy"
                                }
                            }
                            li {
                                a {
                                    href: "/terms",
                                    class: "text-gray-300 hover:text-white text-sm",
                                    "Terms of Service"
                                }
                            }
                        }
                    }
                }
                div { class: "border-t border-gray-700 mt-8 pt-6 text-center",
                    p { class: "text-gray-400 text-sm",
                        "© 2025 Kyle. All rights reserved."
                    }
                }
            }
        }
    }
}