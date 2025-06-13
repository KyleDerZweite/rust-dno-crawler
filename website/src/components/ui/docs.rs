#![allow(non_snake_case)]

/// Component documentation system
/// 
/// This module provides comprehensive documentation for all UI components,
/// including usage examples, prop descriptions, and best practices.

use std::collections::HashMap;

/// Documentation for a component property
#[derive(Debug, Clone)]
pub struct PropDoc {
    pub name: String,
    pub prop_type: String,
    pub description: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub examples: Vec<String>,
}

/// Documentation for a component variant or enum
#[derive(Debug, Clone)]
pub struct VariantDoc {
    pub name: String,
    pub description: String,
    pub use_case: String,
    pub example: String,
}

/// Complete documentation for a UI component
#[derive(Debug, Clone)]
pub struct ComponentDoc {
    pub name: String,
    pub description: String,
    pub category: String,
    pub accessibility_notes: Vec<String>,
    pub props: Vec<PropDoc>,
    pub variants: HashMap<String, Vec<VariantDoc>>,
    pub examples: Vec<ComponentExample>,
    pub best_practices: Vec<String>,
    pub related_components: Vec<String>,
}

/// Usage example for a component
#[derive(Debug, Clone)]
pub struct ComponentExample {
    pub title: String,
    pub description: String,
    pub code: String,
    pub use_case: String,
}

/// Get complete documentation for all UI components
pub fn get_component_docs() -> HashMap<String, ComponentDoc> {
    let mut docs = HashMap::new();
    
    // Button Component Documentation
    docs.insert("Button".to_string(), ComponentDoc {
        name: "Button".to_string(),
        description: "A versatile button component with multiple variants, sizes, and states. Supports both button and link functionality with comprehensive accessibility features.".to_string(),
        category: "Interactive".to_string(),
        accessibility_notes: vec![
            "Automatically includes ARIA labels for different states".to_string(),
            "Supports keyboard navigation with proper focus indicators".to_string(),
            "Loading state is announced to screen readers".to_string(),
            "Disabled state prevents interaction and is properly announced".to_string(),
        ],
        props: vec![
            PropDoc {
                name: "variant".to_string(),
                prop_type: "ButtonVariant".to_string(),
                description: "Visual style variant of the button".to_string(),
                default_value: Some("Primary".to_string()),
                required: false,
                examples: vec!["Primary".to_string(), "Secondary".to_string(), "Ghost".to_string()],
            },
            PropDoc {
                name: "size".to_string(),
                prop_type: "ButtonSize".to_string(),
                description: "Size of the button affecting padding and text size".to_string(),
                default_value: Some("Medium".to_string()),
                required: false,
                examples: vec!["Small".to_string(), "Medium".to_string(), "Large".to_string(), "ExtraLarge".to_string()],
            },
            PropDoc {
                name: "state".to_string(),
                prop_type: "ButtonState".to_string(),
                description: "Interactive state of the button".to_string(),
                default_value: Some("Default".to_string()),
                required: false,
                examples: vec!["Default".to_string(), "Loading".to_string(), "Disabled".to_string()],
            },
            PropDoc {
                name: "onclick".to_string(),
                prop_type: "Option<EventHandler<MouseEvent>>".to_string(),
                description: "Click event handler".to_string(),
                default_value: Some("None".to_string()),
                required: false,
                examples: vec!["move |_| { /* handle click */ }".to_string()],
            },
            PropDoc {
                name: "href".to_string(),
                prop_type: "Option<String>".to_string(),
                description: "When provided, renders as a link instead of button".to_string(),
                default_value: Some("None".to_string()),
                required: false,
                examples: vec!["\"/dashboard\"".to_string(), "\"https://example.com\"".to_string()],
            },
        ],
        variants: {
            let mut variants = HashMap::new();
            variants.insert("ButtonVariant".to_string(), vec![
                VariantDoc {
                    name: "Primary".to_string(),
                    description: "Main call-to-action button with forest-green background".to_string(),
                    use_case: "Primary actions like submit, save, or continue".to_string(),
                    example: "variant: ButtonVariant::Primary".to_string(),
                },
                VariantDoc {
                    name: "Secondary".to_string(),
                    description: "Secondary action button with vibrant-orange background".to_string(),
                    use_case: "Secondary actions like cancel or alternative options".to_string(),
                    example: "variant: ButtonVariant::Secondary".to_string(),
                },
                VariantDoc {
                    name: "Ghost".to_string(),
                    description: "Transparent button with hover background".to_string(),
                    use_case: "Subtle actions or navigation links".to_string(),
                    example: "variant: ButtonVariant::Ghost".to_string(),
                },
            ]);
            variants
        },
        examples: vec![
            ComponentExample {
                title: "Basic Button".to_string(),
                description: "A simple primary button".to_string(),
                code: r#"PrimaryButton { "Save Changes" }"#.to_string(),
                use_case: "Most common button usage".to_string(),
            },
            ComponentExample {
                title: "Loading Button".to_string(),
                description: "Button showing loading state".to_string(),
                code: r#"PrimaryButton { 
    state: ButtonState::Loading, 
    "Saving..." 
}"#.to_string(),
                use_case: "During async operations".to_string(),
            },
        ],
        best_practices: vec![
            "Use Primary variant for the most important action on a page".to_string(),
            "Limit to one Primary button per section to maintain hierarchy".to_string(),
            "Use descriptive text that clearly indicates the action".to_string(),
            "Show loading state during async operations".to_string(),
            "Use appropriate sizes - Large for main CTAs, Small for compact interfaces".to_string(),
        ],
        related_components: vec!["Link".to_string(), "IconButton".to_string()],
    });
    
    // Card Component Documentation
    docs.insert("Card".to_string(), ComponentDoc {
        name: "Card".to_string(),
        description: "A flexible container component with multiple visual variants and consistent spacing. Perfect for grouping related content.".to_string(),
        category: "Layout".to_string(),
        accessibility_notes: vec![
            "Uses semantic HTML structure for better screen reader support".to_string(),
            "Supports keyboard navigation when interactive".to_string(),
            "Proper heading hierarchy with CardTitle component".to_string(),
        ],
        props: vec![
            PropDoc {
                name: "variant".to_string(),
                prop_type: "CardVariant".to_string(),
                description: "Visual style variant of the card".to_string(),
                default_value: Some("Default".to_string()),
                required: false,
                examples: vec!["Default".to_string(), "Bordered".to_string(), "Elevated".to_string(), "Glass".to_string()],
            },
            PropDoc {
                name: "padding".to_string(),
                prop_type: "CardPadding".to_string(),
                description: "Internal padding of the card content".to_string(),
                default_value: Some("Medium".to_string()),
                required: false,
                examples: vec!["None".to_string(), "Small".to_string(), "Medium".to_string(), "Large".to_string()],
            },
            PropDoc {
                name: "onclick".to_string(),
                prop_type: "Option<EventHandler<MouseEvent>>".to_string(),
                description: "Makes the card interactive when provided".to_string(),
                default_value: Some("None".to_string()),
                required: false,
                examples: vec!["move |_| { /* handle click */ }".to_string()],
            },
        ],
        variants: HashMap::new(),
        examples: vec![
            ComponentExample {
                title: "Basic Card".to_string(),
                description: "Simple card with header and content".to_string(),
                code: r#"Card {
    CardHeader {
        CardTitle { "Card Title" }
    }
    CardContent {
        "Card content goes here"
    }
}"#.to_string(),
                use_case: "Most common card pattern".to_string(),
            },
        ],
        best_practices: vec![
            "Use consistent card variants across similar content".to_string(),
            "Include CardTitle for better structure and accessibility".to_string(),
            "Use appropriate padding based on content density".to_string(),
            "Glass variant works well for overlay content".to_string(),
        ],
        related_components: vec!["Modal".to_string(), "Container".to_string()],
    });
    
    docs
}

