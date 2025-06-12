#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Error404Props {
    #[props(default = Vec::new())]
    pub route: Vec<String>,
}

#[component]
pub fn Error404(props: Error404Props) -> Element {
    // Theme-aware classes (simplified to dark theme)
    let container_bg = "bg-neutral-800 shadow-xl border border-neutral-700";
    let heading_color = "text-neutral-100";
    let text_color = "text-neutral-400";
    let error_number_color = "text-amber-600";
    let log_bg = "bg-red-900/20 border border-red-800/30";
    let log_text = "text-amber-400";
    let icon_color = "text-neutral-500";
    let dashboard_button = "border-neutral-600 text-neutral-300 hover:text-green-500 hover:bg-green-900/20 hover:border-green-500 bg-transparent";

    rsx! {
        div {
            class: "flex-grow flex items-center justify-center px-4 py-8",
            div {
                class: format!("text-center {} rounded-2xl p-8 max-w-lg mx-auto", container_bg),

                // 404 Number with animation
                div {
                    class: format!("text-8xl md:text-9xl font-bold {} mb-6", error_number_color),
                    "404"
                }

                // Error Icon
                div {
                    class: "mb-6",
                    svg {
                        class: format!("w-16 h-16 {} mx-auto", icon_color),
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "1.5",
                            d: "M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        }
                    }
                }

                // Main heading
                h1 {
                    class: format!("text-3xl md:text-4xl font-bold {} mb-4", heading_color),
                    "Page Not Found"
                }

                // Description
                p {
                    class: format!("{} mb-8 text-lg leading-relaxed", text_color),
                    "Oops! The page you're looking for seems to have wandered off into the digital wilderness. Don't worry, it happens to the best of us!"
                }

                // Debug info (only show if route is not empty)
                if !props.route.is_empty() {
                    div {
                        class: "mb-8",
                        details {
                            class: "text-left",
                            summary {
                                class: format!("cursor-pointer {} text-sm font-medium mb-2", text_color),
                                "🔍 Debug Information"
                            }
                            pre {
                                class: format!("text-xs {} {} p-3 rounded-lg overflow-x-auto", log_text, log_bg),
                                "Attempted to navigate to:\n{props.route.join(\"/\")}"
                            }
                        }
                    }
                }

                // Action buttons
                div {
                    class: "flex flex-col sm:flex-row gap-4 justify-center items-center",
                    // Dashboard button (Secondary - Green hover for user navigation)
                    a {
                        href: "/dashboard",
                        class: format!("inline-flex items-center px-6 py-3 {} font-medium rounded-xl transition-all duration-200 border-2", dashboard_button),
                        svg {
                            class: "w-5 h-5 mr-2",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z"
                            }
                        }
                        "Dashboard"
                    }
                }

                // Additional help text
                div {
                    class: "mt-8 pt-6 border-t border-gray-200 dark:border-neutral-700",
                    p {
                        class: format!("{} text-sm", text_color),
                        "Still lost? Try using the navigation menu above or "
                        a {
                            href: "https://github.com/KyleDerZweite/rust-dno-crawler/issues",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-amber-600 hover:text-amber-700 underline font-medium transition-colors underline-offset-4 hover:underline",
                            "report this issue"
                        }
                        " if you think this is a bug."
                    }
                }
            }
        }
    }
}
