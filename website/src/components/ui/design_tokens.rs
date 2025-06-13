#![allow(non_snake_case)]

/// Design tokens and color usage guidelines
/// 
/// This module provides comprehensive documentation for the design system,
/// including color palettes, spacing, typography, and usage guidelines.

use std::collections::HashMap;

/// Color token with comprehensive usage information
#[derive(Debug, Clone)]
pub struct ColorToken {
    pub name: String,
    pub hex_value: String,
    pub description: String,
    pub usage: Vec<String>,
    pub accessibility_notes: Vec<String>,
    pub contrast_ratios: HashMap<String, f64>,
}

/// Spacing token with size and usage information
#[derive(Debug, Clone)]
pub struct SpacingToken {
    pub name: String,
    pub value: String,
    pub pixel_equivalent: u16,
    pub usage: Vec<String>,
}

/// Typography token with font information
#[derive(Debug, Clone)]
pub struct TypographyToken {
    pub name: String,
    pub size: String,
    pub line_height: String,
    pub font_weight: String,
    pub usage: Vec<String>,
}

/// Design system documentation
#[derive(Debug, Clone)]
pub struct DesignTokens {
    pub color_palette: HashMap<String, Vec<ColorToken>>,
    pub spacing_scale: Vec<SpacingToken>,
    pub typography_scale: Vec<TypographyToken>,
    pub breakpoints: HashMap<String, String>,
    pub shadows: HashMap<String, String>,
    pub border_radius: HashMap<String, String>,
}

/// Get complete design tokens documentation
pub fn get_design_tokens() -> DesignTokens {
    DesignTokens {
        color_palette: get_color_palette(),
        spacing_scale: get_spacing_scale(),
        typography_scale: get_typography_scale(),
        breakpoints: get_breakpoints(),
        shadows: get_shadows(),
        border_radius: get_border_radius(),
    }
}

