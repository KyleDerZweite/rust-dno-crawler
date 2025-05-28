#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-neutral-900 border-t border-neutral-700 mt-auto",
            div {
                class: "max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8",

                // Main Footer Content
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12",

                    // Brand Section
                    div {
                        class: "col-span-1 sm:col-span-2 lg:col-span-1",
                        div {
                            class: "flex items-center space-x-3 mb-4",
                            div {
                                class: "w-8 h-8 bg-green-500 rounded-lg flex items-center justify-center",
                                span { class: "text-white font-bold text-sm", "K" }
                            }
                            h3 {
                                class: "text-xl font-bold text-neutral-100",
                                "DNO - Crawler"
                            }
                        }
                        p {
                            class: "text-neutral-400 text-sm leading-relaxed mb-4",
                            "Open Source Web Crawler for DNO Data.\nPart of the KyleHub ecosystem."
                        }
                        div {
                            class: "flex space-x-4",
                            a {
                                href: "https://github.com/KyleDerZweite/rust-dno-crawler",
                                class: "text-neutral-400 hover:text-green-500 transition-colors",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                svg {
                                    class: "w-5 h-5",
                                    fill: "currentColor",
                                    view_box: "0 0 24 24",
                                    path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                }
                            }
                        }
                    }

                    // Projects Section
                    div {
                        h4 {
                            class: "text-lg font-semibold text-neutral-100 mb-4",
                            "Projects"
                        }
                        ul {
                            class: "space-y-3",
                            li {
                                a {
                                    href: "https://kylehub.dev/projects/dno-crawler",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "DNO Crawler"
                                }
                            }
                            li {
                                a {
                                    href: "https://kylehub.dev/projects",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "All Projects"
                                }
                            }
                            li {
                                a {
                                    href: "https://kylehub.dev/tools",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "Development Tools"
                                }
                            }
                        }
                    }

                    // Quick Links Section
                    div {
                        h4 {
                            class: "text-lg font-semibold text-neutral-100 mb-4",
                            "Quick Links"
                        }
                        ul {
                            class: "space-y-3",
                            li {
                                a {
                                    href: "/dashboard",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "Dashboard"
                                }
                            }
                        }
                    }

                    // Legal Section
                    div {
                        h4 {
                            class: "text-lg font-semibold text-neutral-100 mb-4",
                            "Legal"
                        }
                        ul {
                            class: "space-y-3",
                            li {
                                a {
                                    href: "/privacy",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "Privacy Policy"
                                }
                            }
                            li {
                                a {
                                    href: "/terms",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "Terms of Service"
                                }
                            }
                            li {
                                a {
                                    href: "/impressum",
                                    class: "text-neutral-400 hover:text-green-500 text-sm transition-colors",
                                    "Impressum"
                                }
                            }
                        }
                    }
                }

                // Footer Bottom Section
                div {
                    class: "border-t border-neutral-700 mt-12 pt-8",
                    div {
                        class: "flex flex-col sm:flex-row justify-between items-center space-y-4 sm:space-y-0",
                        p {
                            class: "text-neutral-400 text-sm text-center sm:text-left",
                            "© 2025 Kyle (Leander Grau). Licensed under Apache-2.0."
                        }
                        div {
                            class: "flex items-center space-x-4 text-xs text-neutral-500",
                            span { "Built with " }
                            span {
                                class: "text-amber-600 font-medium",
                                "Rust"
                            }
                            span { "+" }
                            span {
                                class: "text-green-500 font-medium",
                                "Dioxus"
                            }
                            span { "+" }
                            span {
                                class: "text-neutral-400 font-medium",
                                "TailwindCSS"
                            }
                        }
                    }
                }
            }
        }
    }
}