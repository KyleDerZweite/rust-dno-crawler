#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::ui::*;

#[component]
pub fn Demo() -> Element {
    let mut modal_open = use_signal(|| false);
    let mut input_value = use_signal(|| String::new());
    
    rsx! {
        div {
            class: "container mx-auto px-4 py-8 space-y-12",
            
            // Page Header
            div {
                class: "text-center mb-12",
                h1 { class: "text-4xl font-bold text-light-beige-200 mb-4", "UI Components Demo" }
                p { class: "text-light-beige-500", "A showcase of all available UI components in the design system" }
            }

            // Buttons Section
            Card {
                variant: CardVariant::Glass,
                CardHeader {
                    CardTitle { "Buttons" }
                }
                CardContent {
                    div {
                        class: "space-y-6",
                        
                        // Button Variants
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Variants" }
                            div { class: "flex flex-wrap gap-4",
                                PrimaryButton { "Primary" }
                                SecondaryButton { "Secondary" }
                                Button { variant: ButtonVariant::Ghost, "Ghost" }
                                Button { variant: ButtonVariant::Outline, "Outline" }
                                Button { variant: ButtonVariant::Danger, "Danger" }
                                Button { variant: ButtonVariant::Success, "Success" }
                            }
                        }
                        
                        // Button Sizes
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Sizes" }
                            div { class: "flex flex-wrap items-center gap-4",
                                PrimaryButton { size: ButtonSize::Small, "Small" }
                                PrimaryButton { size: ButtonSize::Medium, "Medium" }
                                PrimaryButton { size: ButtonSize::Large, "Large" }
                                PrimaryButton { size: ButtonSize::ExtraLarge, "Extra Large" }
                            }
                        }
                        
                        // Button States
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "States" }
                            div { class: "flex flex-wrap gap-4",
                                PrimaryButton { state: ButtonState::Default, "Default" }
                                PrimaryButton { state: ButtonState::Loading, "Loading" }
                                PrimaryButton { state: ButtonState::Disabled, "Disabled" }
                            }
                        }
                    }
                }
            }

            // Cards Section
            Card {
                variant: CardVariant::Glass,
                CardHeader {
                    CardTitle { "Cards" }
                }
                CardContent {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6",
                        
                        Card {
                            variant: CardVariant::Default,
                            padding: CardPadding::Small,
                            CardContent { "Default Card" }
                        }
                        
                        Card {
                            variant: CardVariant::Bordered,
                            padding: CardPadding::Medium,
                            CardContent { "Bordered Card" }
                        }
                        
                        Card {
                            variant: CardVariant::Elevated,
                            padding: CardPadding::Medium,
                            CardContent { "Elevated Card" }
                        }
                        
                        Card {
                            variant: CardVariant::Glass,
                            padding: CardPadding::Medium,
                            CardContent { "Glass Card" }
                        }
                    }
                }
            }

            // Inputs Section
            Card {
                variant: CardVariant::Glass,
                CardHeader {
                    CardTitle { "Inputs" }
                }
                CardContent {
                    div {
                        class: "space-y-6",
                        
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Text Inputs" }
                            div { class: "space-y-4",
                                Input {
                                    placeholder: "Default input",
                                    value: input_value(),
                                    oninput: move |evt: FormEvent| input_value.set(evt.value()),
                                }
                                Input {
                                    placeholder: "Success state",
                                    variant: InputVariant::Success,
                                }
                                Input {
                                    placeholder: "Warning state",
                                    variant: InputVariant::Warning,
                                }
                                Input {
                                    placeholder: "Error state",
                                    variant: InputVariant::Error,
                                }
                            }
                        }
                        
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Textarea" }
                            Textarea {
                                placeholder: "Enter your message here...",
                                rows: 4,
                            }
                        }
                    }
                }
            }

            // Badges Section
            Card {
                variant: CardVariant::Glass,
                CardHeader {
                    CardTitle { "Badges" }
                }
                CardContent {
                    div {
                        class: "space-y-6",
                        
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Badge Variants" }
                            div { class: "flex flex-wrap gap-3",
                                Badge { variant: BadgeVariant::Default, "Default" }
                                Badge { variant: BadgeVariant::Primary, "Primary" }
                                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                                Badge { variant: BadgeVariant::Success, "Success" }
                                Badge { variant: BadgeVariant::Warning, "Warning" }
                                Badge { variant: BadgeVariant::Danger, "Danger" }
                                Badge { variant: BadgeVariant::Info, "Info" }
                            }
                        }
                        
                        div {
                            h4 { class: "text-lg font-semibold mb-4", "Status Badges" }
                            div { class: "flex flex-wrap gap-3",
                                StatusBadge { status: "online".to_string() }
                                StatusBadge { status: "offline".to_string() }
                                StatusBadge { status: "pending".to_string() }
                                StatusBadge { status: "idle".to_string() }
                            }
                        }
                    }
                }
            }

            // Modal Section
            Card {
                variant: CardVariant::Glass,
                CardHeader {
                    CardTitle { "Modal" }
                }
                CardContent {
                    div {
                        PrimaryButton {
                            onclick: move |_| modal_open.set(true),
                            "Open Modal"
                        }
                    }
                }
            }
        }
        
        // Modal Component
        Modal {
            open: modal_open(),
            onclose: move |_| modal_open.set(false),
            size: ModalSize::Medium,
            
            ModalHeader {
                ModalTitle { "Example Modal" }
            }
            
            ModalBody {
                p { class: "text-light-beige-400", 
                    "This is an example modal dialog. You can put any content here including forms, images, or other components."
                }
            }
            
            ModalFooter {
                GhostButton {
                    onclick: move |_| modal_open.set(false),
                    "Cancel"
                }
                PrimaryButton {
                    onclick: move |_| modal_open.set(false),
                    "Confirm"
                }
            }
        }
    }
}