/// Get comprehensive color palette with usage guidelines
pub fn get_color_palette() -> HashMap<String, Vec<ColorToken>> {
    let mut palette = HashMap::new();
    
    // Dark Charcoal - Primary backgrounds
    palette.insert("dark-charcoal".to_string(), vec![
        ColorToken {
            name: "dark-charcoal-100".to_string(),
            hex_value: "#4A4A4A".to_string(),
            description: "Lightest charcoal for subtle variations".to_string(),
            usage: vec![
                "Light text on dark-charcoal-500+ backgrounds".to_string(),
                "Subtle border accents".to_string(),
                "Icon colors on dark backgrounds".to_string(),
            ],
            accessibility_notes: vec![
                "Use with caution as primary text color".to_string(),
                "Best for decorative elements".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-200".to_string(), 3.2),
                ("light-beige-300".to_string(), 2.8),
            ]),
        },
        ColorToken {
            name: "dark-charcoal-500".to_string(),
            hex_value: "#1C1C1C".to_string(),
            description: "Base dark charcoal for main sections".to_string(),
            usage: vec![
                "Card backgrounds".to_string(),
                "Modal backgrounds".to_string(),
                "Input field backgrounds".to_string(),
                "Secondary page sections".to_string(),
            ],
            accessibility_notes: vec![
                "Excellent contrast with light-beige colors".to_string(),
                "WCAG AAA compliant with light-beige-200+".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-200".to_string(), 17.87),
                ("light-beige-300".to_string(), 15.42),
                ("light-beige-400".to_string(), 13.24),
            ]),
        },
        ColorToken {
            name: "dark-charcoal-800".to_string(),
            hex_value: "#0D0D0D".to_string(),
            description: "Primary page background".to_string(),
            usage: vec![
                "Main page background".to_string(),
                "Navigation backgrounds".to_string(),
                "Deep section backgrounds".to_string(),
            ],
            accessibility_notes: vec![
                "Maximum contrast with light colors".to_string(),
                "Perfect for main page background".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-200".to_string(), 19.44),
                ("light-beige-300".to_string(), 18.73),
            ]),
        },
    ]);
    
    // Forest Green - Primary accents
    palette.insert("forest-green".to_string(), vec![
        ColorToken {
            name: "forest-green-400".to_string(),
            hex_value: "#5D856B".to_string(),
            description: "Lighter primary accent for enhanced contrast".to_string(),
            usage: vec![
                "High contrast primary buttons".to_string(),
                "Link colors in enhanced contrast mode".to_string(),
                "Success state indicators".to_string(),
            ],
            accessibility_notes: vec![
                "Better contrast than forest-green-500 for accessibility".to_string(),
                "Use for high contrast themes".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-100".to_string(), 7.2),
                ("dark-charcoal-800".to_string(), 2.9),
            ]),
        },
        ColorToken {
            name: "forest-green-500".to_string(),
            hex_value: "#3B533E".to_string(),
            description: "Base primary accent color".to_string(),
            usage: vec![
                "Primary buttons".to_string(),
                "Primary icons".to_string(),
                "Active states".to_string(),
                "Focus indicators".to_string(),
                "Primary CTAs".to_string(),
            ],
            accessibility_notes: vec![
                "WCAG AA compliant with light-beige-200+".to_string(),
                "Primary brand color for interactive elements".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-200".to_string(), 5.8),
                ("light-beige-300".to_string(), 4.9),
            ]),
        },
    ]);
    
    // Light Beige - Text and highlights
    palette.insert("light-beige".to_string(), vec![
        ColorToken {
            name: "light-beige-200".to_string(),
            hex_value: "#F7F1EB".to_string(),
            description: "High contrast text for headings".to_string(),
            usage: vec![
                "Primary headings".to_string(),
                "Button text on colored backgrounds".to_string(),
                "High importance content".to_string(),
                "Enhanced contrast mode text".to_string(),
            ],
            accessibility_notes: vec![
                "Maximum contrast on dark backgrounds".to_string(),
                "WCAG AAA compliant on dark-charcoal-500+".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("dark-charcoal-500".to_string(), 17.87),
                ("dark-charcoal-800".to_string(), 19.44),
                ("forest-green-500".to_string(), 5.8),
            ]),
        },
        ColorToken {
            name: "light-beige-300".to_string(),
            hex_value: "#EEE7E1".to_string(),
            description: "Primary readable text".to_string(),
            usage: vec![
                "Main content text".to_string(),
                "Card content".to_string(),
                "Form labels".to_string(),
                "Navigation text".to_string(),
            ],
            accessibility_notes: vec![
                "Excellent readability on dark backgrounds".to_string(),
                "WCAG AA+ compliant on most dark surfaces".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("dark-charcoal-500".to_string(), 15.42),
                ("dark-charcoal-800".to_string(), 18.73),
            ]),
        },
        ColorToken {
            name: "light-beige-500".to_string(),
            hex_value: "#DCC09B".to_string(),
            description: "Secondary text and subtle accents".to_string(),
            usage: vec![
                "Secondary text".to_string(),
                "Placeholder text".to_string(),
                "Subtle button text".to_string(),
                "Disabled state text".to_string(),
            ],
            accessibility_notes: vec![
                "Use for less critical text content".to_string(),
                "Still maintains good contrast on dark backgrounds".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("dark-charcoal-500".to_string(), 8.9),
                ("dark-charcoal-800".to_string(), 10.2),
            ]),
        },
    ]);
    
    // Amber Brown - Secondary accents
    palette.insert("vibrant-orange".to_string(), vec![
        ColorToken {
            name: "vibrant-orange-500".to_string(),
            hex_value: "#C37D57".to_string(),
            description: "Secondary accent for warm elements".to_string(),
            usage: vec![
                "Secondary buttons".to_string(),
                "Badge backgrounds".to_string(),
                "Warning indicators".to_string(),
                "Warm accent elements".to_string(),
            ],
            accessibility_notes: vec![
                "Good contrast with light text".to_string(),
                "Warm alternative to forest-green".to_string(),
            ],
            contrast_ratios: HashMap::from([
                ("light-beige-200".to_string(), 4.2),
                ("dark-charcoal-800".to_string(), 4.6),
            ]),
        },
    ]);
    
    palette
}

