#![allow(non_snake_case)]
use dioxus::prelude::*;
use chrono::{Local, Datelike, Timelike};

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    #[props(default = "Demo User".to_string())]
    pub name: String,
    #[props(default = "demo@example.com".to_string())]
    pub email: String,
    #[props(default = "user".to_string())]
    pub role: String,
}

#[derive(Clone, PartialEq)]
struct ChatMessage {
    id: u32,
    content: String,
    is_user: bool,
    timestamp: String,
    is_processing: bool,
}

#[derive(Clone, PartialEq)]
struct RunningProcess {
    id: u32,
    name: String,
    status: String,
    progress: u32,
    started_at: String,
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    // State management
    let mut current_input = use_signal(|| String::new());
    let mut chat_messages = use_signal(|| Vec::<ChatMessage>::new());
    let mut is_processing = use_signal(|| false);
    let mut running_processes = use_signal(|| {
        vec![
            RunningProcess {
                id: 1,
                name: "DNO Data Crawl - Batch 1".to_string(),
                status: "Running".to_string(),
                progress: 67,
                started_at: "2025-05-28 22:45:12".to_string(),
            },
            RunningProcess {
                id: 2,
                name: "Data Validation - 2024 Records".to_string(),
                status: "Queued".to_string(),
                progress: 0,
                started_at: "2025-05-28 23:00:00".to_string(),
            },
        ]
    });
    let mut show_jwt_copied = use_signal(|| false);

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

    // Mock data - replace with real data
    let mock_name = if props.name.is_empty() {
        "KyleDerZweite".to_string()
    } else {
        props.name.clone()
    };
    let mock_email = if props.email.is_empty() {
        "kyle@kylehub.dev".to_string()
    } else {
        props.email.clone()
    };
    let mock_role = if props.role.is_empty() {
        "admin".to_string()
    } else {
        props.role.clone()
    };
    let mock_jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJLeWxlRGVyWndlaXRlIiwibmFtZSI6Ikt5bGUiLCJpYXQiOjE3MzI4MzE1MzF9.abc123...".to_string();

    // Theme-aware classes using custom color palette
    let container_bg = "bg-dark-charcoal-900/60 backdrop-blur-xl border border-dark-charcoal-400/40 shadow-2xl shadow-dark-charcoal-900/20";
    let card_bg = "bg-dark-charcoal-600/80 backdrop-blur-lg border border-dark-charcoal-400/30 shadow-xl shadow-dark-charcoal-900/10";
    let input_bg = "bg-dark-charcoal-500/90 border-dark-charcoal-400/50 text-light-beige-300 focus:border-forest-green-500 focus:ring-forest-green-500/20";
    let text_primary = "text-light-beige-300";
    let text_secondary = "text-light-beige-500";
    let accent_bg = "bg-dark-charcoal-500/50";
    let suggestion_button_bg = "bg-forest-green-500/10 hover:bg-forest-green-500/20";
    let suggestion_button_text = "text-light-beige-300";
    let primary_button_bg = "bg-forest-green-500 hover:bg-forest-green-600";
    let primary_button_shadow = "shadow-lg hover:shadow-forest-green-500/30";
    let token_bg = "bg-dark-charcoal-500";
    let copy_button_bg = "bg-dark-charcoal-500";
    let icon_primary = "text-light-beige-400";
    let icon_secondary = "text-light-beige-500";
    let icon_forest = "text-forest-green-400";

