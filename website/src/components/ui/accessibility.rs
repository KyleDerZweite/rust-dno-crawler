#![allow(non_snake_case)]
//! Advanced accessibility enhancements for keyboard navigation and screen reader support
//! 
//! This module provides comprehensive accessibility features including:
//! - Keyboard navigation management
//! - Screen reader optimizations
//! - ARIA attributes and roles
//! - Focus management
//! - Skip links and landmarks

use dioxus::prelude::*;

/// Keyboard navigation utilities
pub struct KeyboardNav;

impl KeyboardNav {
    /// Handle arrow key navigation for lists and menus
    pub fn handle_arrow_navigation(key: &str, current_index: i32, max_index: i32) -> Option<i32> {
        match key {
            "ArrowDown" => {
                let next = current_index + 1;
                if next <= max_index { Some(next) } else { Some(0) }
            },
            "ArrowUp" => {
                let prev = current_index - 1;
                if prev >= 0 { Some(prev) } else { Some(max_index) }
            },
            "Home" => Some(0),
            "End" => Some(max_index),
            _ => None,
        }
    }
    
    /// Handle Enter and Space key activation
    pub fn is_activation_key(key: &str) -> bool {
        matches!(key, "Enter" | " " | "Space")
    }
    
    /// Handle Escape key for dismissing overlays
    pub fn is_escape_key(key: &str) -> bool {
        key == "Escape"
    }
    
    /// Handle Tab key for focus management
    pub fn is_tab_key(key: &str, shift: bool) -> (bool, bool) {
        (key == "Tab", shift)
    }
}

/// ARIA attributes and roles for semantic HTML
pub struct AriaAttributes;

impl AriaAttributes {
    /// Get ARIA expanded attribute for collapsible content
    pub fn expanded(is_expanded: bool) -> String {
        if is_expanded { "true".to_string() } else { "false".to_string() }
    }
    
    /// Get ARIA selected attribute for selectable items
    pub fn selected(is_selected: bool) -> String {
        if is_selected { "true".to_string() } else { "false".to_string() }
    }
    
    /// Get ARIA checked attribute for checkboxes and radio buttons
    pub fn checked(is_checked: bool) -> String {
        if is_checked { "true".to_string() } else { "false".to_string() }
    }
    
    /// Get ARIA disabled attribute
    pub fn disabled(is_disabled: bool) -> String {
        if is_disabled { "true".to_string() } else { "false".to_string() }
    }
    
    /// Get ARIA hidden attribute for decorative elements
    pub fn hidden(is_hidden: bool) -> String {
        if is_hidden { "true".to_string() } else { "false".to_string() }
    }
    
    /// Get ARIA live region attribute for dynamic content
    pub fn live(level: &str) -> &str {
        match level {
            "assertive" => "assertive",
            "polite" => "polite",
            _ => "off",
        }
    }
    
    /// Get ARIA role for semantic elements
    pub fn role(role_type: &str) -> &str {
        match role_type {
            "button" => "button",
            "link" => "link",
            "menuitem" => "menuitem",
            "tab" => "tab",
            "tabpanel" => "tabpanel",
            "dialog" => "dialog",
            "alertdialog" => "alertdialog",
            "navigation" => "navigation",
            "main" => "main",
            "banner" => "banner",
            "complementary" => "complementary",
            "contentinfo" => "contentinfo",
            "region" => "region",
            "article" => "article",
            "listbox" => "listbox",
            "option" => "option",
            "combobox" => "combobox",
            "tree" => "tree",
            "treeitem" => "treeitem",
            _ => "",
        }
    }
    
    /// Get ARIA describedby for element descriptions
    pub fn describedby(element_id: &str) -> String {
        format!("{}-description", element_id)
    }
    
    /// Get ARIA labelledby for element labels
    pub fn labelledby(element_id: &str) -> String {
        format!("{}-label", element_id)
    }
    
    /// Get ARIA owns for parent-child relationships
    pub fn owns(children_ids: &[&str]) -> String {
        children_ids.join(" ")
    }
}

/// Screen reader specific utilities
pub struct ScreenReader;