/// Get spacing scale tokens
pub fn get_spacing_scale() -> Vec<SpacingToken> {
    vec![
        SpacingToken {
            name: "xs".to_string(),
            value: "0.25rem".to_string(),
            pixel_equivalent: 4,
            usage: vec!["Small gaps between related elements".to_string()],
        },
        SpacingToken {
            name: "sm".to_string(),
            value: "0.5rem".to_string(),
            pixel_equivalent: 8,
            usage: vec!["Icon margins", "Form field gaps"].into_iter().map(|s| s.to_string()).collect(),
        },
        SpacingToken {
            name: "md".to_string(),
            value: "1rem".to_string(),
            pixel_equivalent: 16,
            usage: vec!["Standard component spacing".to_string(), "Card padding base unit".to_string()],
        },
        SpacingToken {
            name: "lg".to_string(),
            value: "1.5rem".to_string(),
            pixel_equivalent: 24,
            usage: vec!["Section spacing".to_string(), "Card padding".to_string()],
        },
        SpacingToken {
            name: "xl".to_string(),
            value: "2rem".to_string(),
            pixel_equivalent: 32,
            usage: vec!["Page section margins".to_string(), "Large card padding".to_string()],
        },
        SpacingToken {
            name: "2xl".to_string(),
            value: "3rem".to_string(),
            pixel_equivalent: 48,
            usage: vec!["Page-level spacing".to_string(), "Hero section padding".to_string()],
        },
    ]
}

/// Get typography scale tokens
pub fn get_typography_scale() -> Vec<TypographyToken> {
    vec![
        TypographyToken {
            name: "text-sm".to_string(),
            size: "0.875rem".to_string(),
            line_height: "1.25rem".to_string(),
            font_weight: "400".to_string(),
            usage: vec!["Small text".to_string(), "Captions".to_string(), "Desktop button text".to_string()],
        },
        TypographyToken {
            name: "text-base".to_string(),
            size: "1rem".to_string(),
            line_height: "1.5rem".to_string(),
            font_weight: "400".to_string(),
            usage: vec!["Body text".to_string(), "Mobile-first text".to_string(), "Default component text".to_string()],
        },
        TypographyToken {
            name: "text-lg".to_string(),
            size: "1.125rem".to_string(),
            line_height: "1.75rem".to_string(),
            font_weight: "400".to_string(),
            usage: vec!["Large body text".to_string(), "Mobile headings".to_string(), "Large buttons".to_string()],
        },
        TypographyToken {
            name: "text-xl".to_string(),
            size: "1.25rem".to_string(),
            line_height: "1.75rem".to_string(),
            font_weight: "500".to_string(),
            usage: vec!["Section headings".to_string(), "Card titles".to_string(), "Extra large buttons".to_string()],
        },
    ]
}

/// Get responsive breakpoints
pub fn get_breakpoints() -> HashMap<String, String> {
    HashMap::from([
        ("sm".to_string(), "640px".to_string()),
        ("md".to_string(), "768px".to_string()),
        ("lg".to_string(), "1024px".to_string()),
        ("xl".to_string(), "1280px".to_string()),
        ("2xl".to_string(), "1536px".to_string()),
    ])
}

