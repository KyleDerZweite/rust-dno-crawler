#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BadgeVariant {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default = BadgeVariant::Default)]
    pub variant: BadgeVariant,
    #[props(default = BadgeSize::Medium)]
    pub size: BadgeSize,
    #[props(default = false)]
    pub dot: bool,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = None)]
    pub icon: Option<Element>,
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base_classes = "inline-flex items-center font-medium rounded-full";
    
    let size_classes = match props.size {
        BadgeSize::Small => "px-2 py-0.5 text-xs gap-1",
        BadgeSize::Medium => "px-2.5 py-0.5 text-sm gap-1.5",
        BadgeSize::Large => "px-3 py-1 text-sm gap-2",
    };

    let variant_classes = match props.variant {
        BadgeVariant::Default => "bg-dark-charcoal-500 text-light-beige-400 border border-dark-charcoal-300",
        BadgeVariant::Primary => "bg-forest-green-500/20 text-forest-green-300 border border-forest-green-500/30",
        BadgeVariant::Secondary => "bg-vibrant-orange-500/20 text-vibrant-orange-300 border border-vibrant-orange-500/30",
        BadgeVariant::Success => "bg-forest-green-400/20 text-forest-green-300 border border-forest-green-400/30",
        BadgeVariant::Warning => "bg-vibrant-orange-400/20 text-vibrant-orange-300 border border-vibrant-orange-400/30",
        BadgeVariant::Danger => "bg-vibrant-orange-500/20 text-vibrant-orange-300 border border-vibrant-orange-500/30",
        BadgeVariant::Info => "bg-blue-500/20 text-blue-600 border border-blue-500/30",
    };

    let classes = format!("{} {} {} {}", base_classes, size_classes, variant_classes, props.class);

    rsx! {
        span {
            class: classes,
            if props.dot {
                span {
                    class: "w-1.5 h-1.5 rounded-full bg-current",
                }
            }
            if let Some(icon) = props.icon {
                span {
                    class: "w-4 h-4",
                    {icon}
                }
            }
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct StatusBadgeProps {
    #[props(default = "online".to_string())]
    pub status: String,
    #[props(default = BadgeSize::Medium)]
    pub size: BadgeSize,
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn StatusBadge(props: StatusBadgeProps) -> Element {
    let (variant, text) = match props.status.to_lowercase().as_str() {
        "online" | "active" | "running" => (BadgeVariant::Success, "Online"),
        "offline" | "inactive" | "stopped" => (BadgeVariant::Danger, "Offline"),
        "pending" | "loading" => (BadgeVariant::Warning, "Pending"),
        "idle" | "paused" => (BadgeVariant::Info, "Idle"),
        _ => (BadgeVariant::Default, props.status.as_str()),
    };

    rsx! {
        Badge {
            variant: variant,
            size: props.size,
            dot: true,
            class: props.class,
            {text}
        }
    }
}