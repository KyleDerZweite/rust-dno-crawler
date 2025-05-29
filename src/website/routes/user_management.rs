#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::theme::{use_theme, Theme};
use chrono::{Local, Datelike, Timelike};

#[derive(Props, Clone, PartialEq)]
pub struct UserManagementProps {
    #[props(default = "admin".to_string())]
    pub current_user_role: String,
}

#[derive(Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: String,
    status: String,
    created_at: String,
    last_login: String,
}

#[component]
pub fn UserManagement(props: UserManagementProps) -> Element {
    let theme = use_theme();

    // Mock data
    let mock_users = use_signal(|| vec![
        User {
            id: 1,
            name: "KyleDerZweite".to_string(),
            email: "kyle@kylehub.dev".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
            created_at: "2024-01-15".to_string(),
            last_login: "2025-05-29 00:10:22".to_string(),
        },
        User {
            id: 2,
            name: "Sarah Johnson".to_string(),
            email: "sarah.j@company.com".to_string(),
            role: "guest".to_string(),
            status: "active".to_string(),
            created_at: "2024-03-22".to_string(),
            last_login: "2025-05-28 18:45:12".to_string(),
        },
        User {
            id: 3,
            name: "Mike Chen".to_string(),
            email: "mike.chen@company.com".to_string(),
            role: "user".to_string(),
            status: "inactive".to_string(),
            created_at: "2024-02-08".to_string(),
            last_login: "2025-05-20 14:32:55".to_string(),
        },
        User {
            id: 4,
            name: "Admin Assistant".to_string(),
            email: "admin@company.com".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
            created_at: "2024-01-01".to_string(),
            last_login: "2025-05-28 22:18:44".to_string(),
        },
    ]);

    let mut show_create_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut selected_user_id = use_signal(|| 0u32);
    let mut search_query = use_signal(|| String::new());
    let mut selected_role_filter = use_signal(|| "all".to_string());

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

    // Only show content if user is admin
    if props.current_user_role != "admin" {
        return rsx! {
            div {
                class: "w-full min-h-screen py-8 px-4 sm:px-6 lg:px-8",
                style: "padding-top: 6rem;",
                div {
                    class: match theme() {
                        Theme::Light => "max-w-4xl mx-auto bg-white/60 backdrop-blur-xl border border-stone-200/40 shadow-2xl shadow-stone-500/10 rounded-3xl p-8",
                        Theme::Dark => "max-w-4xl mx-auto bg-neutral-900/60 backdrop-blur-xl border border-stone-700/40 shadow-2xl shadow-black/20 rounded-3xl p-8",
                    },
                    div {
                        class: match theme() {
                            Theme::Light => "bg-red-50/80 border border-red-200/60 rounded-2xl p-6",
                            Theme::Dark => "bg-red-900/20 border border-red-600/40 rounded-2xl p-6",
                        },
                        div { class: "flex items-center",
                            div { class: "flex-shrink-0",
                                svg {
                                    class: match theme() {
                                        Theme::Light => "h-6 w-6 text-red-500",
                                        Theme::Dark => "h-6 w-6 text-red-400",
                                    },
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path {
                                        fill_rule: "evenodd",
                                        d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
                                        clip_rule: "evenodd"
                                    }
                                }
                            }
                            div { class: "ml-4",
                                h3 {
                                    class: match theme() {
                                        Theme::Light => "text-lg font-semibold text-red-800",
                                        Theme::Dark => "text-lg font-semibold text-red-200",
                                    },
                                    "Access Denied"
                                }
                                div {
                                    class: match theme() {
                                        Theme::Light => "mt-2 text-red-700",
                                        Theme::Dark => "mt-2 text-red-300",
                                    },
                                    p { "You do not have permission to access user management. Admin privileges required." }
                                }
                            }
                        }
                        div { class: "mt-6",
                            a {
                                href: "/dashboard",
                                class: match theme() {
                                    Theme::Light => "inline-flex items-center px-4 py-2 bg-red-600 hover:bg-red-700 {text_primary} rounded-xl transition-all duration-200 hover:scale-105 shadow-lg",
                                    Theme::Dark => "inline-flex items-center px-4 py-2 bg-red-500 hover:bg-red-600 {text_primary} rounded-xl transition-all duration-200 hover:scale-105 shadow-lg",
                                },
                                "← Return to Dashboard"
                            }
                        }
                    }
                }
            }
        };
    }

    // Theme-aware classes
    let container_bg = match theme() {
        Theme::Light => "bg-white/60 backdrop-blur-xl border border-stone-200/40 shadow-2xl shadow-stone-500/10",
        Theme::Dark => "bg-neutral-900/60 backdrop-blur-xl border border-stone-700/40 shadow-2xl shadow-black/20",
    };

    let card_bg = match theme() {
        Theme::Light => "bg-white/80 backdrop-blur-lg border border-stone-200/30 shadow-lg shadow-stone-500/5",
        Theme::Dark => "bg-neutral-800/80 backdrop-blur-lg border border-stone-700/30 shadow-xl shadow-black/10",
    };

    let input_bg = match theme() {
        Theme::Light => "bg-white/90 border-stone-300/50 text-stone-800 focus:border-moss-500 focus:ring-moss-500/20",
        Theme::Dark => "bg-neutral-700/90 border-stone-600/50 text-stone-100 focus:border-moss-500 focus:ring-moss-500/20",
    };

    let text_primary = match theme() {
        Theme::Light => "text-stone-800",
        Theme::Dark => "text-stone-100",
    };

    let text_secondary = match theme() {
        Theme::Light => "text-stone-600",
        Theme::Dark => "text-stone-400",
    };

    let accent_bg = match theme() {
        Theme::Light => "bg-stone-50/80",
        Theme::Dark => "bg-neutral-700/50",
    };

    let table_header_bg = match theme() {
        Theme::Light => "bg-stone-100/80",
        Theme::Dark => "bg-neutral-800/80",
    };

    let table_row_bg = match theme() {
        Theme::Light => "bg-white/40 hover:bg-white/60",
        Theme::Dark => "bg-neutral-800/40 hover:bg-neutral-800/60",
    };

    let primary_button_bg = match theme() {
        Theme::Light => "bg-moss-600 hover:bg-moss-700",
        Theme::Dark => "bg-moss-500 hover:bg-moss-600",
    };

    // Filter users based on search and role
    let filtered_users = use_memo(move || {
        let users = mock_users();
        let query = search_query().to_lowercase();
        let role_filter = selected_role_filter();

        users.into_iter().filter(|user| {
            let matches_search = query.is_empty() ||
                user.name.to_lowercase().contains(&query) ||
                user.email.to_lowercase().contains(&query);
            let matches_role = role_filter == "all" || user.role == role_filter;

            matches_search && matches_role
        }).collect::<Vec<_>>()
    });

    // Calculate statistics
    let total_users = mock_users().len();
    let active_users = mock_users().iter().filter(|u| u.status == "active").count();
    let admin_users = mock_users().iter().filter(|u| u.role == "admin").count();

    rsx! {
        div {
            class: "w-full min-h-screen py-8 px-4 sm:px-6 lg:px-8",
            style: "padding-top: 6rem;",

            // Main container
            div {
                class: "{container_bg} rounded-3xl p-6 lg:p-8 max-w-7xl mx-auto",

                // Header Section
                div {
                    class: "flex flex-col lg:flex-row lg:items-center lg:justify-between mb-8 gap-6",

                    // Title and Date
                    div {
                        h1 {
                            class: "text-3xl lg:text-4xl font-bold {text_primary} mb-2",
                            "User Management"
                        }
                        p {
                            class: "{text_secondary} text-lg",
                            "{current_time}"
                        }
                        p {
                            class: "{text_secondary} text-sm mt-1",
                            "Manage users, roles, and permissions for the DNO Crawler system"
                        }
                    }

                    // Add User Button
                    button {
                        onclick: move |_| show_create_modal.set(true),
                        class: "flex items-center space-x-2 px-6 py-3 {primary_button_bg} {text_primary} rounded-xl transition-all duration-200 hover:scale-105 shadow-lg hover:shadow-moss-500/30",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 6v6m0 0v6m0-6h6m-6 0H6"
                            }
                        }
                        span { "Add User" }
                    }
                }

                // Statistics Cards
                div {
                    class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-8",

                    // Total Users
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "w-12 h-12 bg-blue-500/20 rounded-xl flex items-center justify-center",
                                svg {
                                    class: "w-6 h-6 text-blue-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
                                    }
                                }
                            }
                            div {
                                class: "ml-4",
                                h3 {
                                    class: "text-sm font-medium {text_secondary} mb-1",
                                    "Total Users"
                                }
                                p {
                                    class: "text-2xl font-bold text-blue-500",
                                    "{total_users}"
                                }
                            }
                        }
                    }

                    // Active Users
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "w-12 h-12 bg-green-500/20 rounded-xl flex items-center justify-center",
                                svg {
                                    class: "w-6 h-6 text-green-600",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                }
                            }
                            div {
                                class: "ml-4",
                                h3 {
                                    class: "text-sm font-medium {text_secondary} mb-1",
                                    "Active Users"
                                }
                                p {
                                    class: "text-2xl font-bold text-green-500",
                                    "{active_users}"
                                }
                            }
                        }
                    }

                    // Admin Users
                    div {
                        class: "{card_bg} rounded-2xl p-6",
                        div {
                            class: "flex items-center",
                            div {
                                class: "w-12 h-12 bg-amber-500/20 rounded-xl flex items-center justify-center",
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
                            div {
                                class: "ml-4",
                                h3 {
                                    class: "text-sm font-medium {text_secondary} mb-1",
                                    "Admin Users"
                                }
                                p {
                                    class: "text-2xl font-bold text-amber-500",
                                    "{admin_users}"
                                }
                            }
                        }
                    }
                }

                // Search and Filter Section
                div {
                    class: "{card_bg} rounded-2xl p-6 mb-8",
                    div {
                        class: "flex flex-col sm:flex-row gap-4",

                        // Search Input
                        div {
                            class: "flex-1",
                            label {
                                class: "block text-sm font-medium {text_secondary} mb-2",
                                "Search Users"
                            }
                            input {
                                r#type: "text",
                                value: "{search_query}",
                                oninput: move |evt| search_query.set(evt.value()),
                                placeholder: "Search by name or email...",
                                class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                            }
                        }

                        // Role Filter
                        div {
                            class: "sm:w-48",
                            label {
                                class: "block text-sm font-medium {text_secondary} mb-2",
                                "Filter by Role"
                            }
                            select {
                                value: "{selected_role_filter}",
                                onchange: move |evt| selected_role_filter.set(evt.value()),
                                class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                option { value: "all", "All Roles" }
                                option { value: "admin", "Admin" }
                                option { value: "user", "User" }
                                option { value: "guest", "Guest" }
                            }
                        }
                    }
                }

                // Users Table
                div {
                    class: "{card_bg} rounded-2xl overflow-hidden",
                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full",
                            thead {
                                class: "{table_header_bg}",
                                tr {
                                    th {
                                        class: "px-6 py-4 text-left text-xs font-semibold {text_secondary} uppercase tracking-wider",
                                        "User"
                                    }
                                    th {
                                        class: "px-6 py-4 text-left text-xs font-semibold {text_secondary} uppercase tracking-wider",
                                        "Role"
                                    }
                                    th {
                                        class: "px-6 py-4 text-left text-xs font-semibold {text_secondary} uppercase tracking-wider",
                                        "Status"
                                    }
                                    th {
                                        class: "px-6 py-4 text-left text-xs font-semibold {text_secondary} uppercase tracking-wider",
                                        "Last Login"
                                    }
                                    th {
                                        class: "px-6 py-4 text-left text-xs font-semibold {text_secondary} uppercase tracking-wider",
                                        "Actions"
                                    }
                                }
                            }
                            tbody {
                                class: "divide-y divide-stone-200 dark:divide-stone-700",
                                for user in filtered_users() {
                                    tr {
                                        key: "{user.id}",
                                        class: "{table_row_bg} transition-colors duration-200",

                                        // User Info
                                        td {
                                            class: "px-6 py-4",
                                            div {
                                                class: "flex items-center space-x-4",
                                                div {
                                                    class: "w-10 h-10 bg-moss-500 rounded-xl flex items-center justify-center",
                                                    span {
                                                        class: "{text_primary} font-bold text-sm",
                                                        "{user.email.chars().next().unwrap_or('U').to_uppercase()}"
                                                    }
                                                }
                                                div {
                                                    h4 {
                                                        class: "text-sm font-semibold {text_primary}",
                                                        "{user.name}"
                                                    }
                                                    p {
                                                        class: "text-sm {text_secondary}",
                                                        "{user.email}"
                                                    }
                                                }
                                            }
                                        }

                                        // Role
                                        td {
                                            class: "px-6 py-4",
                                            span {
                                                class: if user.role == "admin" {
                                                    "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-amber-500/20 text-amber-600 border border-amber-500/30"
                                                } else {
                                                    "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-blue-500/20 text-blue-600 border border-blue-500/30"
                                                },
                                                "{user.role}"
                                            }
                                        }

                                        // Status
                                        td {
                                            class: "px-6 py-4",
                                            span {
                                                class: if user.status == "active" {
                                                    "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-green-500/20 text-green-600 border border-green-500/30"
                                                } else {
                                                    "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-gray-500/20 text-gray-600 border border-gray-500/30"
                                                },
                                                "• {user.status}"
                                            }
                                        }

                                        // Last Login
                                        td {
                                            class: "px-6 py-4",
                                            p {
                                                class: "text-sm {text_primary}",
                                                "{user.last_login}"
                                            }
                                        }

                                        // Actions
                                        td {
                                            class: "px-6 py-4",
                                            div {
                                                class: "flex items-center space-x-3",
                                                button {
                                                    onclick: move |_| {
                                                        selected_user_id.set(user.id);
                                                        show_edit_modal.set(true);
                                                    },
                                                    class: "text-sm font-medium {text_primary} hover:text-moss-700 transition-colors",
                                                    "Edit"
                                                }
                                                button {
                                                    class: "text-sm font-medium text-red-600 hover:text-red-700 transition-colors",
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
                    div {
                        class: "fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50",
                        onclick: move |_| show_create_modal.set(false),
                        div {
                            class: "{card_bg} rounded-2xl p-8 w-full max-w-md mx-4",
                            onclick: move |evt| evt.stop_propagation(),
                            h3 {
                                class: "text-xl font-bold {text_primary} mb-6",
                                "Create New User"
                            }
                            form {
                                class: "space-y-4",
                                div {
                                    label {
                                        class: "block text-sm font-medium {text_secondary} mb-2",
                                        "Full Name"
                                    }
                                    input {
                                        r#type: "text",
                                        placeholder: "Enter full name",
                                        class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                    }
                                }
                                div {
                                    label {
                                        class: "block text-sm font-medium {text_secondary} mb-2",
                                        "Email Address"
                                    }
                                    input {
                                        r#type: "email",
                                        placeholder: "Enter email address",
                                        class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                    }
                                }
                                div {
                                    label {
                                        class: "block text-sm font-medium {text_secondary} mb-2",
                                        "Password"
                                    }
                                    input {
                                        r#type: "password",
                                        placeholder: "Enter password",
                                        class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                    }
                                }
                                div {
                                    label {
                                        class: "block text-sm font-medium {text_secondary} mb-2",
                                        "Role"
                                    }
                                    select {
                                        class: "w-full px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                        option { value: "guest", "Guest" }
                                        option { value: "user", "User" }
                                        option { value: "admin", "Admin" }
                                    }
                                }
                                div {
                                    class: "flex space-x-4 pt-4",
                                    button {
                                        r#type: "submit",
                                        class: "flex-1 {primary_button_bg} {text_primary} py-3 px-4 rounded-xl font-medium transition-all duration-200 hover:scale-105 shadow-lg",
                                        "Create User"
                                    }
                                    button {
                                        r#type: "button",
                                        onclick: move |_| show_create_modal.set(false),
                                        class: "flex-1 {accent_bg} {text_secondary} py-3 px-4 rounded-xl font-medium transition-all duration-200 hover:scale-105",
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