/// Get documentation for a specific component
pub fn get_component_doc(component_name: &str) -> Option<ComponentDoc> {
    get_component_docs().get(component_name).cloned()
}

/// Generate markdown documentation for a component
pub fn generate_markdown_docs(component: &ComponentDoc) -> String {
    let mut md = String::new();
    
    md.push_str(&format!("# {}\n\n", component.name));
    md.push_str(&format!("{}\n\n", component.description));
    md.push_str(&format!("**Category:** {}\n\n", component.category));
    
    // Accessibility
    if !component.accessibility_notes.is_empty() {
        md.push_str("## Accessibility\n\n");
        for note in &component.accessibility_notes {
            md.push_str(&format!("- {}\n", note));
        }
        md.push_str("\n");
    }
    
    // Props
    if !component.props.is_empty() {
        md.push_str("## Props\n\n");
        md.push_str("| Name | Type | Required | Default | Description |\n");
        md.push_str("|------|------|----------|---------|-------------|\n");
        for prop in &component.props {
            md.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                prop.name,
                prop.prop_type,
                if prop.required { "✓" } else { " " },
                prop.default_value.as_deref().unwrap_or("—"),
                prop.description
            ));
        }
        md.push_str("\n");
    }
    
    // Examples
    if !component.examples.is_empty() {
        md.push_str("## Examples\n\n");
        for example in &component.examples {
            md.push_str(&format!("### {}\n\n", example.title));
            md.push_str(&format!("{}\n\n", example.description));
            md.push_str("```rust\n");
            md.push_str(&example.code);
            md.push_str("\n```\n\n");
        }
    }
    
    // Best Practices
    if !component.best_practices.is_empty() {
        md.push_str("## Best Practices\n\n");
        for practice in &component.best_practices {
            md.push_str(&format!("- {}\n", practice));
        }
        md.push_str("\n");
    }
    
    md
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_docs_available() {
        let docs = get_component_docs();
        assert!(docs.contains_key("Button"));
        assert!(docs.contains_key("Card"));
    }
    
    #[test]
    fn test_get_specific_component_doc() {
        let button_doc = get_component_doc("Button");
        assert!(button_doc.is_some());
        assert_eq!(button_doc.unwrap().name, "Button");
    }
    
    #[test]
    fn test_markdown_generation() {
        let docs = get_component_docs();
        let button_doc = docs.get("Button").unwrap();
        let markdown = generate_markdown_docs(button_doc);
        assert!(markdown.contains("# Button"));
        assert!(markdown.contains("## Props"));
    }
}