impl ScreenReader {
    /// Get screen reader only text classes
    pub fn sr_only_classes() -> &'static str {
        "sr-only"
    }
    
    /// Get classes to hide from screen readers
    pub fn sr_hide_classes() -> &'static str {
        "aria-hidden"
    }
    
    /// Generate descriptive text for complex UI elements
    pub fn describe_button(button_type: &str, state: Option<&str>, icon: bool) -> String {
        let mut description = button_type.to_string();
        
        if icon {
            description.push_str(" button");
        }
        
        if let Some(state) = state {
            description.push_str(&format!(" - {}", state));
        }
        
        description
    }
    
    /// Generate descriptive text for form fields
    pub fn describe_form_field(field_type: &str, required: bool, error: Option<&str>) -> String {
        let mut description = field_type.to_string();
        
        if required {
            description.push_str(" - required");
        }
        
        if let Some(error) = error {
            description.push_str(&format!(" - error: {}", error));
        }
        
        description
    }
    
    /// Generate descriptive text for navigation
    pub fn describe_navigation(current_page: &str, total_pages: Option<i32>) -> String {
        if let Some(total) = total_pages {
            format!("Navigation - current page: {} of {}", current_page, total)
        } else {
            format!("Navigation - current page: {}", current_page)
        }
    }
    
    /// Generate descriptive text for modals
    pub fn describe_modal(title: &str, closable: bool) -> String {
        if closable {
            format!("Dialog: {} - press Escape to close", title)
        } else {
            format!("Dialog: {}", title)
        }
    }
}

/// Focus management for complex components
#[derive(Debug, Clone, PartialEq)]
pub enum FocusTarget {
    First,
    Last,
    Next,
    Previous,
    Index(usize),
    Element(String), // element ID
}

pub struct FocusManager;

impl FocusManager {
    /// Get focus trap classes for modals and dialogs
    pub fn focus_trap_classes() -> &'static str {
        "focus-trap"
    }
    
    /// Get focus within classes for containers
    pub fn focus_within_classes() -> &'static str {
        "focus-within:ring-2 focus-within:ring-forest-green-500"
    }
    
    /// Generate focus management attributes
    pub fn focus_attributes(target: FocusTarget) -> Vec<(&'static str, String)> {
        match target {
            FocusTarget::First => vec![
                ("tabindex", "0".to_string()),
                ("data-focus-target", "first".to_string()),
            ],
            FocusTarget::Last => vec![
                ("tabindex", "0".to_string()),
                ("data-focus-target", "last".to_string()),
            ],
            FocusTarget::Next => vec![
                ("tabindex", "-1".to_string()),
                ("data-focus-target", "next".to_string()),
            ],
            FocusTarget::Previous => vec![
                ("tabindex", "-1".to_string()),
                ("data-focus-target", "previous".to_string()),
            ],
            FocusTarget::Index(index) => vec![
                ("tabindex", if index == 0 { "0" } else { "-1" }.to_string()),
                ("data-focus-index", index.to_string()),
            ],
            FocusTarget::Element(id) => vec![
                ("tabindex", "-1".to_string()),
                ("data-focus-element", id),
            ],
        }
    }
}

/// Skip links for keyboard navigation
#[derive(Props, Clone, PartialEq)]
pub struct SkipLinksProps {
    #[props(default = vec![])]
    pub links: Vec<(String, String)>, // (text, target_id)
}

#[component]
pub fn SkipLinks(props: SkipLinksProps) -> Element {
    if props.links.is_empty() {
        return rsx! { div {} };
    }
    
    rsx! {
        div {
            class: "sr-only focus-within:not-sr-only fixed top-0 left-0 z-50 bg-forest-green-500 text-light-beige-200 p-2 rounded-br-md",
            ul {
                class: "list-none p-0 m-0",
                for (text, target) in props.links.iter() {
                    li {
                        a {
                            href: format!("#{}", target),
                            class: "text-light-beige-200 underline hover:text-light-beige-100 focus:outline-none focus:ring-2 focus:ring-light-beige-300 p-1 rounded",
                            onclick: move |_| {
                                // Focus the target element
                                // In a real implementation, this would use web_sys to focus the element
                            },
                            "{text}"
                        }
                    }
                }
            }
        }
    }
}

