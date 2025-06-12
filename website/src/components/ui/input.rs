#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputSize {
    Small,
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(default = "text".to_string())]
    pub input_type: String,
    #[props(default = "".to_string())]
    pub placeholder: String,
    #[props(default = "".to_string())]
    pub value: String,
    #[props(default = None)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default = None)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default = InputVariant::Default)]
    pub variant: InputVariant,
    #[props(default = InputSize::Medium)]
    pub size: InputSize,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default = "".to_string())]
    pub name: String,
    #[props(default = "".to_string())]
    pub id: String,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = None)]
    pub icon_start: Option<Element>,
    #[props(default = None)]
    pub icon_end: Option<Element>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    let base_classes = "w-full transition-all duration-200 focus:outline-none focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50";
    
    let size_classes = match props.size {
        InputSize::Small => "px-4 py-3 text-base sm:px-3 sm:py-1.5 sm:text-sm rounded-lg min-h-[44px] sm:min-h-[36px] touch-manipulation",
        InputSize::Medium => "px-5 py-3 text-base sm:px-4 sm:py-2 sm:text-sm rounded-xl min-h-[44px] sm:min-h-[40px] touch-manipulation",
        InputSize::Large => "px-6 py-4 text-lg sm:px-5 sm:py-3 sm:text-base rounded-xl min-h-[48px] sm:min-h-[44px] touch-manipulation",
    };

    let variant_classes = match props.variant {
        InputVariant::Default => "bg-dark-charcoal-400 border border-dark-charcoal-300 text-light-beige-300 placeholder-light-beige-600 focus:border-forest-green-500 focus:ring-forest-green-500/20",
        InputVariant::Success => "bg-dark-charcoal-400 border border-forest-green-400 text-light-beige-300 placeholder-light-beige-600 focus:border-forest-green-400 focus:ring-forest-green-400/20",
        InputVariant::Warning => "bg-dark-charcoal-400 border border-amber-brown-500 text-light-beige-300 placeholder-light-beige-600 focus:border-amber-brown-500 focus:ring-amber-brown-500/20",
        InputVariant::Error => "bg-dark-charcoal-400 border border-sienna-brown-500 text-light-beige-300 placeholder-light-beige-600 focus:border-sienna-brown-500 focus:ring-sienna-brown-500/20",
    };

    let classes = format!("{} {} {} {}", base_classes, size_classes, variant_classes, props.class);
    
    let has_icons = props.icon_start.is_some() || props.icon_end.is_some();

    if has_icons {
        let has_start_icon = props.icon_start.is_some();
        let has_end_icon = props.icon_end.is_some();
        
        rsx! {
            div {
                class: "relative",
                if let Some(icon) = props.icon_start {
                    div {
                        class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-light-beige-600",
                        {icon}
                    }
                }
                input {
                    r#type: props.input_type,
                    placeholder: props.placeholder,
                    value: props.value,
                    oninput: move |evt| {
                        if let Some(handler) = &props.oninput {
                            handler.call(evt);
                        }
                    },
                    onchange: move |evt| {
                        if let Some(handler) = &props.onchange {
                            handler.call(evt);
                        }
                    },
                    disabled: props.disabled,
                    required: props.required,
                    name: props.name,
                    id: props.id,
                    class: format!("{} {} {}", 
                        classes,
                        if has_start_icon { "pl-10" } else { "" },
                        if has_end_icon { "pr-10" } else { "" }
                    ),
                }
                if let Some(icon) = props.icon_end {
                    div {
                        class: "absolute right-3 top-1/2 transform -translate-y-1/2 text-light-beige-600",
                        {icon}
                    }
                }
            }
        }
    } else {
        rsx! {
            input {
                r#type: props.input_type,
                placeholder: props.placeholder,
                value: props.value,
                oninput: move |evt| {
                    if let Some(handler) = &props.oninput {
                        handler.call(evt);
                    }
                },
                onchange: move |evt| {
                    if let Some(handler) = &props.onchange {
                        handler.call(evt);
                    }
                },
                disabled: props.disabled,
                required: props.required,
                name: props.name,
                id: props.id,
                class: classes,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TextareaProps {
    #[props(default = "".to_string())]
    pub placeholder: String,
    #[props(default = "".to_string())]
    pub value: String,
    #[props(default = None)]
    pub oninput: Option<EventHandler<FormEvent>>,
    #[props(default = None)]
    pub onchange: Option<EventHandler<FormEvent>>,
    #[props(default = InputVariant::Default)]
    pub variant: InputVariant,
    #[props(default = InputSize::Medium)]
    pub size: InputSize,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub required: bool,
    #[props(default = "".to_string())]
    pub name: String,
    #[props(default = "".to_string())]
    pub id: String,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = 4)]
    pub rows: i32,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    let base_classes = "w-full transition-all duration-200 focus:outline-none focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50 resize-y";
    
    let size_classes = match props.size {
        InputSize::Small => "px-3 py-1.5 text-sm rounded-lg",
        InputSize::Medium => "px-4 py-2 text-sm rounded-xl",
        InputSize::Large => "px-5 py-3 text-base rounded-xl",
    };

    let variant_classes = match props.variant {
        InputVariant::Default => "bg-dark-charcoal-400 border border-dark-charcoal-300 text-light-beige-300 placeholder-light-beige-600 focus:border-forest-green-500 focus:ring-forest-green-500/20",
        InputVariant::Success => "bg-dark-charcoal-400 border border-forest-green-400 text-light-beige-300 placeholder-light-beige-600 focus:border-forest-green-400 focus:ring-forest-green-400/20",
        InputVariant::Warning => "bg-dark-charcoal-400 border border-amber-brown-500 text-light-beige-300 placeholder-light-beige-600 focus:border-amber-brown-500 focus:ring-amber-brown-500/20",
        InputVariant::Error => "bg-dark-charcoal-400 border border-sienna-brown-500 text-light-beige-300 placeholder-light-beige-600 focus:border-sienna-brown-500 focus:ring-sienna-brown-500/20",
    };

    let classes = format!("{} {} {} {}", base_classes, size_classes, variant_classes, props.class);

    rsx! {
        textarea {
            placeholder: props.placeholder,
            value: props.value,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &props.onchange {
                    handler.call(evt);
                }
            },
            disabled: props.disabled,
            required: props.required,
            name: props.name,
            id: props.id,
            rows: props.rows,
            class: classes,
        }
    }
}