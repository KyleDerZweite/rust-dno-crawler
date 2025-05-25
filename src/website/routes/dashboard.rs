#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    #[props(default = String::new())]
    pub email: String,
    #[props(default = String::new())]
    pub role: String,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    rsx! {
        div { class: "max-w-7xl mx-auto",
            div { class: "bg-white shadow rounded-lg",
                div { class: "px-4 py-5 sm:p-6",
                    div { class: "text-center",
                        h2 { class: "text-2xl font-bold text-gray-900 mb-6",
                            "Welcome to DNO Crawler Dashboard"
                        }
                        div { class: "bg-gray-50 rounded-lg p-6 max-w-sm mx-auto",
                            h3 { class: "text-lg font-medium text-gray-900 mb-4",
                                "User Information"
                            }
                            dl { class: "space-y-3",
                                div {
                                    dt { class: "text-sm font-medium text-gray-500",
                                        "Email"
                                    }
                                    dd { class: "text-sm text-gray-900",
                                        if !props.email.is_empty() {
                                            "{props.email}"
                                        } else {
                                            "Not available"
                                        }
                                    }
                                }
                                div {
                                    dt { class: "text-sm font-medium text-gray-500",
                                        "Role"
                                    }
                                    dd { class: "text-sm text-gray-900",
                                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800",
                                            if !props.role.is_empty() {
                                                "{props.role}"
                                            } else {
                                                "user"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "mt-6 space-y-4",
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div { class: "bg-blue-50 p-4 rounded-lg",
                                    h4 { class: "text-lg font-medium text-blue-900 mb-2",
                                        "DNO Data Collection"
                                    }
                                    p { class: "text-blue-700 text-sm",
                                        "Automated data retrieval from German Distribution Network Operators"
                                    }
                                }
                                div { class: "bg-green-50 p-4 rounded-lg",
                                    h4 { class: "text-lg font-medium text-green-900 mb-2",
                                        "System Status"
                                    }
                                    p { class: "text-green-700 text-sm",
                                        "All systems operational"
                                    }
                                }
                            }
                            form { method: "POST", action: "/logout",
                                button {
                                    r#type: "submit",
                                    class: "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-md text-sm font-medium",
                                    "Logout"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}