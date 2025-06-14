use dioxus::prelude::*;

#[component]
pub fn DnoList() -> Element {
    rsx! {
        div { class: "max-w-6xl mx-auto",
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                DnoCard {
                    name: "Avacon Netz GmbH",
                    region: "Niedersachsen",
                    website: "https://www.avacon-netz.de"
                }
                DnoCard {
                    name: "Bayernwerk Netz GmbH",
                    region: "Bayern",
                    website: "https://www.bayernwerk-netz.de"
                }
                DnoCard {
                    name: "E.ON Netz GmbH",
                    region: "Multiple Regions",
                    website: "https://www.eon-netz.de"
                }
            }
        }
    }
}

#[component]
fn DnoCard(name: String, region: String, website: String) -> Element {
    rsx! {
        div { class: "bg-stone-800/50 backdrop-blur-sm rounded-xl p-6 border border-stone-700 hover:border-green-500/50 transition-all duration-300",
            div { class: "mb-4",
                h3 { class: "text-lg font-semibold text-green-400 mb-2", "{name}" }
                p { class: "text-stone-300 text-sm mb-1",
                    span { class: "text-stone-400", "Region: " }
                    "{region}"
                }
                p { class: "text-stone-300 text-sm",
                    span { class: "text-stone-400", "Website: " }
                    a {
                        href: "{website}",
                        target: "_blank",
                        class: "text-green-400 hover:text-green-300 hover:underline",
                        "{website}"
                    }
                }
            }
            div { class: "flex space-x-2",
                button {
                    class: "flex-1 px-3 py-2 bg-green-600 hover:bg-green-500 text-white text-sm rounded-lg transition-colors",
                    "View Details"
                }
                button {
                    class: "flex-1 px-3 py-2 bg-stone-600 hover:bg-stone-500 text-white text-sm rounded-lg transition-colors",
                    "Crawl Data"
                }
            }
        }
    }
}