#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Error404Props {
    #[props(default = Vec::new())]
    pub route: Vec<String>,
}

#[component]
pub fn Error404(props: Error404Props) -> Element {
    // Theme-aware classes using custom color palette
    let container_bg = "bg-dark-charcoal-600/80 backdrop-blur-lg border border-dark-charcoal-400/30 shadow-xl shadow-dark-charcoal-900/10";
    let heading_color = "text-light-beige-200";
    let text_color = "text-light-beige-500";
    let error_number_color = "text-vibrant-orange-400";
    let log_bg = "bg-vibrant-orange-500/20 border border-vibrant-orange-500/30";
    let log_text = "text-light-beige-300";
    let icon_color = "text-light-beige-600";
    let dashboard_button = "border-forest-green-500 text-forest-green-400 hover:text-light-beige-200 hover:bg-forest-green-500 hover:border-forest-green-600 bg-transparent";

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
                    class: "mt-8 pt-6 border-t border-dark-charcoal-400",
                    p {
                        class: format!("{} text-sm", text_color),
                        "Still lost? Try using the navigation menu above or "
                        a {
                            href: "https://github.com/KyleDerZweite/rust-dno-crawler/issues",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-vibrant-orange-400 hover:text-vibrant-orange-300 underline font-medium transition-colors underline-offset-4 hover:underline",
                            "report this issue"
                        }
                        " if you think this is a bug."
                    }
                }
            }
        }
    }
}
