#![allow(non_snake_case)]
//! Animation system for smooth transitions and micro-interactions
//! 
//! This module provides utilities for creating consistent animations across components.
//! It includes predefined animation classes, transition utilities, and accessibility considerations.

use dioxus::prelude::*;

/// Animation timing presets for consistent motion design
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationTiming {
    /// Quick interactions (100ms) - button clicks, small state changes
    Quick,
    /// Standard interactions (200ms) - hover states, toggles
    Standard,
    /// Moderate interactions (300ms) - dropdowns, collapsible content
    Moderate,
    /// Slow interactions (500ms) - page transitions, large content changes
    Slow,
    /// Custom duration in milliseconds
    Custom(u32),
}

impl AnimationTiming {
    pub fn to_class(&self) -> &'static str {
        match self {
            AnimationTiming::Quick => "duration-100",
            AnimationTiming::Standard => "duration-200", 
            AnimationTiming::Moderate => "duration-300",
            AnimationTiming::Slow => "duration-500",
            AnimationTiming::Custom(_) => "duration-300", // fallback for now
        }
    }
    
    pub fn to_ms(&self) -> u32 {
        match self {
            AnimationTiming::Quick => 100,
            AnimationTiming::Standard => 200,
            AnimationTiming::Moderate => 300,
            AnimationTiming::Slow => 500,
            AnimationTiming::Custom(ms) => *ms,
        }
    }
}

/// Animation easing curves for natural motion
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationEasing {
    /// Linear progression - rarely used for UI
    Linear,
    /// Ease in - slow start, accelerates
    EaseIn,
    /// Ease out - fast start, decelerates (recommended for most interactions)
    EaseOut,
    /// Ease in-out - slow start and end, fast middle
    EaseInOut,
    /// Custom cubic-bezier curve
    Custom(&'static str),
}

impl AnimationEasing {
    pub fn to_class(&self) -> &'static str {
        match self {
            AnimationEasing::Linear => "ease-linear",
            AnimationEasing::EaseIn => "ease-in",
            AnimationEasing::EaseOut => "ease-out",
            AnimationEasing::EaseInOut => "ease-in-out",
            AnimationEasing::Custom(_) => "ease-out", // fallback
        }
    }
}

/// Pre-built animation combinations for common UI patterns
pub struct AnimationPresets;

