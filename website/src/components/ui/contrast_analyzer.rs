#![allow(non_snake_case)]

/// Contrast analysis and validation for WCAG compliance
/// 
/// This module provides tools to analyze and validate color contrast ratios
/// for our custom color palette to ensure WCAG AA/AAA compliance.

use std::collections::HashMap;

/// RGB color representation for contrast calculations
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Create color from hex string (e.g., "#FF0000" or "FF0000")
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color format");
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;
        
        Ok(Self::new(r, g, b))
    }
    
    /// Calculate relative luminance according to WCAG 2.1
    pub fn relative_luminance(&self) -> f64 {
        fn linearize(component: u8) -> f64 {
            let c = component as f64 / 255.0;
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        
        0.2126 * linearize(self.r) + 0.7152 * linearize(self.g) + 0.0722 * linearize(self.b)
    }
}

/// Calculate contrast ratio between two colors according to WCAG 2.1
pub fn contrast_ratio(color1: Color, color2: Color) -> f64 {
    let l1 = color1.relative_luminance();
    let l2 = color2.relative_luminance();
    
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    
    (lighter + 0.05) / (darker + 0.05)
}

/// WCAG compliance levels
#[derive(Debug, Clone, PartialEq)]
pub enum WCAGLevel {
    /// WCAG AA normal text (4.5:1)
    AA,
    /// WCAG AA large text (3:1)
    AALarge,
    /// WCAG AAA normal text (7:1)
    AAA,
    /// WCAG AAA large text (4.5:1)
    AAALarge,
    /// Does not meet WCAG requirements
    Fail,
}

/// Evaluate WCAG compliance for a contrast ratio
pub fn evaluate_wcag_compliance(ratio: f64) -> Vec<WCAGLevel> {
    let mut levels = Vec::new();
    
    if ratio >= 7.0 {
        levels.push(WCAGLevel::AAA);
        levels.push(WCAGLevel::AA);
        levels.push(WCAGLevel::AAALarge);
        levels.push(WCAGLevel::AALarge);
    } else if ratio >= 4.5 {
        levels.push(WCAGLevel::AA);
        levels.push(WCAGLevel::AALarge);
        levels.push(WCAGLevel::AAALarge);
    } else if ratio >= 3.0 {
        levels.push(WCAGLevel::AALarge);
    }
    
    if levels.is_empty() {
        levels.push(WCAGLevel::Fail);
    }
    
    levels
}

/// Our custom color palette with hex values
pub fn get_custom_palette() -> HashMap<&'static str, HashMap<u16, &'static str>> {
    let mut palette = HashMap::new();
    
    // Dark Charcoal
    let mut dark_charcoal = HashMap::new();
    dark_charcoal.insert(100, "#4A4A4A");
    dark_charcoal.insert(200, "#3D3D3D");
    dark_charcoal.insert(300, "#303030");
    dark_charcoal.insert(400, "#282828");
    dark_charcoal.insert(500, "#1C1C1C");
    dark_charcoal.insert(600, "#171717");
    dark_charcoal.insert(700, "#121212");
    dark_charcoal.insert(800, "#0D0D0D");
    dark_charcoal.insert(900, "#080808");
    palette.insert("dark-charcoal", dark_charcoal);
    
    // Forest Green
    let mut forest_green = HashMap::new();
    forest_green.insert(100, "#AEC1B1");
    forest_green.insert(200, "#93AD99");
    forest_green.insert(300, "#789982");
    forest_green.insert(400, "#5D856B");
    forest_green.insert(500, "#3B533E");
    forest_green.insert(600, "#324736");
    forest_green.insert(700, "#293B2E");
    forest_green.insert(800, "#202F26");
    forest_green.insert(900, "#17231E");
    palette.insert("forest-green", forest_green);
    
    // Light Beige
    let mut light_beige = HashMap::new();
    light_beige.insert(100, "#FFFFFF");
    light_beige.insert(200, "#F7F1EB");
    light_beige.insert(300, "#EEE7E1");
    light_beige.insert(400, "#E5DED7");
    light_beige.insert(500, "#DCC09B");
    light_beige.insert(600, "#C3A984");
    light_beige.insert(700, "#AA926D");
    light_beige.insert(800, "#917B56");
    light_beige.insert(900, "#78643F");
    palette.insert("light-beige", light_beige);
    
    // Amber Brown
    let mut amber_brown = HashMap::new();
    amber_brown.insert(100, "#F5D1B8");
    amber_brown.insert(200, "#EBB394");
    amber_brown.insert(300, "#E19570");
    amber_brown.insert(400, "#D7774C");
    amber_brown.insert(500, "#C37D57");
    amber_brown.insert(600, "#B06845");
    amber_brown.insert(700, "#9D5333");
    amber_brown.insert(800, "#8A3E21");
    amber_brown.insert(900, "#77290F");
    palette.insert("amber-brown", amber_brown);
    
    // Sienna Brown
    let mut sienna_brown = HashMap::new();
    sienna_brown.insert(100, "#E1B89B");
    sienna_brown.insert(200, "#C99E82");
    sienna_brown.insert(300, "#B18469");
    sienna_brown.insert(400, "#996A50");
    sienna_brown.insert(500, "#8B5C3D");
    sienna_brown.insert(600, "#784B30");
    sienna_brown.insert(700, "#653A23");
    sienna_brown.insert(800, "#522916");
    sienna_brown.insert(900, "#3F1809");
    palette.insert("sienna-brown", sienna_brown);
    
    // Muted Olive
    let mut muted_olive = HashMap::new();
    muted_olive.insert(100, "#C5D1BD");
    muted_olive.insert(200, "#A9BBA3");
    muted_olive.insert(300, "#8EAA89");
    muted_olive.insert(400, "#739470");
    muted_olive.insert(500, "#6D7C66");
    muted_olive.insert(600, "#5A6A54");
    muted_olive.insert(700, "#475842");
    muted_olive.insert(800, "#344630");
    muted_olive.insert(900, "#21341E");
    palette.insert("muted-olive", muted_olive);
    
    palette
}