    // Mock functions
    let send_message = move |_| {
        let input_value = current_input();
        if !input_value.trim().is_empty() {
            let new_message = ChatMessage {
                id: (chat_messages().len() + 1) as u32,
                content: input_value.clone(),
                is_user: true,
                timestamp: format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second()),
                is_processing: false,
            };

            chat_messages.with_mut(|messages| messages.push(new_message));
            current_input.set(String::new());
            is_processing.set(true);

            // Simulate AI response
            use_future(move || {
                let value = input_value.clone();
                async move {
                    // Simulate processing delay for demo
                    // In a real app, this would be an API call

                    let response_time = Local::now();
                    let ai_response = ChatMessage {
                        id: (chat_messages().len() + 1) as u32,
                        content: format!("Processing your request: '{}'. I found 23 DNO endpoints related to your query. Would you like me to start a targeted crawl?", value),
                        is_user: false,
                        timestamp: format!("{:02}:{:02}:{:02}", response_time.hour(), response_time.minute(), response_time.second()),
                        is_processing: false,
                    };

                    chat_messages.with_mut(|messages| messages.push(ai_response));
                    is_processing.set(false);
                }
            });
        }
    };

    let copy_jwt = move |_| {
        // Mock JWT copy - in real app, use web_sys clipboard API
        show_jwt_copied.set(true);
        use_future(move || async move {
            // Simulate processing delay for demo
            show_jwt_copied.set(false);
        });
    };

    let stop_current_process = move |_| {
        is_processing.set(false);
    };

    let new_chat = move |_| {
        chat_messages.set(Vec::new());
        is_processing.set(false);
    };

    rsx! {
        div {
            class: "w-full min-h-screen py-8 px-4 sm:px-6 lg:px-8",
            // Add top padding to account for floating header
            style: "padding-top: 6rem;",

            // Main container - not full screen, highlighted
            div {
                class: "{container_bg} rounded-3xl p-6 lg:p-8 max-w-7xl mx-auto",

                // Header Section
                div {
                    class: "flex flex-col lg:flex-row lg:items-center lg:justify-between mb-8 gap-6",

                    // Title and Date
                    div {
                        h1 {
                            class: "text-3xl lg:text-4xl font-bold {text_primary} mb-2",
                            "DNO Crawler"
                        }
                        p {
                            class: "{text_secondary} text-lg",
                            "{current_time}"
                        }
                    }

                    // Action buttons row
                    div {
                        class: "flex items-center gap-3 flex-wrap",

                        // New Chat Button - Theme aware
                        button {
                            onclick: new_chat,
                            class: "flex items-center space-x-2 px-4 py-2 {primary_button_bg} {text_primary} rounded-xl transition-all duration-200 hover:scale-105 {primary_button_shadow}",
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
                            span { "New Chat" }
                        }

                        // Stop Process Button (only show when processing)
                        if is_processing() {
                            button {
                                onclick: stop_current_process,
                                class: "flex items-center space-x-2 px-4 py-2 bg-red-500 hover:bg-red-600 {text_primary} rounded-xl transition-all duration-200 hover:scale-105 shadow-lg hover:shadow-red-500/30",
                                svg {
                                    class: "w-5 h-5",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M9 10h6v4H9z"
                                    }
                                }
                                span { "Stop Process" }
                            }
                        }

                        // Admin Configuration (only for admins)
                        if mock_role == "admin" {
                            button {
                                class: "flex items-center space-x-2 px-4 py-2 {accent_bg} {text_primary} border border-stone-300 dark:border-stone-600 rounded-xl transition-all duration-200 hover:scale-105",
                                svg {
                                    class: "w-5 h-5 {text_secondary}",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                    }
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    }
                                }
                                span { class: "{text_secondary}", "Configuration" }
                            }
                        }
                    }
                }

                // Main Content Grid
                div {
                    class: "grid grid-cols-1 xl:grid-cols-4 gap-6 lg:gap-8",

                    // Left Sidebar - User Info & Metrics
                    div {
                        class: "xl:col-span-1 space-y-6",

                        // User Profile Card
                        div {
                            class: "{card_bg} rounded-2xl p-6",

                            // Profile Header
                            div {
                                class: "text-center mb-6",
                                div {
                                    class: "w-8 h-8 bg-forest-green-500 rounded-lg flex items-center justify-center mx-auto mb-4 transition-colors",
                                    span {
                                        class: "text-white font-bold text-sm z-10",
                                        "{mock_email.chars().next().unwrap_or('K').to_uppercase()}"
                                    }
                                }
                                h3 {
                                    class: "text-lg font-semibold {text_primary} mb-1",
                                    "{mock_name}"
                                }
                                span {
                                    class: if mock_role == "admin" {
                                        "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-vibrant-orange-500/20 text-vibrant-orange-400 border border-vibrant-orange-500/30"
                                    } else {
                                        "inline-flex items-center px-3 py-1 rounded-lg text-xs font-medium bg-forest-green-500/20 text-forest-green-400 border border-forest-green-500/30"
                                    },
                                    "{mock_role}"
                                }
                            }

                            // User Details
                            div {
                                class: "space-y-3 {accent_bg} rounded-xl p-4",
                                div {
                                    div {
                                        class: "text-xs font-medium {text_secondary} mb-1 uppercase tracking-wide",
                                        "Email"
                                    }
                                    div {
                                        class: "{text_primary} text-sm break-all",
                                        "{mock_email}"
                                    }
                                }
                                div {
                                    div {
                                        class: "text-xs font-medium {text_secondary} mb-1 uppercase tracking-wide",
                                        "API Token"
                                    }
                                    div {
                                        class: "flex items-center gap-2",
                                        div {
                                            class: "{text_secondary} text-xs font-mono {token_bg} px-2 py-1 rounded truncate flex-1",
                                            "{&mock_jwt[..30]}..."
                                        }
                                        button {
                                            onclick: copy_jwt,
                                            class: if show_jwt_copied() {
                                                "p-1 bg-green-500 {text_primary} rounded hover:bg-green-600 transition-colors"
                                            } else {
                                                "p-1 {copy_button_bg} {text_primary} rounded transition-colors"
                                            },
                                            if show_jwt_copied() {
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M5 13l4 4L19 7"
                                                    }
                                                }
                                            } else {
                                                svg {
                                                    class: "w-4 h-4",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    view_box: "0 0 24 24",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "2",
                                                        d: "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // DNO Metrics Card
                        div {
                            class: "{card_bg} rounded-2xl p-6",
                            h4 {
                                class: "text-lg font-semibold {text_primary} mb-4",
                                "DNO Data Metrics"
                            }
                            div {
                                class: "space-y-4",

                                // Total DNOs
                                div {
                                    class: "{accent_bg} rounded-xl p-4",
                                    div {
                                        class: "flex items-center justify-between mb-2",
                                        span {
                                            class: "text-sm font-medium {text_secondary}",
                                            "Stored DNOs"
                                        }
                                        span {
                                            class: "text-2xl font-bold text-forest-green-500",
                                            "1,247"
                                        }
                                    }
                                    div {
                                        class: "text-xs {text_secondary}",
                                        "Distribution Network Operators"
                                    }
                                }

                                // Data Completeness
                                div {
                                    class: "{accent_bg} rounded-xl p-4",
                                    div {
                                        class: "flex items-center justify-between mb-2",
                                        span {
                                            class: "text-sm font-medium {text_secondary}",
                                            "Completeness (2014-2025)"
                                        }
                                        span {
                                            class: "text-2xl font-bold text-green-500",
                                            "87.3%"
                                        }
                                    }
                                    div {
                                        class: "w-full bg-stone-200 dark:bg-neutral-600 rounded-full h-2 mb-2",
                                        div {
                                            class: "bg-gradient-to-r from-forest-green-500 to-forest-green-600 h-2 rounded-full",
                                            style: "width: 87.3%"
                                        }
                                    }
                                }
                            }
                        }

                        // Running Processes (Admin only)
                        if mock_role == "admin" {
                            div {
                                class: "{card_bg} rounded-2xl p-6",
                                h4 {
                                    class: "text-lg font-semibold {text_primary} mb-4",
                                    "Running Processes"
                                }
                                div {
                                    class: "space-y-3",
                                    for process in running_processes() {
                                        div {
                                            key: "{process.id}",
                                            class: "{accent_bg} rounded-xl p-3",
                                            div {
                                                class: "flex items-center justify-between mb-2",
                                                span {
                                                    class: "text-sm font-medium {text_primary} truncate",
                                                    "{process.name}"
                                                }
                                                span {
                                                    class: if process.status == "Running" {
                                                        "text-xs bg-forest-green-500/20 text-forest-green-400 px-2 py-1 rounded border border-forest-green-500/30"
                                                    } else {
                                                        "text-xs bg-vibrant-orange-500/20 text-vibrant-orange-400 px-2 py-1 rounded border border-vibrant-orange-500/30"
                                                    },
                                                    "{process.status}"
                                                }
                                            }
                                            if process.status == "Running" {
                                                div {
                                                    class: "w-full bg-stone-200 dark:bg-neutral-600 rounded-full h-1.5 mb-1",
                                                    div {
                                                        class: "bg-forest-green-500 h-1.5 rounded-full transition-all duration-300",
                                                        style: "width: {process.progress}%"
                                                    }
                                                }
                                                div {
                                                    class: "text-xs {text_secondary}",
                                                    "{process.progress}% • Started {process.started_at}"
                                                }
                                            } else {
                                                div {
                                                    class: "text-xs {text_secondary}",
                                                    "Queued • {process.started_at}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Main Chat Interface
                    div {
                        class: "xl:col-span-3",
                        div {
                            class: "{card_bg} rounded-2xl flex flex-col",
                            style: "height: 70vh;",

                            // Chat Header
                            div {
                                class: "flex items-center justify-between p-6 border-b border-stone-200 dark:border-stone-700",
                                div {
                                    h3 {
                                        class: "text-xl font-semibold {text_primary}",
                                        "Assistant Chat"
                                    }
                                    p {
                                        class: "text-sm {text_secondary}",
                                        "Ask me anything about DNO data, start crawls, or get system insights"
                                    }
                                }
                                div {
                                    class: "flex items-center space-x-2",
                                    div {
                                        class: if is_processing() {
                                            "w-3 h-3 bg-vibrant-orange-500 rounded-full animate-bounce"
                                        } else {
                                            "w-3 h-3 bg-forest-green-500 rounded-full"
                                        }
                                    }
                                    span {
                                        class: "text-sm {text_secondary}",
                                        if is_processing() { "Processing..." } else { "Ready" }
                                    }
                                }
                            }

                            // Chat Messages Area
                            div {
                                class: "flex-1 overflow-y-auto p-6 space-y-4",

                                // Welcome message if no messages
                                if chat_messages().is_empty() {
                                    div {
                                        class: "{accent_bg} rounded-xl p-6 text-center",
                                        div {
                                            class: "w-16 h-16 bg-forest-green-500/20 rounded-full flex items-center justify-center mx-auto mb-4",
                                            svg {
                                                class: "w-8 h-8 {icon_forest}",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                                                }
                                            }
                                        }
                                        h4 {
                                            class: "text-lg font-semibold {text_primary} mb-2",
                                            "Welcome to DNO Crawler Agent"
                                        }
                                        p {
                                            class: "text-sm {text_secondary} mb-4",
                                            "Start by asking me about DNO data, system status, or request a new crawl"
                                        }
                                        div {
                                            class: "flex flex-wrap gap-2 justify-center",
                                            button {
                                                onclick: move |_| current_input.set("Show me the latest DNO data status".to_string()),
                                                class: "px-3 py-1 {suggestion_button_bg} {suggestion_button_text} rounded-lg text-sm transition-colors",
                                                "Latest status"
                                            }
                                            button {
                                                onclick: move |_| current_input.set("Start a new crawl for missing 2022 Q1 data".to_string()),
                                                class: "px-3 py-1 {suggestion_button_bg} {suggestion_button_text} rounded-lg text-sm transition-colors",
                                                "Start crawl"
                                            }
                                            button {
                                                onclick: move |_| current_input.set("What DNOs are currently offline?".to_string()),
                                                class: "px-3 py-1 {suggestion_button_bg} {suggestion_button_text} rounded-lg text-sm transition-colors",
                                                "Check offline DNOs"
                                            }
                                        }
                                    }
                                }

                                // Chat Messages
                                for message in chat_messages() {
                                    div {
                                        key: "{message.id}",
                                        class: if message.is_user {
                                            "flex justify-end"
                                        } else {
                                            "flex justify-start"
                                        },
                                        div {
                                            class: if message.is_user {
                                                "max-w-xs lg:max-w-md bg-forest-green-500 text-white rounded-2xl rounded-br-md p-4"
                                            } else {
                                                "max-w-xs lg:max-w-md {accent_bg} rounded-2xl rounded-bl-md p-4"
                                            },
                                            p {
                                                class: if message.is_user {
                                                    "text-white"
                                                } else {
                                                    "{text_primary}"
                                                },
                                                "{message.content}"
                                            }
                                            div {
                                                class: if message.is_user {
                                                    "text-xs text-white/70 mt-2"
                                                } else {
                                                    "text-xs {text_secondary} mt-2"
                                                },
                                                "{message.timestamp}"
                                            }
                                        }
                                    }
                                }

                                // Processing indicator
                                if is_processing() {
                                    div {
                                        class: "flex justify-start",
                                        div {
                                            class: "max-w-xs lg:max-w-md {accent_bg} rounded-2xl rounded-bl-md p-4",
                                            div {
                                                class: "flex items-center space-x-2",
                                                div {
                                                    class: "flex space-x-1",
                                                    div { class: "w-2 h-2 bg-forest-green-500 rounded-full animate-bounce" }
                                                    div { class: "w-2 h-2 bg-forest-green-500 rounded-full animate-bounce", style: "animation-delay: 0.1s;" }
                                                    div { class: "w-2 h-2 bg-forest-green-500 rounded-full animate-bounce", style: "animation-delay: 0.2s;" }
                                                }
                                                span {
                                                    class: "text-sm {text_secondary}",
                                                    "AI is thinking..."
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Chat Input
                            div {
                                class: "p-6 border-t border-stone-200 dark:border-stone-700",
                                form {
                                    onsubmit: send_message,
                                    prevent_default: "onsubmit",
                                    class: "flex space-x-4",
                                    input {
                                        r#type: "text",
                                        value: "{current_input}",
                                        oninput: move |evt| current_input.set(evt.value()),
                                        placeholder: "Ask me about DNO data, system status, or request a crawl...",
                                        disabled: is_processing(),
                                        class: "flex-1 px-4 py-3 {input_bg} rounded-xl border focus:outline-none focus:ring-2 transition-all duration-200",
                                    }
                                    button {
                                        r#type: "submit",
                                        disabled: is_processing() || current_input().trim().is_empty(),
                                        class: "px-6 py-3 {primary_button_bg} disabled:bg-forest-green-300 disabled:cursor-not-allowed {text_primary} rounded-xl transition-all duration-200 hover:scale-105 {primary_button_shadow}",
                                        if is_processing() {
                                            "Processing..."
                                        } else {
                                            "Send"
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
