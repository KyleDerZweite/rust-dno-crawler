#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "container mx-auto px-4 py-8",
            h1 { class: "text-3xl font-bold text-white mb-6", "Contact Us" }
            p { class: "text-gray-300 mb-8", "Get in touch with the DNO Crawler team." }
            
            div { class: "bg-gray-900/80 backdrop-blur-lg border border-white/10 p-8 rounded-xl shadow-2xl max-w-2xl mx-auto",
                form { class: "space-y-4",
                    div {
                        label { class: "block text-gray-300 mb-2", "Name" }
                        input { 
                            class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white transition-all duration-300 focus:ring-2 focus:ring-emerald-500",
                            r#type: "text", 
                            placeholder: "Your name" 
                        }
                    }
                    div {
                        label { class: "block text-gray-300 mb-2", "Email" }
                        input { 
                            class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white transition-all duration-300 focus:ring-2 focus:ring-emerald-500",
                            r#type: "email", 
                            placeholder: "your@email.com" 
                        }
                    }
                    div {
                        label { class: "block text-gray-300 mb-2", "Message" }
                        textarea { 
                            class: "w-full px-4 py-3 bg-gray-800/50 border border-gray-600/50 rounded-xl text-white h-32 transition-all duration-300 focus:ring-2 focus:ring-emerald-500",
                            placeholder: "Your message" 
                        }
                    }
                    button { 
                        class: "bg-gradient-to-r from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white font-medium py-3 px-6 rounded-xl transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-105",
                        "Send Message" 
                    }
                }
            }
        }
    }
}