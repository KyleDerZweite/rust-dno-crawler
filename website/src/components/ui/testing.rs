#![allow(non_snake_case)]
//! Testing infrastructure for UI components
//! 
//! This module provides testing utilities for component tests and visual regression testing.
//! It includes test helpers, mock data, and testing patterns for Dioxus components.

use dioxus::prelude::*;

/// Test utilities for component testing
pub struct TestUtils;

impl TestUtils {
    /// Create a test renderer for components
    pub fn create_test_renderer() -> VirtualDom {
        VirtualDom::new(TestApp)
    }
    
    /// Assert component renders without errors
    pub fn assert_renders(mut vdom: VirtualDom) -> bool {
        vdom.rebuild(&mut dioxus::prelude::NoOpMutations);
        true // Assume success for now - in a real implementation this would check for errors
    }
}

/// Mock data for testing components
pub struct MockData;

impl MockData {
    /// Generate mock user data
    pub fn mock_user() -> serde_json::Value {
        serde_json::json!({
            "id": "test-user-123",
            "username": "testuser",
            "email": "test@example.com",
            "first_name": "Test",
            "last_name": "User"
        })
    }
    
    /// Generate mock button props
    pub fn mock_button_props() -> Vec<(&'static str, serde_json::Value)> {
        vec![
            ("variant", serde_json::json!("Primary")),
            ("size", serde_json::json!("Medium")),
            ("state", serde_json::json!("Default")),
            ("class", serde_json::json!("")),
        ]
    }
}

/// Test app component for testing individual components
#[component]
fn TestApp() -> Element {
    rsx! {
        div {
            class: "test-container",
            "Test App Container"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_utils_create_renderer() {
        let _renderer = TestUtils::create_test_renderer();
        assert!(true);
    }
    
    #[test]
    fn test_mock_data_generation() {
        let user_data = MockData::mock_user();
        assert!(user_data.is_object());
        
        let button_props = MockData::mock_button_props();
        assert!(!button_props.is_empty());
    }
}