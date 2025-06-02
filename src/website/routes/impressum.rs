#![allow(non_snake_case)]
use dioxus::prelude::*;
use chrono::{Local, Datelike, Timelike};

#[component]
pub fn Impressum() -> Element {
    // Get current local time
    let now = Local::now();
    let weekday = match now.weekday() {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    };
    let month = match now.month() {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown"
    };
    let current_time = format!("{}, {} {}, {} • {:02}:{:02}:{:02}",
                               weekday, month, now.day(), now.year(), now.hour(), now.minute(), now.second());

    // Theme-aware classes (simplified to dark theme)
    let container_bg = "bg-neutral-900/60 backdrop-blur-xl border border-stone-700/40 shadow-2xl shadow-black/20";
    let card_bg = "bg-neutral-800/80 backdrop-blur-lg border border-stone-700/30 shadow-xl shadow-black/10";
    let text_primary = "text-stone-100";
    let text_secondary = "text-stone-400";
    let accent_bg = "bg-neutral-700/50";
    let link_color = "text-moss-400 hover:text-moss-300";
    let divider_color = "border-stone-700/60";

    rsx! {
        div {
            class: "w-full min-h-screen py-8 px-4 sm:px-6 lg:px-8",
            style: "padding-top: 6rem;",

            // Main container
            div {
                class: "{container_bg} rounded-3xl p-6 lg:p-8 max-w-4xl mx-auto",

                // Header Section
                div {
                    class: "text-center mb-8",
                    h1 {
                        class: "text-3xl lg:text-4xl font-bold {text_primary} mb-2",
                        "Impressum"
                    }
                    p {
                        class: "{text_secondary} text-lg mb-1",
                        "{current_time}"
                    }
                    p {
                        class: "{text_secondary} text-sm",
                        "Legal information according to § 5 TMG"
                    }
                }

                // Main Content
                div {
                    class: "space-y-8",

                    // Company Information
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center mb-6",
                            div {
                                class: "w-12 h-12 bg-moss-500/20 {text_primary} rounded-xl flex items-center justify-center mr-4",
                                svg {
                                    class: "w-6 h-6 text-moss-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"
                                    }
                                }
                            }
                            h2 {
                                class: "text-xl font-bold {text_primary} ml-2",
                                " Service Provider"
                            }
                        }
                        div {
                            class: "{accent_bg} rounded-xl p-4 space-y-3",
                            div {
                                h3 {
                                    class: "text-sm font-semibold {text_secondary} uppercase tracking-wide mb-2",
                                    "Company Name"
                                }
                                p {
                                    class: "text-lg font-semibold {text_primary}",
                                    "KyleHub"
                                }
                            }
                            div {
                                h3 {
                                    class: "text-sm font-semibold {text_secondary} uppercase tracking-wide mb-2",
                                    "Represented by"
                                }
                                p {
                                    class: "{text_primary}",
                                    "Leander Grau"
                                }
                            }
                        }
                    }

                    // Contact Information
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center mb-6",
                            div {
                                class: "w-12 h-12 bg-blue-500/20 rounded-xl flex items-center justify-center mr-4",
                                svg {
                                    class: "w-6 h-6 text-blue-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                    }
                                }
                            }
                            h2 {
                                class: "text-xl font-bold {text_primary} ml-2",
                                "Contact Information"
                            }
                        }
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                class: "{accent_bg} rounded-xl p-4",
                                h3 {
                                    class: "text-sm font-semibold {text_secondary} uppercase tracking-wide mb-2",
                                    "Address"
                                }
                                div {
                                    class: "{text_primary} space-y-1",
                                    p { "Musterstraße 123" }
                                    p { "12345 Musterstadt" }
                                    p { "Germany" }
                                }
                            }
                            div {
                                class: "{accent_bg} rounded-xl p-4",
                                h3 {
                                    class: "text-sm font-semibold {text_secondary} uppercase tracking-wide mb-2",
                                    "Contact Details"
                                }
                                div {
                                    class: "{text_primary} space-y-2",
                                    div {
                                        class: "flex items-center space-x-2",
                                        svg {
                                            class: "w-4 h-4 {text_secondary}",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M3 8l7.89 4.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                            }
                                        }
                                        a {
                                            href: "mailto:info@kylehub.dev",
                                            class: "{link_color} transition-colors duration-200",
                                            "info@kylehub.dev"
                                        }
                                    }
                                    div {
                                        class: "flex items-center space-x-2",
                                        svg {
                                            class: "w-4 h-4 {text_secondary}",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"
                                            }
                                        }
                                        span { "+49 (0) 123 456789" }
                                    }
                                }
                            }
                        }
                    }

                    // Legal Information
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center mb-6",
                            div {
                                class: "w-12 h-12 bg-amber-500/20 rounded-xl flex items-center justify-center mr-4",
                                svg {
                                    class: "w-6 h-6 text-amber-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                    }
                                }
                            }
                            h2 {
                                class: "text-xl font-bold {text_primary} ml-2",
                                "Legal Details [WIP]"
                            }
                        }
                        div {
                            class: "space-y-4",
                            div {
                                class: "{accent_bg} rounded-xl p-4",
                                h3 {
                                    class: "text-sm font-semibold {text_secondary} uppercase tracking-wide mb-3",
                                    "Registration & Tax Information"
                                }
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 {text_primary}",
                                    div {
                                        p { class: "font-medium mb-1", "VAT ID:" }
                                        p { "DE123456789" }
                                    }
                                    div {
                                        p { class: "font-medium mb-1", "Trade Register:" }
                                        p { "HRB 12345 Berlin" }
                                    }
                                }
                            }
                        }
                    }

                    // Responsibility and Liability
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center mb-6",
                            div {
                                class: "w-12 h-12 bg-red-500/20 rounded-xl flex items-center justify-center mr-4",
                                svg {
                                    class: "w-6 h-6 text-red-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                                    }
                                }
                            }
                            h2 {
                                class: "text-xl font-bold {text_primary} ml-2",
                                "Liability & Content Responsibility"
                            }
                        }
                        div {
                            class: "{accent_bg} rounded-xl p-4 space-y-4 {text_primary}",
                            div {
                                h3 {
                                    class: "font-semibold mb-2",
                                    "Content Responsibility"
                                }
                                p {
                                    class: "text-sm leading-relaxed",
                                    "The contents of our pages have been created with the utmost care. However, we cannot guarantee the contents' accuracy, completeness, or topicality. According to statutory provisions, we are furthermore responsible for our own content on these web pages."
                                }
                            }
                            div {
                                class: "border-t {divider_color} pt-4",
                                h3 {
                                    class: "font-semibold mb-2",
                                    "Liability for Links"
                                }
                                p {
                                    class: "text-sm leading-relaxed",
                                    "Our offer includes links to external third-party websites. We have no influence on the contents of those websites, therefore we cannot guarantee for those contents. Providers or administrators of linked websites are always responsible for the contents of the linked websites."
                                }
                            }
                        }
                    }

                    // Data Protection
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center mb-6",
                            div {
                                class: "w-12 h-12 bg-green-500/20 rounded-xl flex items-center justify-center mr-4",
                                svg {
                                    class: "w-6 h-6 text-green-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                    }
                                }
                            }
                            h2 {
                                class: "text-xl font-bold {text_primary} ml-2",
                                "Data Protection"
                            }
                        }
                        div {
                            class: "{accent_bg} rounded-xl p-4",
                            div {
                                class: "{text_primary} space-y-3",
                                p {
                                    class: "text-sm leading-relaxed",
                                    "The use of our website is generally possible without providing personal data. If personal data (such as name, address, or email addresses) is collected on our pages, this is always done on a voluntary basis, as far as possible."
                                }
                                p {
                                    class: "text-sm leading-relaxed",
                                    "We would like to point out that data transmission over the Internet (e.g., communication by email) may be subject to security vulnerabilities. Complete protection of data from access by third parties is not possible."
                                }
                                div {
                                    class: "pt-2",
                                    a {
                                        href: "/privacy",
                                        class: "{link_color} text-sm font-medium transition-colors duration-200 inline-flex items-center space-x-1",
                                        span { "Read our Privacy Policy" }
                                        svg {
                                            class: "w-4 h-4",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Footer Information
                    div {
                        class: "text-center pt-8 border-t {divider_color}",
                        div {
                            class: "{accent_bg} rounded-xl p-4",
                            p {
                                class: "text-sm {text_secondary} mb-2",
                                "Last updated: May 29, 2025"
                            }
                            p {
                                class: "text-xs {text_secondary}",
                                "This impressum is generated in compliance with German law (TMG § 5, RStV § 55)"
                            }
                        }
                    }

                    // Back to Dashboard
                    div {
                        class: "text-center pt-6",
                        a {
                            href: "/dashboard",
                            class: "inline-flex items-center space-x-2 px-6 py-3 bg-moss-500 hover:bg-moss-600 {text_primary} rounded-xl transition-all duration-200 hover:scale-105 shadow-lg hover:shadow-moss-500/30",
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M10 19l-7-7m0 0l7-7m-7 7h18"
                                }
                            }
                            span { "Back to Dashboard" }
                        }
                    }
                }
            }
        }
    }
}
