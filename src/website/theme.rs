#![allow(non_snake_case)]
use dioxus::prelude::*;
use web_sys::{window, Storage};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

// Global theme context
pub fn use_theme() -> Signal<Theme> {
    use_context()
}

pub fn provide_theme() -> Signal<Theme> {
    let theme = use_signal(|| {
        // Try to get theme from localStorage, default to light
        get_stored_theme().unwrap_or(Theme::Light)
    });

    use_context_provider(|| theme);

    // Apply theme to document element whenever it changes
    use_effect(move || {
        let current_theme = theme();
        apply_theme_to_document(current_theme);
        store_theme(current_theme);
    });

    theme
}

fn get_stored_theme() -> Option<Theme> {
    let window = window()?;
    let storage = window.local_storage().ok()??;
    let theme_str = storage.get_item("theme").ok()??;
    Some(Theme::from_str(&theme_str))
}

fn store_theme(theme: Theme) {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", theme.as_str());
        }
    }
}

fn apply_theme_to_document(theme: Theme) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(html_element) = document.document_element() {
                let class_list = html_element.class_list();

                // Remove both classes first
                let _ = class_list.remove_1("light");
                let _ = class_list.remove_1("dark");

                // Add the current theme class
                let _ = class_list.add_1(theme.as_str());
            }
        }
    }
}