/// Analyze contrast ratios for our color combinations
pub fn analyze_color_combinations() -> Vec<ContrastAnalysis> {
    let palette = get_custom_palette();
    let mut analyses = Vec::new();
    
    // Primary text combinations (light-beige on dark-charcoal)
    if let (Some(light_beige), Some(dark_charcoal)) = (palette.get("light-beige"), palette.get("dark-charcoal")) {
        for (text_shade, text_hex) in light_beige {
            for (bg_shade, bg_hex) in dark_charcoal {
                if let (Ok(text_color), Ok(bg_color)) = (Color::from_hex(text_hex), Color::from_hex(bg_hex)) {
                    let ratio = contrast_ratio(text_color, bg_color);
                    let compliance = evaluate_wcag_compliance(ratio);
                    
                    analyses.push(ContrastAnalysis {
                        description: format!("light-beige-{} on dark-charcoal-{}", text_shade, bg_shade),
                        text_color: text_color,
                        background_color: bg_color,
                        contrast_ratio: ratio,
                        wcag_levels: compliance,
                        recommended: ratio >= 4.5,
                    });
                }
            }
        }
    }
    
    // Primary button combinations (light-beige on forest-green)
    if let (Some(light_beige), Some(forest_green)) = (palette.get("light-beige"), palette.get("forest-green")) {
        for (text_shade, text_hex) in light_beige {
            for (bg_shade, bg_hex) in forest_green {
                if let (Ok(text_color), Ok(bg_color)) = (Color::from_hex(text_hex), Color::from_hex(bg_hex)) {
                    let ratio = contrast_ratio(text_color, bg_color);
                    let compliance = evaluate_wcag_compliance(ratio);
                    
                    analyses.push(ContrastAnalysis {
                        description: format!("light-beige-{} on forest-green-{}", text_shade, bg_shade),
                        text_color: text_color,
                        background_color: bg_color,
                        contrast_ratio: ratio,
                        wcag_levels: compliance,
                        recommended: ratio >= 4.5,
                    });
                }
            }
        }
    }
    
    analyses
}

/// Results of contrast analysis
#[derive(Debug)]
pub struct ContrastAnalysis {
    pub description: String,
    pub text_color: Color,
    pub background_color: Color,
    pub contrast_ratio: f64,
    pub wcag_levels: Vec<WCAGLevel>,
    pub recommended: bool,
}

/// Get recommended color combinations that meet WCAG AA standards
pub fn get_recommended_combinations() -> Vec<RecommendedCombination> {
    let analyses = analyze_color_combinations();
    let mut recommendations = Vec::new();
    
    for analysis in analyses {
        if analysis.recommended && analysis.wcag_levels.contains(&WCAGLevel::AA) {
            let use_case = determine_use_case(&analysis.description);
            recommendations.push(RecommendedCombination {
                description: analysis.description,
                contrast_ratio: analysis.contrast_ratio,
                use_case,
            });
        }
    }
    
    recommendations.sort_by(|a, b| b.contrast_ratio.partial_cmp(&a.contrast_ratio).unwrap());
    recommendations
}

/// Recommended color combination
#[derive(Debug)]
pub struct RecommendedCombination {
    pub description: String,
    pub contrast_ratio: f64,
    pub use_case: String,
}

/// Determine appropriate use case for a color combination
fn determine_use_case(description: &str) -> String {
    if description.contains("dark-charcoal") {
        if description.contains("light-beige-200") || description.contains("light-beige-300") {
            "Primary text on backgrounds".to_string()
        } else if description.contains("light-beige-400") || description.contains("light-beige-500") {
            "Secondary text on backgrounds".to_string()
        } else {
            "Background combinations".to_string()
        }
    } else if description.contains("forest-green") {
        "Button text on primary buttons".to_string()
    } else {
        "General use".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#FF0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }
    
    #[test]
    fn test_contrast_ratio() {
        let white = Color::new(255, 255, 255);
        let black = Color::new(0, 0, 0);
        let ratio = contrast_ratio(white, black);
        assert!((ratio - 21.0).abs() < 0.1); // Should be exactly 21:1
    }
    
    #[test]
    fn test_wcag_compliance() {
        let levels = evaluate_wcag_compliance(7.5);
        assert!(levels.contains(&WCAGLevel::AAA));
        assert!(levels.contains(&WCAGLevel::AA));
        
        let levels = evaluate_wcag_compliance(4.5);
        assert!(levels.contains(&WCAGLevel::AA));
        assert!(!levels.contains(&WCAGLevel::AAA));
        
        let levels = evaluate_wcag_compliance(2.0);
        assert!(levels.contains(&WCAGLevel::Fail));
    }
    
    #[test]
    fn test_custom_palette_contrast_ratios() {
        let recommendations = get_recommended_combinations();
        
        // Should have some valid combinations
        assert!(!recommendations.is_empty());
        
        // All recommended combinations should meet WCAG AA
        for rec in &recommendations {
            assert!(rec.contrast_ratio >= 4.5, "Combination '{}' has ratio {}, below WCAG AA", rec.description, rec.contrast_ratio);
        }
        
        // Print out some key recommendations for verification
        println!("Top 5 contrast combinations:");
        for (i, rec) in recommendations.iter().take(5).enumerate() {
            println!("{}. {} - Ratio: {:.2} - Use: {}", i+1, rec.description, rec.contrast_ratio, rec.use_case);
        }
    }
}