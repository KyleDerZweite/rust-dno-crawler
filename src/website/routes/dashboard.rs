#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    #[props(default = String::new())]
    pub name: String,
    #[props(default = String::new())]
    pub email: String,
    #[props(default = String::new())]
    pub role: String,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-neutral-900 py-8",
            div {
                class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",

                // Welcome Header
                div {
                    class: "mb-8",
                    h1 {
                        class: "text-5xl font-bold text-neutral-100 mb-4",
                        "Dashboard"
                    }
                    p {
                        class: "text-neutral-400 text-lg",
                        "Welcome to your Crawler control center"
                    }
                }

                // Main Content Grid
                div {
                    class: "grid grid-cols-1 lg:grid-cols-3 gap-8",

                    // User Info Card
                    div {
                        class: "lg:col-span-1",
                        div {
                            class: "card",
                            div {
                                class: "flex items-center space-x-4 mb-6",
                                div {
                                    class: "w-16 h-16 bg-green-500 rounded-xl flex items-center justify-center",
                                    span {
                                        class: "text-white font-bold text-2xl",
                                        if !props.email.is_empty() {
                                            "{props.email.chars().next().unwrap_or('U').to_uppercase()}"
                                        } else {
                                            "U"
                                        }
                                    }
                                }
                                div {
                                    h3 {
                                        class: "text-xl font-semibold text-neutral-100",
                                        "{props.name}"
                                    }
                                    p {
                                        class: "text-neutral-400 text-sm",
                                        "Account information"
                                    }
                                }
                            }

                            dl {
                                class: "space-y-4",
                                div {
                                    dt {
                                        class: "text-sm font-medium text-neutral-400 mb-1",
                                        "Email Address"
                                    }
                                    dd {
                                        class: "text-neutral-100 font-medium",
                                        if !props.email.is_empty() {
                                            "{props.email}"
                                        } else {
                                            "Not available"
                                        }
                                    }
                                }
                                div {
                                    dt {
                                        class: "text-sm font-medium text-neutral-400 mb-1",
                                        "Role"
                                    }
                                    dd {
                                        span {
                                            class: if props.role == "admin" {
                                                "inline-flex items-center px-3 py-1 rounded-xl text-xs font-medium bg-amber-600/20 text-amber-600 border border-amber-600/30"
                                            } else {
                                                "inline-flex items-center px-3 py-1 rounded-xl text-xs font-medium bg-green-500/20 text-green-500 border border-green-500/30"
                                            },
                                            if !props.role.is_empty() {
                                                "{props.role}"
                                            } else {
                                                "user"
                                            }
                                        }
                                    }
                                }
                                div {
                                    dt {
                                        class: "text-sm font-medium text-neutral-400 mb-1",
                                        "Status"
                                    }
                                    dd {
                                        span {
                                            class: "inline-flex items-center px-3 py-1 rounded-xl text-xs font-medium bg-green-500/20 text-green-500 border border-green-500/30",
                                            "• Active"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Main Dashboard Content
                    div {
                        class: "lg:col-span-2 space-y-6",

                        // Quick Stats
                        div {
                            class: "grid grid-cols-1 sm:grid-cols-2 gap-6",

                            // DNO Data Collection Card
                            div {
                                class: "card card-hover",
                                div {
                                    class: "flex items-center justify-between mb-4",
                                    h4 {
                                        class: "text-lg font-semibold text-neutral-100",
                                        "DNO Data Collection"
                                    }
                                    div {
                                        class: "w-3 h-3 bg-green-500 rounded-full animate-pulse"
                                    }
                                }
                                p {
                                    class: "text-neutral-400 text-sm mb-4",
                                    "Automated data retrieval from German Distribution Network Operators"
                                }
                                div {
                                    class: "flex items-center justify-between",
                                    span {
                                        class: "text-2xl font-bold text-green-500",
                                        "24/7"
                                    }
                                    span {
                                        class: "text-xs text-neutral-400",
                                        "Active monitoring"
                                    }
                                }
                            }

                            // System Status Card
                            div {
                                class: "card card-hover",
                                div {
                                    class: "flex items-center justify-between mb-4",
                                    h4 {
                                        class: "text-lg font-semibold text-neutral-100",
                                        "System Status"
                                    }
                                    div {
                                        class: "w-3 h-3 bg-green-500 rounded-full"
                                    }
                                }
                                p {
                                    class: "text-neutral-400 text-sm mb-4",
                                    "All systems operational and running smoothly"
                                }
                                div {
                                    class: "flex items-center justify-between",
                                    span {
                                        class: "text-2xl font-bold text-green-500",
                                        "99.9%"
                                    }
                                    span {
                                        class: "text-xs text-neutral-400",
                                        "Uptime"
                                    }
                                }
                            }
                        }

                        // Recent Activity
                        div {
                            class: "card",
                            h4 {
                                class: "text-lg font-semibold text-neutral-100 mb-6",
                                "Recent Activity"
                            }
                            div {
                                class: "space-y-4",
                                div {
                                    class: "flex items-center space-x-3 p-3 bg-neutral-700/50 rounded-lg",
                                    div {
                                        class: "w-2 h-2 bg-green-500 rounded-full"
                                    }
                                    div {
                                        class: "flex-1",
                                        p {
                                            class: "text-neutral-100 text-sm font-medium",
                                            "Data crawl completed successfully"
                                        }
                                        p {
                                            class: "text-neutral-400 text-xs",
                                            "2 minutes ago"
                                        }
                                    }
                                }
                                div {
                                    class: "flex items-center space-x-3 p-3 bg-neutral-700/50 rounded-lg",
                                    div {
                                        class: "w-2 h-2 bg-amber-600 rounded-full"
                                    }
                                    div {
                                        class: "flex-1",
                                        p {
                                            class: "text-neutral-100 text-sm font-medium",
                                            "System maintenance scheduled"
                                        }
                                        p {
                                            class: "text-neutral-400 text-xs",
                                            "1 hour ago"
                                        }
                                    }
                                }
                                div {
                                    class: "flex items-center space-x-3 p-3 bg-neutral-700/50 rounded-lg",
                                    div {
                                        class: "w-2 h-2 bg-green-500 rounded-full"
                                    }
                                    div {
                                        class: "flex-1",
                                        p {
                                            class: "text-neutral-100 text-sm font-medium",
                                            "New DNO endpoint discovered"
                                        }
                                        p {
                                            class: "text-neutral-400 text-xs",
                                            "3 hours ago"
                                        }
                                    }
                                }
                            }
                        }

                        // Quick Actions
                        div {
                            class: "card",
                            h4 {
                                class: "text-lg font-semibold text-neutral-100 mb-6",
                                "Quick Actions"
                            }
                            div {
                                class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                button {
                                    class: "btn-primary text-left p-4 h-auto",
                                    div {
                                        class: "font-medium mb-1",
                                        "Start Manual Crawl"
                                    }
                                    div {
                                        class: "text-sm opacity-80",
                                        "Trigger data collection"
                                    }
                                }
                                button {
                                    class: "btn-secondary text-left p-4 h-auto",
                                    div {
                                        class: "font-medium mb-1",
                                        "View Reports"
                                    }
                                    div {
                                        class: "text-sm opacity-80",
                                        "Check crawl statistics"
                                    }
                                }
                                button {
                                    class: "bg-neutral-700 hover:bg-neutral-600 text-neutral-100 rounded-xl text-left p-4 h-auto transition-colors",
                                    div {
                                        class: "font-medium mb-1",
                                        "Configuration"
                                    }
                                    div {
                                        class: "text-sm text-neutral-400",
                                        "Manage crawler settings"
                                    }
                                }
                                form {
                                    method: "POST",
                                    action: "/logout",
                                    button {
                                        r#type: "submit",
                                        class: "bg-red-600/20 hover:bg-red-600/30 text-red-400 border border-red-600/30 rounded-xl text-left p-4 h-auto w-full transition-colors",
                                        div {
                                            class: "font-medium mb-1",
                                            "Sign Out"
                                        }
                                        div {
                                            class: "text-sm opacity-80",
                                            "End current session"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}