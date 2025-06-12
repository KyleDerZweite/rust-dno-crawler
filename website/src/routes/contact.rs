#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Contact() -> Element {
    let mut form_submitted = use_signal(|| false);
    let mut selected_category = use_signal(|| "general".to_string());

    // Theme-aware classes using custom color palette
    let bg_color = "bg-dark-charcoal-800";
    let heading_color = "text-light-beige-200";
    let text_color = "text-light-beige-500";
    let label_color = "text-light-beige-300";
    let input_bg = "bg-dark-charcoal-500 border-dark-charcoal-400 text-light-beige-300 focus:border-forest-green-500 focus:ring-forest-green-500";
    let select_bg = "bg-dark-charcoal-500 border-dark-charcoal-400 text-light-beige-300 focus:border-forest-green-500 focus:ring-forest-green-500";
    let info_card_bg = "bg-forest-green-900/20 border border-forest-green-800/30";
    let info_text = "text-forest-green-300";

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
                            class: "w-16 h-16 bg-forest-green-500 rounded-2xl flex items-center justify-center",
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
                        Card {
                            variant: CardVariant::Glass,
                            padding: CardPadding::Medium,
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
                                        class: "flex-shrink-0 w-10 h-10 bg-forest-green-500 rounded-lg flex items-center justify-center",
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
                                            class: format!("{} text-sm hover:text-forest-green-400 transition-colors", text_color),
                                            "info@kylehub.dev"
                                        }
                                    }
                                }

                                // GitHub
                                div {
                                    class: "flex items-start space-x-3",
                                    div {
                                        class: "flex-shrink-0 w-10 h-10 bg-dark-charcoal-400 rounded-lg flex items-center justify-center",
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
                                            class: format!("{} text-sm hover:text-forest-green-400 transition-colors", text_color),
                                            "@KyleDerZweite"
                                        }
                                    }
                                }

                                // Response Time
                                div {
                                    class: "flex items-start space-x-3",
                                    div {
                                        class: "flex-shrink-0 w-10 h-10 bg-amber-brown-500 rounded-lg flex items-center justify-center",
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
                        Card {
                            variant: CardVariant::Elevated,
                            padding: CardPadding::Medium,
                            class: "bg-forest-green-900/20 border border-forest-green-800/30".to_string(),
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
                        Card {
                            variant: CardVariant::Glass,
                            padding: CardPadding::Large,

                            if form_submitted() {
                                // Success Message
                                div {
                                    class: "text-center py-12",
                                    div {
                                        class: "w-16 h-16 bg-forest-green-500 rounded-full flex items-center justify-center mx-auto mb-4",
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
                                        class: "text-forest-green-400 hover:text-forest-green-300 font-medium transition-colors underline-offset-4 hover:underline",
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
                                            Input {
                                                id: "name".to_string(),
                                                name: "name".to_string(),
                                                input_type: "text".to_string(),
                                                placeholder: "Your full name".to_string(),
                                                required: true,
                                            }
                                        }
                                        div {
                                            label {
                                                r#for: "email",
                                                class: format!("block text-sm font-medium {} mb-2", label_color),
                                                "Email *"
                                            }
                                            Input {
                                                id: "email".to_string(),
                                                name: "email".to_string(),
                                                input_type: "email".to_string(),
                                                placeholder: "your@email.com".to_string(),
                                                required: true,
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
                                        Input {
                                            id: "subject".to_string(),
                                            name: "subject".to_string(),
                                            input_type: "text".to_string(),
                                            placeholder: match selected_category().as_str() {
                                                "technical" => "Technical issue with...",
                                                "bug" => "Bug found in...",
                                                "feature" => "Feature request for...",
                                                "contribution" => "I'd like to contribute to...",
                                                "business" => "Business inquiry about...",
                                                _ => "Subject of your message"
                                            }.to_string(),
                                            required: true,
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
                                        PrimaryButton {
                                            size: ButtonSize::Large,
                                            class: "w-full".to_string(),
                                            span { class: "flex items-center justify-center space-x-2",
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
}