impl AnimationPresets {
    /// Smooth button interactions with scale and shadow
    pub fn button_interaction() -> String {
        format!(
            "transition-all {} {} hover:scale-105 hover:shadow-xl active:scale-95",
            AnimationTiming::Standard.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Card hover effects with elevation
    pub fn card_hover() -> String {
        format!(
            "transition-all {} {} hover:shadow-2xl hover:-translate-y-1",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Input focus states
    pub fn input_focus() -> String {
        format!(
            "transition-all {} {} focus:scale-105 focus:shadow-lg",
            AnimationTiming::Standard.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Modal/overlay entrance animations
    pub fn modal_entrance() -> String {
        format!(
            "transition-all {} {} animate-in fade-in-0 zoom-in-95 slide-in-from-bottom-4",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Modal/overlay exit animations
    pub fn modal_exit() -> String {
        format!(
            "transition-all {} {} animate-out fade-out-0 zoom-out-95 slide-out-to-bottom-4",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseIn.to_class()
        )
    }
    
    /// Dropdown menu animations
    pub fn dropdown_entrance() -> String {
        format!(
            "transition-all {} {} animate-in fade-in-0 zoom-in-95 slide-in-from-top-2",
            AnimationTiming::Standard.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Loading spinner rotation
    pub fn loading_spin() -> String {
        "animate-spin".to_string()
    }
    
    /// Pulse animation for loading states
    pub fn loading_pulse() -> String {
        "animate-pulse".to_string()
    }
    
    /// Bounce animation for notifications/alerts
    pub fn notification_bounce() -> String {
        format!(
            "animate-bounce {} {}",
            AnimationTiming::Standard.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Slide transitions for page changes
    pub fn page_slide_enter() -> String {
        format!(
            "transition-transform {} {} animate-in slide-in-from-right-full",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseOut.to_class()
        )
    }
    
    /// Slide transitions for page exits
    pub fn page_slide_exit() -> String {
        format!(
            "transition-transform {} {} animate-out slide-out-to-left-full",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseIn.to_class()
        )
    }
}

/// Utility functions for creating custom animations
pub struct AnimationUtils;

impl AnimationUtils {
    /// Create a custom transition with specific properties
    pub fn custom_transition(
        properties: &[&str],
        timing: AnimationTiming,
        easing: AnimationEasing,
    ) -> String {
        let props = if properties.is_empty() {
            "all".to_string()
        } else {
            properties.join(",")
        };
        
        format!(
            "transition-[{}] {} {}",
            props,
            timing.to_class(),
            easing.to_class()
        )
    }
    
    /// Add animation delay
    pub fn with_delay(base_animation: &str, delay_ms: u32) -> String {
        let delay_class = match delay_ms {
            0 => "delay-0",
            75 => "delay-75",
            100 => "delay-100",
            150 => "delay-150", 
            200 => "delay-200",
            300 => "delay-300",
            500 => "delay-500",
            700 => "delay-700",
            1000 => "delay-1000",
            _ => "delay-200", // fallback
        };
        
        format!("{} {}", base_animation, delay_class)
    }
    
    /// Check if user prefers reduced motion for accessibility
    pub fn respect_reduced_motion(animation_classes: &str) -> String {
        format!("motion-safe:{} motion-reduce:transition-none", animation_classes)
    }
    
    /// Stagger animations for lists of items
    pub fn stagger_delay(index: usize, base_delay: u32) -> String {
        let delay = base_delay * (index as u32 + 1);
        match delay {
            0..=75 => "delay-75",
            76..=100 => "delay-100", 
            101..=150 => "delay-150",
            151..=200 => "delay-200",
            201..=300 => "delay-300",
            301..=500 => "delay-500",
            _ => "delay-500",
        }.to_string()
    }
}

/// Animation wrapper component for adding entrance animations to any content
#[derive(Props, Clone, PartialEq)]
pub struct AnimatedWrapperProps {
    #[props(default = AnimationPresets::modal_entrance())]
    pub animation: String,
    #[props(default = 0)]
    pub delay_ms: u32,
    #[props(default = true)]
    pub respect_reduced_motion: bool,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AnimatedWrapper(props: AnimatedWrapperProps) -> Element {
    let mut animation_class = props.animation.clone();
    
    if props.delay_ms > 0 {
        animation_class = AnimationUtils::with_delay(&animation_class, props.delay_ms);
    }
    
    if props.respect_reduced_motion {
        animation_class = AnimationUtils::respect_reduced_motion(&animation_class);
    }
    
    let classes = format!("{} {}", animation_class, props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}

/// Loading state animation component
#[derive(Props, Clone, PartialEq)]
pub struct LoadingAnimationProps {
    #[props(default = "spin".to_string())]
    pub animation_type: String, // "spin", "pulse", "bounce"
    #[props(default = "h-6 w-6".to_string())]
    pub size: String,
    #[props(default = "currentColor".to_string())]
    pub color: String,
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn LoadingAnimation(props: LoadingAnimationProps) -> Element {
    let animation_class = match props.animation_type.as_str() {
        "spin" => AnimationPresets::loading_spin(),
        "pulse" => AnimationPresets::loading_pulse(),
        "bounce" => AnimationPresets::notification_bounce(),
        _ => AnimationPresets::loading_spin(),
    };
    
    let classes = format!("{} {} {}", animation_class, props.size, props.class);
    
    match props.animation_type.as_str() {
        "spin" => rsx! {
            svg {
                class: classes,
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                circle {
                    class: "opacity-25",
                    cx: "12",
                    cy: "12", 
                    r: "10",
                    stroke: props.color.clone(),
                    "stroke-width": "4"
                }
                path {
                    class: "opacity-75",
                    fill: props.color.clone(),
                    d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                }
            }
        },
        "pulse" => rsx! {
            div {
                class: classes,
                style: format!("background-color: {};", props.color.clone()),
            }
        },
        "bounce" => rsx! {
            div {
                class: format!("{} rounded-full", classes),
                style: format!("background-color: {};", props.color.clone()),
            }
        },
        _ => rsx! {
            div {
                class: classes,
                style: format!("background-color: {};", props.color.clone()),
            }
        }
    }
}

/// Page transition component for route changes
#[derive(Props, Clone, PartialEq)]
pub struct PageTransitionProps {
    #[props(default = "slide".to_string())]
    pub transition_type: String, // "slide", "fade", "scale"
    #[props(default = "enter".to_string())]
    pub direction: String, // "enter", "exit"
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn PageTransition(props: PageTransitionProps) -> Element {
    let animation_class = match (props.transition_type.as_str(), props.direction.as_str()) {
        ("slide", "enter") => AnimationPresets::page_slide_enter(),
        ("slide", "exit") => AnimationPresets::page_slide_exit(),
        ("fade", "enter") => format!(
            "transition-opacity {} {} animate-in fade-in-0",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseOut.to_class()
        ),
        ("fade", "exit") => format!(
            "transition-opacity {} {} animate-out fade-out-0", 
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseIn.to_class()
        ),
        ("scale", "enter") => format!(
            "transition-transform {} {} animate-in zoom-in-95",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseOut.to_class()
        ),
        ("scale", "exit") => format!(
            "transition-transform {} {} animate-out zoom-out-95",
            AnimationTiming::Moderate.to_class(),
            AnimationEasing::EaseIn.to_class()
        ),
        _ => AnimationPresets::page_slide_enter(),
    };
    
    let animation_class = AnimationUtils::respect_reduced_motion(&animation_class);
    let classes = format!("{} {}", animation_class, props.class);
    
    rsx! {
        div {
            class: classes,
            {props.children}
        }
    }
}