#![allow(non_snake_case)]
use dioxus::prelude::*;
use super::theme::{get_focus_ring_classes, accessibility};
use super::animations::{AnimationPresets, AnimationUtils, LoadingAnimation};

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
    Outline,
    Danger,
    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonState {
    Default,
    Loading,
    Disabled,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    #[props(default = ButtonState::Default)]
    pub state: ButtonState,
    #[props(default = "button".to_string())]
    pub button_type: String,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = None)]
    pub href: Option<String>,
    #[props(default = None)]
    pub icon_start: Option<Element>,
    #[props(default = None)]
    pub icon_end: Option<Element>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = format!(
        "inline-flex items-center justify-center font-medium disabled:cursor-not-allowed {} {}",
        get_focus_ring_classes(),
        AnimationUtils::respect_reduced_motion(&AnimationPresets::button_interaction())
    );
    
    let size_classes = match props.size {
        ButtonSize::Small => "px-4 py-2.5 text-base sm:px-3 sm:py-2 sm:text-sm rounded-lg gap-2 sm:gap-1.5 min-h-[44px] sm:min-h-[40px] touch-manipulation",
        ButtonSize::Medium => "px-5 py-3 text-base sm:px-4 sm:py-2.5 sm:text-sm rounded-lg gap-2 min-h-[48px] sm:min-h-[44px] touch-manipulation",
        ButtonSize::Large => "px-6 py-3.5 text-lg sm:px-5 sm:py-3 sm:text-base rounded-xl gap-3 sm:gap-2 min-h-[52px] sm:min-h-[48px] touch-manipulation",
        ButtonSize::ExtraLarge => "px-8 py-4 text-xl sm:px-6 sm:py-3.5 sm:text-lg rounded-xl gap-4 sm:gap-3 min-h-[56px] sm:min-h-[52px] touch-manipulation",
    };

    let variant_classes = match props.variant {
        ButtonVariant::Primary => match props.state {
            ButtonState::Default => "bg-forest-green-500 hover:bg-forest-green-600 text-light-beige-200 shadow-lg focus:ring-forest-green-500",
            ButtonState::Loading => "bg-forest-green-500 text-light-beige-200 shadow-lg cursor-not-allowed",
            ButtonState::Disabled => "bg-dark-charcoal-400 text-light-beige-700 cursor-not-allowed",
        },
        ButtonVariant::Secondary => match props.state {
            ButtonState::Default => "bg-vibrant-orange-500 hover:bg-vibrant-orange-600 text-light-beige-200 shadow-lg focus:ring-vibrant-orange-500",
            ButtonState::Loading => "bg-vibrant-orange-500 text-light-beige-200 shadow-lg cursor-not-allowed",
            ButtonState::Disabled => "bg-dark-charcoal-400 text-light-beige-700 cursor-not-allowed",
        },
        ButtonVariant::Ghost => match props.state {
            ButtonState::Default => "bg-transparent hover:bg-dark-charcoal-600 text-light-beige-500 hover:text-light-beige-300 border-0 focus:ring-forest-green-500",
            ButtonState::Loading => "bg-transparent text-light-beige-500 cursor-not-allowed",
            ButtonState::Disabled => "bg-transparent text-light-beige-700 cursor-not-allowed",
        },
        ButtonVariant::Outline => match props.state {
            ButtonState::Default => "bg-transparent hover:bg-forest-green-500 text-forest-green-400 hover:text-light-beige-200 border-2 border-forest-green-500 hover:border-forest-green-500 focus:ring-forest-green-500",
            ButtonState::Loading => "bg-transparent text-forest-green-400 border-2 border-forest-green-500 cursor-not-allowed",
            ButtonState::Disabled => "bg-transparent text-light-beige-700 border-2 border-dark-charcoal-400 cursor-not-allowed",
        },
        ButtonVariant::Danger => match props.state {
            ButtonState::Default => "bg-vibrant-orange-600 hover:bg-vibrant-orange-700 text-light-beige-200 shadow-lg focus:ring-vibrant-orange-500",
            ButtonState::Loading => "bg-vibrant-orange-600 text-light-beige-200 shadow-lg cursor-not-allowed",
            ButtonState::Disabled => "bg-dark-charcoal-400 text-light-beige-700 cursor-not-allowed",
        },
        ButtonVariant::Success => match props.state {
            ButtonState::Default => "bg-forest-green-400 hover:bg-forest-green-500 text-light-beige-200 shadow-lg focus:ring-forest-green-400",
            ButtonState::Loading => "bg-forest-green-400 text-light-beige-200 shadow-lg cursor-not-allowed",
            ButtonState::Disabled => "bg-dark-charcoal-400 text-light-beige-700 cursor-not-allowed",
        },
    };

    let classes = format!("{} {} {} {}", base_classes, size_classes, variant_classes, props.class);
    let is_disabled = matches!(props.state, ButtonState::Disabled | ButtonState::Loading);
    
    // Generate ARIA label for accessibility
    let aria_label = accessibility::get_interactive_aria_label(
        "button", 
        match props.state {
            ButtonState::Loading => Some("loading"),
            ButtonState::Disabled => Some("disabled"),
            _ => None,
        }
    );

    // Render as link if href is provided
    if let Some(href) = props.href {
        rsx! {
            a {
                href: href,
                class: classes,
                "aria-label": aria_label.clone(),
                "aria-disabled": if is_disabled { "true" } else { "false" },
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                {props.icon_start}
                span { {props.children} }
                if matches!(props.state, ButtonState::Loading) {
                    LoadingAnimation {
                        animation_type: "spin".to_string(),
                        size: "h-4 w-4".to_string(),
                        color: "currentColor".to_string(),
                        class: "ml-2".to_string(),
                    }
                }
                {props.icon_end}
            }
        }
    } else {
        rsx! {
            button {
                r#type: props.button_type,
                class: classes,
                disabled: is_disabled,
                "aria-label": aria_label,
                "aria-disabled": if is_disabled { "true" } else { "false" },
                onclick: move |evt| {
                    if !is_disabled {
                        if let Some(handler) = &props.onclick {
                            handler.call(evt);
                        }
                    }
                },
                {props.icon_start}
                span { {props.children} }
                if matches!(props.state, ButtonState::Loading) {
                    LoadingAnimation {
                        animation_type: "spin".to_string(),
                        size: "h-4 w-4".to_string(),
                        color: "currentColor".to_string(),
                        class: "ml-2".to_string(),
                    }
                }
                {props.icon_end}
            }
        }
    }
}


// Convenience components for common button types
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryButtonProps {
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    #[props(default = ButtonState::Default)]
    pub state: ButtonState,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = None)]
    pub href: Option<String>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Primary,
            size: props.size,
            state: props.state,
            onclick: props.onclick,
            href: props.href,
            class: props.class,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SecondaryButtonProps {
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    #[props(default = ButtonState::Default)]
    pub state: ButtonState,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = None)]
    pub href: Option<String>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn SecondaryButton(props: SecondaryButtonProps) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            size: props.size,
            state: props.state,
            onclick: props.onclick,
            href: props.href,
            class: props.class,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct GhostButtonProps {
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    #[props(default = ButtonState::Default)]
    pub state: ButtonState,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = None)]
    pub href: Option<String>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn GhostButton(props: GhostButtonProps) -> Element {
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: props.size,
            state: props.state,
            onclick: props.onclick,
            href: props.href,
            class: props.class,
            {props.children}
        }
    }
}