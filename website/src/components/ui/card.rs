#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Bordered,
    Elevated,
    Glass,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardPadding {
    None,
    Small,
    Medium,
    Large,
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default = CardVariant::Default)]
    pub variant: CardVariant,
    #[props(default = CardPadding::Medium)]
    pub padding: CardPadding,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let base_classes = "rounded-xl transition-all duration-200";
    
    let variant_classes = match props.variant {
        CardVariant::Default => "bg-dark-charcoal-500 text-light-beige-300",
        CardVariant::Bordered => "bg-dark-charcoal-500 text-light-beige-300 border border-dark-charcoal-400",
        CardVariant::Elevated => "bg-dark-charcoal-500 text-light-beige-300 shadow-xl shadow-dark-charcoal-900/20",
        CardVariant::Glass => "bg-dark-charcoal-500/80 backdrop-blur-lg border border-dark-charcoal-400/30 shadow-xl shadow-dark-charcoal-900/10 text-light-beige-300",
    };

    let padding_classes = match props.padding {
        CardPadding::None => "",
        CardPadding::Small => "p-4 sm:p-5 md:p-6",
        CardPadding::Medium => "p-5 sm:p-6 md:p-8", 
        CardPadding::Large => "p-6 sm:p-8 md:p-10",
    };

    let interactive_classes = if props.onclick.is_some() {
        "cursor-pointer hover:scale-105 hover:shadow-2xl"
    } else {
        ""
    };

    let classes = format!("{} {} {} {} {}", 
        base_classes, variant_classes, padding_classes, interactive_classes, props.class);

    rsx! {
        div {
            class: classes,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let classes = format!("mb-4 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let classes = format!("text-xl font-semibold text-light-beige-200 {}", props.class);
    
    rsx! {
        h3 {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let classes = format!("text-light-beige-400 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let classes = format!("mt-4 pt-4 border-t border-dark-charcoal-400 {}", props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}