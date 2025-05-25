#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UserManagementProps {
    #[props(default = String::new())]
    pub current_user_role: String,
}

#[component]
pub fn UserManagement(props: UserManagementProps) -> Element {
    // Only show content if user is admin
    if props.current_user_role != "admin" {
        return rsx! {
            div { class: "max-w-4xl mx-auto",
                div { class: "bg-red-50 border border-red-200 rounded-lg p-6",
                    div { class: "flex items-center",
                        div { class: "flex-shrink-0",
                            svg {
                                class: "h-5 w-5 text-red-400",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                    clip_rule: "evenodd"
                                }
                            }
                        }
                        div { class: "ml-3",
                            h3 { class: "text-sm font-medium text-red-800",
                                "Access Denied"
                            }
                            div { class: "mt-2 text-sm text-red-700",
                                p { "You do not have permission to access user management. Admin privileges required." }
                            }
                        }
                    }
                    div { class: "mt-4",
                        a {
                            href: "/dashboard",
                            class: "text-sm font-medium text-red-800 hover:text-red-900",
                            "Return to Dashboard →"
                        }
                    }
                }
            }
        };
    }

    let mut show_create_modal = use_signal(|| false);

    rsx! {
        div { class: "max-w-7xl mx-auto",
            div { class: "bg-white shadow rounded-lg",
                div { class: "px-4 py-5 sm:p-6",
                    div { class: "sm:flex sm:items-center",
                        div { class: "sm:flex-auto",
                            h1 { class: "text-2xl font-bold text-gray-900",
                                "User Management"
                            }
                            p { class: "mt-2 text-sm text-gray-700",
                                "Manage users, roles, and permissions for the DNO Crawler system."
                            }
                        }
                        div { class: "mt-4 sm:mt-0 sm:ml-16 sm:flex-none",
                            button {
                                r#type: "button",
                                class: "inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto",
                                onclick: move |_| show_create_modal.set(true),
                                "Add User"
                            }
                        }
                    }

                    // User Statistics
                    div { class: "mt-8 grid grid-cols-1 gap-5 sm:grid-cols-3",
                        div { class: "bg-blue-50 overflow-hidden shadow rounded-lg",
                            div { class: "p-5",
                                div { class: "flex items-center",
                                    div { class: "flex-shrink-0",
                                        div { class: "h-8 w-8 bg-blue-500 rounded-md flex items-center justify-center",
                                            span { class: "text-white font-medium", "U" }
                                        }
                                    }
                                    div { class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt { class: "text-sm font-medium text-gray-500 truncate",
                                                "Total Users"
                                            }
                                            dd { class: "text-lg font-medium text-gray-900",
                                                "Loading..."
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "bg-green-50 overflow-hidden shadow rounded-lg",
                            div { class: "p-5",
                                div { class: "flex items-center",
                                    div { class: "flex-shrink-0",
                                        div { class: "h-8 w-8 bg-green-500 rounded-md flex items-center justify-center",
                                            span { class: "text-white font-medium", "A" }
                                        }
                                    }
                                    div { class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt { class: "text-sm font-medium text-gray-500 truncate",
                                                "Active Users"
                                            }
                                            dd { class: "text-lg font-medium text-gray-900",
                                                "Loading..."
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "bg-yellow-50 overflow-hidden shadow rounded-lg",
                            div { class: "p-5",
                                div { class: "flex items-center",
                                    div { class: "flex-shrink-0",
                                        div { class: "h-8 w-8 bg-yellow-500 rounded-md flex items-center justify-center",
                                            span { class: "text-white font-medium", "R" }
                                        }
                                    }
                                    div { class: "ml-5 w-0 flex-1",
                                        dl {
                                            dt { class: "text-sm font-medium text-gray-500 truncate",
                                                "Admin Users"
                                            }
                                            dd { class: "text-lg font-medium text-gray-900",
                                                "Loading..."
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Users Table
                    div { class: "mt-8 flex flex-col",
                        div { class: "-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8",
                            div { class: "inline-block min-w-full py-2 align-middle md:px-6 lg:px-8",
                                div { class: "overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg",
                                    table { class: "min-w-full divide-y divide-gray-300",
                                        thead { class: "bg-gray-50",
                                            tr {
                                                th {
                                                    scope: "col",
                                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                    "User"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                    "Role"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                    "Created"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                                    "Status"
                                                }
                                                th {
                                                    scope: "col",
                                                    class: "relative px-6 py-3",
                                                    span { class: "sr-only", "Actions" }
                                                }
                                            }
                                        }
                                        tbody { class: "bg-white divide-y divide-gray-200",
                                            // Sample row - in real implementation, this would be dynamically generated
                                            tr {
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    div { class: "flex items-center",
                                                        div { class: "h-10 w-10 flex-shrink-0",
                                                            div { class: "h-10 w-10 rounded-full bg-gray-300 flex items-center justify-center",
                                                                span { class: "text-sm font-medium text-gray-700", "K" }
                                                            }
                                                        }
                                                        div { class: "ml-4",
                                                            div { class: "text-sm font-medium text-gray-900",
                                                                "KyleDerZweite"
                                                            }
                                                            div { class: "text-sm text-gray-500",
                                                                "kyle@example.com"
                                                            }
                                                        }
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800",
                                                        "Admin"
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap text-sm text-gray-900",
                                                    "2025-05-25"
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap",
                                                    span { class: "inline-flex px-2 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800",
                                                        "Active"
                                                    }
                                                }
                                                td { class: "px-6 py-4 whitespace-nowrap text-right text-sm font-medium",
                                                    button {
                                                        class: "text-indigo-600 hover:text-indigo-900 mr-3",
                                                        "Edit"
                                                    }
                                                    button {
                                                        class: "text-red-600 hover:text-red-900",
                                                        "Delete"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Create User Modal
                    if show_create_modal() {
                        div { class: "fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full",
                            div { class: "relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white",
                                div { class: "mt-3 text-center",
                                    h3 { class: "text-lg leading-6 font-medium text-gray-900",
                                        "Create New User"
                                    }
                                    form { class: "mt-4 space-y-4",
                                        input {
                                            r#type: "email",
                                            placeholder: "Email",
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                                        }
                                        input {
                                            r#type: "password",
                                            placeholder: "Password",
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                                        }
                                        select {
                                            class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500",
                                            option { value: "user", "User" }
                                            option { value: "admin", "Admin" }
                                        }
                                        div { class: "flex space-x-2",
                                            button {
                                                r#type: "submit",
                                                class: "flex-1 bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700",
                                                "Create"
                                            }
                                            button {
                                                r#type: "button",
                                                class: "flex-1 bg-gray-300 text-gray-700 py-2 px-4 rounded-md hover:bg-gray-400",
                                                onclick: move |_| show_create_modal.set(false),
                                                "Cancel"
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
}