/// Landmark regions for page structure
#[derive(Props, Clone, PartialEq)]
pub struct LandmarkProps {
    #[props(default = "region".to_string())]
    pub landmark_type: String, // "main", "navigation", "banner", "contentinfo", "complementary"
    #[props(default = None)]
    pub label: Option<String>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Landmark(props: LandmarkProps) -> Element {
    let role = AriaAttributes::role(&props.landmark_type);
    let tag = match props.landmark_type.as_str() {
        "main" => "main",
        "navigation" => "nav",
        "banner" => "header",
        "contentinfo" => "footer",
        "complementary" => "aside",
        _ => "section",
    };
    
    let aria_label = props.label.as_ref().map(|l| format!("aria-label=\"{}\"", l)).unwrap_or_default();
    
    match tag {
        "main" => rsx! {
            main {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
        "nav" => rsx! {
            nav {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
        "header" => rsx! {
            header {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
        "footer" => rsx! {
            footer {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
        "aside" => rsx! {
            aside {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
        _ => rsx! {
            section {
                class: props.class,
                role: role,
                "aria-label": props.label.clone(),
                {props.children}
            }
        },
    }
}

/// Live region for dynamic content announcements
#[derive(Props, Clone, PartialEq)]
pub struct LiveRegionProps {
    #[props(default = "polite".to_string())]
    pub politeness: String, // "polite", "assertive", "off"
    #[props(default = false)]
    pub atomic: bool,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn LiveRegion(props: LiveRegionProps) -> Element {
    let aria_live = AriaAttributes::live(&props.politeness);
    let aria_atomic = if props.atomic { "true" } else { "false" };
    
    rsx! {
        div {
            class: format!("{} sr-only", props.class),
            "aria-live": aria_live,
            "aria-atomic": aria_atomic,
            role: "status",
            {props.children}
        }
    }
}

/// Accessible form field wrapper
#[derive(Props, Clone, PartialEq)]
pub struct AccessibleFieldProps {
    pub field_id: String,
    pub label: String,
    #[props(default = None)]
    pub description: Option<String>,
    #[props(default = None)]
    pub error: Option<String>,
    #[props(default = false)]
    pub required: bool,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AccessibleField(props: AccessibleFieldProps) -> Element {
    let description_id = format!("{}-description", props.field_id);
    let error_id = format!("{}-error", props.field_id);
    let label_id = format!("{}-label", props.field_id);
    
    let mut aria_describedby = Vec::new();
    if props.description.is_some() {
        aria_describedby.push(description_id.clone());
    }
    if props.error.is_some() {
        aria_describedby.push(error_id.clone());
    }
    
    rsx! {
        div {
            class: format!("form-field {}", props.class),
            label {
                id: label_id,
                r#for: props.field_id.clone(),
                class: "block text-sm font-medium text-light-beige-300 mb-2",
                "{props.label}"
                if props.required {
                    span {
                        class: "text-vibrant-orange-400 ml-1",
                        "aria-label": "required",
                        "*"
                    }
                }
            }
            
            div {
                class: "relative",
                "aria-describedby": aria_describedby.join(" "),
                {props.children}
            }
            
            if let Some(description) = props.description {
                div {
                    id: description_id,
                    class: "mt-1 text-sm text-light-beige-500",
                    "{description}"
                }
            }
            
            if let Some(error) = props.error {
                div {
                    id: error_id,
                    class: "mt-1 text-sm text-vibrant-orange-400",
                    role: "alert",
                    "aria-live": "polite",
                    "{error}"
                }
            }
        }
    }
}

/// Accessible button with comprehensive ARIA support
#[derive(Props, Clone, PartialEq)]
pub struct AccessibleButtonProps {
    #[props(default = None)]
    pub aria_label: Option<String>,
    #[props(default = None)]
    pub aria_describedby: Option<String>,
    #[props(default = false)]
    pub aria_expanded: bool,
    #[props(default = false)]
    pub aria_pressed: bool,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = None)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = None)]
    pub onkeydown: Option<EventHandler<KeyboardEvent>>,
    #[props(default = "".to_string())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn AccessibleButton(props: AccessibleButtonProps) -> Element {
    let handle_keydown = move |evt: KeyboardEvent| {
        if KeyboardNav::is_activation_key(&evt.key().to_string()) {
            evt.prevent_default();
            // In a real implementation, we would simulate a click event
        }
        
        if let Some(handler) = &props.onkeydown {
            handler.call(evt);
        }
    };
    
    rsx! {
        button {
            class: format!("focus:outline-none focus:ring-2 focus:ring-forest-green-500 focus:ring-offset-2 {}", props.class),
            disabled: props.disabled,
            "aria-label": props.aria_label.clone(),
            "aria-describedby": props.aria_describedby.clone(),
            "aria-expanded": AriaAttributes::expanded(props.aria_expanded),
            "aria-pressed": AriaAttributes::selected(props.aria_pressed),
            onclick: move |evt| {
                if !props.disabled {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                }
            },
            onkeydown: handle_keydown,
            {props.children}
        }
    }
}