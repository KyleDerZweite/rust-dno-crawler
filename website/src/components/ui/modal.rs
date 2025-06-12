#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    #[props(default = false)]
    pub open: bool,
    #[props(default = None)]
    pub onclose: Option<EventHandler<()>>,
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,
    #[props(default = true)]
    pub show_close: bool,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Modal(props: ModalProps) -> Element {
    if !props.open {
        return rsx! { span { style: "display: none;" } };
    }

    let size_classes = match props.size {
        ModalSize::Small => "max-w-md",
        ModalSize::Medium => "max-w-lg",
        ModalSize::Large => "max-w-2xl",
        ModalSize::ExtraLarge => "max-w-4xl",
    };

    let modal_classes = format!("relative bg-dark-charcoal-600 rounded-2xl shadow-2xl w-full {} {}", 
        size_classes, props.class);

    rsx! {
        div {
            class: "fixed inset-0 z-50 overflow-y-auto",
            onclick: move |_| {
                if let Some(handler) = &props.onclose {
                    handler.call(());
                }
            },
            // Backdrop
            div {
                class: "fixed inset-0 bg-black/50 backdrop-blur-sm transition-opacity",
            }
            
            // Modal container
            div {
                class: "flex min-h-full items-center justify-center p-4",
                div {
                    class: modal_classes,
                    onclick: |evt| evt.prevent_default(), // Prevent backdrop click from closing
                    
                    if props.show_close {
                        button {
                            class: "absolute top-4 right-4 text-light-beige-600 hover:text-light-beige-300 transition-colors",
                            onclick: move |_| {
                                if let Some(handler) = &props.onclose {
                                    handler.call(());
                                }
                            },
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }
                    }
                    
                    {props.children}
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalHeaderProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ModalHeader(props: ModalHeaderProps) -> Element {
    let classes = format!("px-6 py-4 border-b border-dark-charcoal-400 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalTitleProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ModalTitle(props: ModalTitleProps) -> Element {
    let classes = format!("text-xl font-semibold text-light-beige-200 {}", props.class);
    
    rsx! {
        h3 {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalBodyProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ModalBody(props: ModalBodyProps) -> Element {
    let classes = format!("px-6 py-4 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ModalFooterProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ModalFooter(props: ModalFooterProps) -> Element {
    let classes = format!("px-6 py-4 border-t border-dark-charcoal-400 flex justify-end gap-3 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}