#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "max-w-2xl mx-auto",
            div { class: "bg-white shadow rounded-lg",
                div { class: "px-4 py-5 sm:p-6",
                    h1 { class: "text-3xl font-bold text-gray-900 mb-6 text-center",
                        "Contact Us"
                    }
                    form { class: "space-y-6",
                        div {
                            label { 
                                r#for: "name",
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Name"
                            }
                            input {
                                id: "name",
                                name: "name",
                                r#type: "text",
                                required: true,
                                class: "block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                placeholder: "Your name"
                            }
                        }
                        div {
                            label { 
                                r#for: "email",
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Email"
                            }
                            input {
                                id: "email",
                                name: "email",
                                r#type: "email",
                                required: true,
                                class: "block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                placeholder: "your@email.com"
                            }
                        }
                        div {
                            label { 
                                r#for: "subject",
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Subject"
                            }
                            input {
                                id: "subject",
                                name: "subject",
                                r#type: "text",
                                required: true,
                                class: "block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                placeholder: "Subject"
                            }
                        }
                        div {
                            label { 
                                r#for: "message",
                                class: "block text-sm font-medium text-gray-700 mb-1",
                                "Message"
                            }
                            textarea {
                                id: "message",
                                name: "message",
                                rows: "4",
                                required: true,
                                class: "block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm",
                                placeholder: "Your message..."
                            }
                        }
                        div {
                            button {
                                r#type: "submit",
                                class: "w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500",
                                "Send Message"
                            }
                        }
                    }
                }
            }
        }
    }
}