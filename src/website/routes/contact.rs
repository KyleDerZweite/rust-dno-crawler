#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::website::theme::{use_theme, Theme};

#[component]
pub fn Contact() -> Element {
    let theme = use_theme();
    let mut form_submitted = use_signal(|| false);
    let mut selected_category = use_signal(|| "general".to_string());

    // Theme-aware classes
    let bg_color = match theme() {
        Theme::Light => "bg-gray-100",
        Theme::Dark => "bg-neutral-900",
    };

    let card_bg = match theme() {
        Theme::Light => "bg-white shadow-lg border border-gray-200",
        Theme::Dark => "bg-neutral-800 shadow-xl border border-neutral-700",
    };

    let heading_color = match theme() {
        Theme::Light => "text-neutral-800",
        Theme::Dark => "text-neutral-100",
    };

    let text_color = match theme() {
        Theme::Light => "text-neutral-600",
        Theme::Dark => "text-neutral-400",
    };

    let label_color = match theme() {
        Theme::Light => "text-neutral-700",
        Theme::Dark => "text-neutral-100",
    };

    let input_bg = match theme() {
        Theme::Light => "bg-white border-gray-300 text-neutral-800 focus:border-green-500 focus:ring-green-500",
        Theme::Dark => "bg-neutral-700 border-neutral-600 text-neutral-100 focus:border-green-500 focus:ring-green-500",
    };

    let select_bg = match theme() {
        Theme::Light => "bg-white border-gray-300 text-neutral-800 focus:border-green-500 focus:ring-green-500",
        Theme::Dark => "bg-neutral-700 border-neutral-600 text-neutral-100 focus:border-green-500 focus:ring-green-500",
    };

    let info_card_bg = match theme() {
        Theme::Light => "bg-green-50 border border-green-200",
        Theme::Dark => "bg-green-900/20 border border-green-800/30",
    };

    let info_text = match theme() {
        Theme::Light => "text-green-700",
        Theme::Dark => "text-green-300",
    };

    rsx! {
        div {
            class: format!("flex-grow {} py-12 px-4 sm:px-6 lg:px-8", bg_color),
            div {
                class: "max-w-4xl mx-auto",

                // Header Section
                div {
                    class: "text-center mb-12",
                    // Logo
                    div {
                        class: "flex justify-center mb-6",
                        div {
                            class: "w-16 h-16 bg-green-500 rounded-2xl flex items-center justify-center",
                            svg {
                                class: "w-8 h-8 text-white",
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
                    }
                    h1 {
                        class: format!("text-4xl md:text-5xl font-bold {} mb-4", heading_color),
                        "Get in Touch"
                    }
                    p {
                        class: format!("{} text-lg max-w-2xl mx-auto", text_color),
                        "Have a question about DNO Crawler? Need technical support? Want to contribute to the project? We'd love to hear from you!"
                    }
                }

                div {
                    class: "grid grid-cols-1 lg:grid-cols-3 gap-8",

                    // Contact Information
                    div {
                        class: "lg:col-span-1 space-y-6",

                        // Quick Contact Info
                        div {
                            class: format!("{} rounded-2xl p-6", card_bg),
                            h3 {
                                class: format!("text-xl font-semibold {} mb-4", heading_color),
                                "Contact Information"
                            }
                            div {
                                class: "space-y-4",

                                // Email
                                div {
                                    class: "flex items-start space-x-3",
                                    div {
                                        class: "flex-shrink-0 w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center",
                                        svg {
                                            class: "w-5 h-5 text-white",
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
                                    div {
                                        h4 {
                                            class: format!("font-medium {}", label_color),
                                            "Email"
                                        }
                                        a {
                                            href: "mailto:info@kylehub.dev",
                                            class: format!("{} text-sm hover:text-green-500 transition-colors", text_color),
                                            "info@kylehub.dev"
                                        }
                                    }
                                }

                                // GitHub
                                div {
                                    class: "flex items-start space-x-3",
                                    div {
                                        class: "flex-shrink-0 w-10 h-10 bg-neutral-600 rounded-lg flex items-center justify-center",
                                        svg {
                                            class: "w-5 h-5 text-white",
                                            fill: "currentColor",
                                            view_box: "0 0 24 24",
                                            path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                        }
                                    }
                                    div {
                                        h4 {
                                            class: format!("font-medium {}", label_color),
                                            "GitHub"
                                        }
                                        a {
                                            href: "https://github.com/KyleDerZweite",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: format!("{} text-sm hover:text-green-500 transition-colors", text_color),
                                            "@KyleDerZweite"
                                        }
                                    }
                                }

                                // Response Time
                                div {
                                    class: "flex items-start space-x-3",
                                    div {
                                        class: "flex-shrink-0 w-10 h-10 bg-amber-500 rounded-lg flex items-center justify-center",
                                        svg {
                                            class: "w-5 h-5 text-white",
                                            fill: "none",
                                            stroke: "currentColor",
                                            view_box: "0 0 24 24",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                            }
                                        }
                                    }
                                    div {
                                        h4 {
                                            class: format!("font-medium {}", label_color),
                                            "Response Time"
                                        }
                                        p {
                                            class: format!("{} text-sm", text_color),
                                            "Usually within 24-48 hours"
                                        }
                                    }
                                }
                            }
                        }

                        // Additional Info
                        div {
                            class: format!("{} rounded-2xl p-6", info_card_bg),
                            h4 {
                                class: format!("font-semibold {} mb-2", info_text),
                                "Open Source Project"
                            }
                            p {
                                class: format!("{} text-sm", info_text),
                                "DNO Crawler is an open-source project. Feel free to contribute, report bugs, or suggest features on our GitHub repository!"
                            }
                        }
                    }

                    // Contact Form
                    div {
                        class: "lg:col-span-2",
                        div {
                            class: format!("{} rounded-2xl p-8", card_bg),

                            if form_submitted() {
                                // Success Message
                                div {
                                    class: "text-center py-12",
                                    div {
                                        class: "w-16 h-16 bg-green-500 rounded-full flex items-center justify-center mx-auto mb-4",
                                        svg {
                                            class: "w-8 h-8 text-white",
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
                                    }
                                    h3 {
                                        class: format!("text-2xl font-bold {} mb-2", heading_color),
                                        "Message Sent!"
                                    }
                                    p {
                                        class: format!("{} mb-6", text_color),
                                        "Thank you for reaching out. We'll get back to you within 24-48 hours."
                                    }
                                    button {
                                        onclick: move |_| form_submitted.set(false),
                                        class: "text-green-500 hover:text-green-600 font-medium transition-colors underline-offset-4 hover:underline",
                                        "Send another message"
                                    }
                                }
                            } else {
                                // Contact Form
                                form {
                                    class: "space-y-6",
                                    onsubmit: move |_| {
                                        form_submitted.set(true);
                                    },

                                    h2 {
                                        class: format!("text-2xl font-bold {} mb-6", heading_color),
                                        "Send us a Message"
                                    }

                                    // Contact Category
                                    div {
                                        label {
                                            r#for: "category",
                                            class: format!("block text-sm font-medium {} mb-2", label_color),
                                            "What can we help you with?"
                                        }
                                        select {
                                            id: "category",
                                            name: "category",
                                            required: true,
                                            class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", select_bg),
                                            onchange: move |evt| {
                                                selected_category.set(evt.value());
                                            },
                                            option { value: "general", "General Question" }
                                            option { value: "technical", "Technical Support" }
                                            option { value: "bug", "Bug Report" }
                                            option { value: "feature", "Feature Request" }
                                            option { value: "contribution", "Contribution/Collaboration" }
                                            option { value: "business", "Business Inquiry" }
                                        }
                                    }

                                    // Name and Email Row
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                        div {
                                            label {
                                                r#for: "name",
                                                class: format!("block text-sm font-medium {} mb-2", label_color),
                                                "Name *"
                                            }
                                            input {
                                                id: "name",
                                                name: "name",
                                                r#type: "text",
                                                required: true,
                                                class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                                placeholder: "Your full name"
                                            }
                                        }
                                        div {
                                            label {
                                                r#for: "email",
                                                class: format!("block text-sm font-medium {} mb-2", label_color),
                                                "Email *"
                                            }
                                            input {
                                                id: "email",
                                                name: "email",
                                                r#type: "email",
                                                required: true,
                                                class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                                placeholder: "your@email.com"
                                            }
                                        }
                                    }

                                    // Subject
                                    div {
                                        label {
                                            r#for: "subject",
                                            class: format!("block text-sm font-medium {} mb-2", label_color),
                                            "Subject *"
                                        }
                                        input {
                                            id: "subject",
                                            name: "subject",
                                            r#type: "text",
                                            required: true,
                                            class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200", input_bg),
                                            placeholder: match selected_category().as_str() {
                                                "technical" => "Technical issue with...",
                                                "bug" => "Bug found in...",
                                                "feature" => "Feature request for...",
                                                "contribution" => "I'd like to contribute to...",
                                                "business" => "Business inquiry about...",
                                                _ => "Subject of your message"
                                            }
                                        }
                                    }

                                    // Message
                                    div {
                                        label {
                                            r#for: "message",
                                            class: format!("block text-sm font-medium {} mb-2", label_color),
                                            "Message *"
                                        }
                                        textarea {
                                            id: "message",
                                            name: "message",
                                            rows: "6",
                                            required: true,
                                            class: format!("w-full px-3 py-2 rounded-xl {} transition-all duration-200 resize-y", input_bg),
                                            placeholder: match selected_category().as_str() {
                                                "technical" => "Please describe the technical issue you're experiencing. Include any error messages, steps to reproduce, and your system information if relevant.",
                                                "bug" => "Please describe the bug you found. Include steps to reproduce, expected behavior, and actual behavior.",
                                                "feature" => "Please describe the feature you'd like to see added and how it would benefit the project.",
                                                "contribution" => "Tell us about how you'd like to contribute to the DNO Crawler project. This could be code, documentation, testing, or other forms of contribution.",
                                                "business" => "Please describe your business inquiry or how you'd like to use DNO Crawler in your organization.",
                                                _ => "Please provide as much detail as possible about your question or request..."
                                            }
                                        }
                                        p {
                                            class: format!("mt-1 text-xs {}", text_color),
                                            "Please be as detailed as possible to help us provide the best assistance."
                                        }
                                    }

                                    // Submit Button
                                    div {
                                        button {
                                            r#type: "submit",
                                            class: "w-full bg-green-500 hover:bg-green-600 text-white py-3 px-6 rounded-xl text-base font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105 flex items-center justify-center space-x-2",
                                            svg {
                                                class: "w-5 h-5",
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M12 19l9 2-9-18-9 18 9-2zm0 0v-8"
                                                }
                                            }
                                            span { "Send Message" }
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