/// Get shadow tokens
pub fn get_shadows() -> HashMap<String, String> {
    HashMap::from([
        ("sm".to_string(), "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string()),
        ("md".to_string(), "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string()),
        ("lg".to_string(), "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string()),
        ("xl".to_string(), "0 20px 25px -5px rgb(0 0 0 / 0.1)".to_string()),
        ("2xl".to_string(), "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string()),
    ])
}

/// Get border radius tokens
pub fn get_border_radius() -> HashMap<String, String> {
    HashMap::from([
        ("sm".to_string(), "0.125rem".to_string()),
        ("md".to_string(), "0.375rem".to_string()),
        ("lg".to_string(), "0.5rem".to_string()),
        ("xl".to_string(), "0.75rem".to_string()),
        ("2xl".to_string(), "1rem".to_string()),
    ])
}

/// Generate design system documentation as markdown
pub fn generate_design_tokens_markdown() -> String {
    let tokens = get_design_tokens();
    let mut md = String::new();
    
    md.push_str("# Design System Documentation\n\n");
    md.push_str("This document outlines the complete design system including colors, spacing, typography, and usage guidelines.\n\n");
    
    // Color Palette
    md.push_str("## Color Palette\n\n");
    md.push_str("Our color system is optimized for dark themes with high contrast ratios and accessibility compliance.\n\n");
    
    for (family_name, colors) in &tokens.color_palette {
        md.push_str(&format!("### {}\n\n", family_name));
        for color in colors {
            md.push_str(&format!("#### {}\n", color.name));
            md.push_str(&format!("**Hex:** `{}`\n\n", color.hex_value));
            md.push_str(&format!("{}\n\n", color.description));
            
            md.push_str("**Usage:**\n");
            for usage in &color.usage {
                md.push_str(&format!("- {}\n", usage));
            }
            md.push_str("\n");
            
            if !color.accessibility_notes.is_empty() {
                md.push_str("**Accessibility:**\n");
                for note in &color.accessibility_notes {
                    md.push_str(&format!("- {}\n", note));
                }
                md.push_str("\n");
            }
            
            if !color.contrast_ratios.is_empty() {
                md.push_str("**Contrast Ratios:**\n");
                for (bg, ratio) in &color.contrast_ratios {
                    let compliance = if *ratio >= 7.0 { " (AAA)" } else if *ratio >= 4.5 { " (AA)" } else { " (Fail)" };
                    md.push_str(&format!("- vs {}: {:.1}:1{}\n", bg, ratio, compliance));
                }
                md.push_str("\n");
            }
        }
    }
    
    // Typography
    md.push_str("## Typography Scale\n\n");
    md.push_str("| Class | Size | Line Height | Weight | Usage |\n");
    md.push_str("|-------|------|-------------|--------|-------|\n");
    for token in &tokens.typography_scale {
        md.push_str(&format!(
            "| `{}` | {} | {} | {} | {} |\n",
            token.name,
            token.size,
            token.line_height,
            token.font_weight,
            token.usage.join(", ")
        ));
    }
    md.push_str("\n");
    
    // Spacing
    md.push_str("## Spacing Scale\n\n");
    md.push_str("| Token | Value | Pixels | Usage |\n");
    md.push_str("|-------|-------|--------|-------|\n");
    for token in &tokens.spacing_scale {
        md.push_str(&format!(
            "| `{}` | {} | {}px | {} |\n",
            token.name,
            token.value,
            token.pixel_equivalent,
            token.usage.join(", ")
        ));
    }
    md.push_str("\n");
    
    // Mobile-First Guidelines
    md.push_str("## Mobile-First Design Guidelines\n\n");
    md.push_str("### Touch Targets\n");
    md.push_str("- Minimum 44px height for interactive elements\n");
    md.push_str("- Use `touch-manipulation` CSS property\n");
    md.push_str("- Larger padding on mobile, smaller on desktop\n\n");
    
    md.push_str("### Responsive Patterns\n");
    md.push_str("- Start with mobile styles, enhance for larger screens\n");
    md.push_str("- Use `text-base` (16px) on mobile, `text-sm` (14px) on desktop\n");
    md.push_str("- Larger spacing on mobile for easier touch interaction\n\n");
    
    md.push_str("### Breakpoints\n");
    md.push_str("| Breakpoint | Value | Description |\n");
    md.push_str("|------------|-------|-------------|\n");
    for (name, value) in &tokens.breakpoints {
        md.push_str(&format!("| `{}` | {} | {} and up |\n", name, value, name.to_uppercase()));
    }
    md.push_str("\n");
    
    md
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_design_tokens_structure() {
        let tokens = get_design_tokens();
        assert!(!tokens.color_palette.is_empty());
        assert!(!tokens.spacing_scale.is_empty());
        assert!(!tokens.typography_scale.is_empty());
    }
    
    #[test]
    fn test_color_palette_completeness() {
        let palette = get_color_palette();
        assert!(palette.contains_key("dark-charcoal"));
        assert!(palette.contains_key("forest-green"));
        assert!(palette.contains_key("light-beige"));
    }
    
    #[test]
    fn test_markdown_generation() {
        let markdown = generate_design_tokens_markdown();
        assert!(markdown.contains("# Design System Documentation"));
        assert!(markdown.contains("## Color Palette"));
        assert!(markdown.contains("## Typography Scale"));
    }
}