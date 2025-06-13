#![allow(non_snake_case)]

/// Theme system for the UI components
/// 
/// This module provides theme variants and accessibility enhancements for dark-optimized design.
/// All components use the custom color palette defined in tailwind.config.js.

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeVariant {
    /// Standard dark theme optimized for regular use
    Dark,
    /// High contrast dark theme for better accessibility
    HighContrast,
    /// Reduced motion variant for accessibility
    ReducedMotion,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContrastLevel {
    /// Standard contrast ratios (WCAG AA compliant)
    Standard,
    /// Enhanced contrast ratios (WCAG AAA compliant)
    Enhanced,
}

/// Theme context providing consistent theming across components
pub struct ThemeContext {
    pub variant: ThemeVariant,
    pub contrast: ContrastLevel,
    pub reduce_motion: bool,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            variant: ThemeVariant::Dark,
            contrast: ContrastLevel::Standard,
            reduce_motion: false,
        }
    }
}

/// Get theme-appropriate classes for text contrast
pub fn get_text_contrast_classes(level: ContrastLevel) -> &'static str {
    match level {
        ContrastLevel::Standard => "text-light-beige-300", // Standard readable text
        ContrastLevel::Enhanced => "text-light-beige-200", // Higher contrast text
    }
}

/// Get theme-appropriate classes for secondary text
pub fn get_secondary_text_classes(level: ContrastLevel) -> &'static str {
    match level {
        ContrastLevel::Standard => "text-light-beige-500", // Subtle secondary text
        ContrastLevel::Enhanced => "text-light-beige-400", // More visible secondary text
    }
}

/// Get theme-appropriate classes for background contrast
pub fn get_background_contrast_classes(level: ContrastLevel) -> &'static str {
    match level {
        ContrastLevel::Standard => "bg-dark-charcoal-500", // Standard background
        ContrastLevel::Enhanced => "bg-dark-charcoal-600", // Higher contrast background
    }
}

/// Get theme-appropriate classes for border contrast
pub fn get_border_contrast_classes(level: ContrastLevel) -> &'static str {
    match level {
        ContrastLevel::Standard => "border-dark-charcoal-300", // Subtle borders
        ContrastLevel::Enhanced => "border-dark-charcoal-200", // More visible borders
    }
}

/// Get animation classes based on motion preferences
pub fn get_animation_classes(reduce_motion: bool) -> &'static str {
    if reduce_motion {
        "transition-none" // Disable animations for accessibility
    } else {
        "transition-all duration-200" // Standard smooth animations
    }
}

/// Get focus ring classes for accessibility
pub fn get_focus_ring_classes() -> &'static str {
    "focus:outline-none focus:ring-2 focus:ring-forest-green-500 focus:ring-offset-2 focus:ring-offset-dark-charcoal-800"
}

/// Color contrast ratios for WCAG compliance
pub mod contrast_ratios {
    /// WCAG AA minimum contrast ratio for normal text
    pub const AA_NORMAL: f32 = 4.5;
    
    /// WCAG AA minimum contrast ratio for large text
    pub const AA_LARGE: f32 = 3.0;
    
    /// WCAG AAA minimum contrast ratio for normal text
    pub const AAA_NORMAL: f32 = 7.0;
    
    /// WCAG AAA minimum contrast ratio for large text
    pub const AAA_LARGE: f32 = 4.5;
}

/// Accessibility helpers
pub mod accessibility {
    
    /// Get ARIA label for interactive elements
    pub fn get_interactive_aria_label(element_type: &str, state: Option<&str>) -> String {
        match state {
            Some(state) => format!("{} - {}", element_type, state),
            None => element_type.to_string(),
        }
    }
    
    /// Get keyboard navigation classes
    pub fn get_keyboard_nav_classes() -> &'static str {
        "focus-visible:ring-2 focus-visible:ring-forest-green-500 focus-visible:ring-offset-2"
    }
    
    /// Get screen reader only classes
    pub fn get_sr_only_classes() -> &'static str {
        "sr-only"
    }
}

/// Theme classes for buttons
pub struct ButtonThemeClasses {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub ghost: &'static str,
    pub outline: &'static str,
    pub danger: &'static str,
    pub success: &'static str,
}

/// Theme classes for cards
pub struct CardThemeClasses {
    pub default: &'static str,
    pub bordered: &'static str,
    pub elevated: &'static str,
    pub glass: &'static str,
}

/// Theme classes for inputs
pub struct InputThemeClasses {
    pub default: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
}

/// Responsive design breakpoints
pub mod breakpoints {
    /// Small screens and up (640px)
    pub const SM: &str = "sm:";
    
    /// Medium screens and up (768px)
    pub const MD: &str = "md:";
    
    /// Large screens and up (1024px)
    pub const LG: &str = "lg:";
    
    /// Extra large screens and up (1280px)
    pub const XL: &str = "xl:";
    
    /// 2X large screens and up (1536px)
    pub const XXL: &str = "2xl:";
}

/// Theme variants optimized for dark design
pub mod theme_variants {
    use super::{ThemeVariant, ContrastLevel, ButtonThemeClasses, CardThemeClasses, InputThemeClasses};
    
    /// Get button theme classes based on variant and contrast level
    pub fn get_button_theme_classes(variant: ThemeVariant, contrast: ContrastLevel) -> ButtonThemeClasses {
        match (variant, contrast) {
            (ThemeVariant::Dark, ContrastLevel::Standard) => ButtonThemeClasses {
                primary: "bg-forest-green-500 hover:bg-forest-green-600 text-light-beige-200 shadow-lg",
                secondary: "bg-vibrant-orange-500 hover:bg-vibrant-orange-600 text-light-beige-200 shadow-lg",
                ghost: "bg-transparent hover:bg-dark-charcoal-600 text-light-beige-500 hover:text-light-beige-300",
                outline: "bg-transparent hover:bg-forest-green-500 text-forest-green-400 hover:text-light-beige-200 border-2 border-forest-green-500",
                danger: "bg-vibrant-orange-600 hover:bg-vibrant-orange-700 text-light-beige-200 shadow-lg",
                success: "bg-forest-green-400 hover:bg-forest-green-500 text-light-beige-200 shadow-lg",
            },
            (ThemeVariant::Dark, ContrastLevel::Enhanced) => ButtonThemeClasses {
                primary: "bg-forest-green-400 hover:bg-forest-green-300 text-light-beige-100 shadow-lg border border-light-beige-400",
                secondary: "bg-vibrant-orange-400 hover:bg-vibrant-orange-300 text-light-beige-100 shadow-lg border border-light-beige-400",
                ghost: "bg-transparent hover:bg-dark-charcoal-500 text-light-beige-200 hover:text-light-beige-100",
                outline: "bg-transparent hover:bg-forest-green-400 text-forest-green-300 hover:text-light-beige-100 border-2 border-forest-green-400",
                danger: "bg-vibrant-orange-500 hover:bg-vibrant-orange-400 text-light-beige-100 shadow-lg border border-light-beige-400",
                success: "bg-forest-green-300 hover:bg-forest-green-200 text-light-beige-100 shadow-lg border border-light-beige-400",
            },
            (ThemeVariant::HighContrast, _) => ButtonThemeClasses {
                primary: "bg-forest-green-400 hover:bg-forest-green-300 text-light-beige-100 shadow-xl border-2 border-light-beige-300",
                secondary: "bg-vibrant-orange-400 hover:bg-vibrant-orange-300 text-light-beige-100 shadow-xl border-2 border-light-beige-300",
                ghost: "bg-transparent hover:bg-dark-charcoal-500 text-light-beige-200 hover:text-light-beige-100 border border-light-beige-400",
                outline: "bg-transparent hover:bg-forest-green-400 text-forest-green-200 hover:text-light-beige-100 border-3 border-forest-green-400",
                danger: "bg-vibrant-orange-500 hover:bg-vibrant-orange-400 text-light-beige-100 shadow-xl border-2 border-light-beige-300",
                success: "bg-forest-green-300 hover:bg-forest-green-200 text-light-beige-100 shadow-xl border-2 border-light-beige-300",
            },
            (ThemeVariant::ReducedMotion, contrast) => {
                match contrast {
                    ContrastLevel::Standard => ButtonThemeClasses {
                        primary: "bg-forest-green-500 hover:bg-forest-green-600 text-light-beige-200 shadow-lg transition-none",
                        secondary: "bg-vibrant-orange-500 hover:bg-vibrant-orange-600 text-light-beige-200 shadow-lg transition-none",
                        ghost: "bg-transparent hover:bg-dark-charcoal-600 text-light-beige-500 hover:text-light-beige-300 transition-none",
                        outline: "bg-transparent hover:bg-forest-green-500 text-forest-green-400 hover:text-light-beige-200 border-2 border-forest-green-500 transition-none",
                        danger: "bg-vibrant-orange-600 hover:bg-vibrant-orange-700 text-light-beige-200 shadow-lg transition-none",
                        success: "bg-forest-green-400 hover:bg-forest-green-500 text-light-beige-200 shadow-lg transition-none",
                    },
                    ContrastLevel::Enhanced => ButtonThemeClasses {
                        primary: "bg-forest-green-400 hover:bg-forest-green-300 text-light-beige-100 shadow-xl border-2 border-light-beige-300 transition-none",
                        secondary: "bg-vibrant-orange-400 hover:bg-vibrant-orange-300 text-light-beige-100 shadow-xl border-2 border-light-beige-300 transition-none",
                        ghost: "bg-transparent hover:bg-dark-charcoal-500 text-light-beige-200 hover:text-light-beige-100 border border-light-beige-400 transition-none",
                        outline: "bg-transparent hover:bg-forest-green-400 text-forest-green-200 hover:text-light-beige-100 border-3 border-forest-green-400 transition-none",
                        danger: "bg-vibrant-orange-500 hover:bg-vibrant-orange-400 text-light-beige-100 shadow-xl border-2 border-light-beige-300 transition-none",
                        success: "bg-forest-green-300 hover:bg-forest-green-200 text-light-beige-100 shadow-xl border-2 border-light-beige-300 transition-none",
                    }
                }
            }
        }
    }
    
    /// Get card theme classes based on variant and contrast level
    pub fn get_card_theme_classes(variant: ThemeVariant, contrast: ContrastLevel) -> CardThemeClasses {
        match (variant, contrast) {
            (ThemeVariant::Dark, ContrastLevel::Standard) => CardThemeClasses {
                default: "bg-dark-charcoal-500 text-light-beige-300",
                bordered: "bg-dark-charcoal-500 text-light-beige-300 border border-dark-charcoal-400",
                elevated: "bg-dark-charcoal-500 text-light-beige-300 shadow-xl shadow-dark-charcoal-900/20",
                glass: "bg-dark-charcoal-500/80 backdrop-blur-lg text-light-beige-300 border border-dark-charcoal-400/30",
            },
            (ThemeVariant::Dark, ContrastLevel::Enhanced) => CardThemeClasses {
                default: "bg-dark-charcoal-600 text-light-beige-200 border border-light-beige-700",
                bordered: "bg-dark-charcoal-600 text-light-beige-200 border-2 border-light-beige-600",
                elevated: "bg-dark-charcoal-600 text-light-beige-200 shadow-xl shadow-dark-charcoal-900/30 border border-light-beige-700",
                glass: "bg-dark-charcoal-600/85 backdrop-blur-lg text-light-beige-200 border border-light-beige-600/50",
            },
            (ThemeVariant::HighContrast, _) => CardThemeClasses {
                default: "bg-dark-charcoal-600 text-light-beige-200 border border-light-beige-600",
                bordered: "bg-dark-charcoal-600 text-light-beige-200 border-2 border-light-beige-500",
                elevated: "bg-dark-charcoal-600 text-light-beige-200 shadow-2xl shadow-dark-charcoal-900/40 border border-light-beige-600",
                glass: "bg-dark-charcoal-600/90 backdrop-blur-xl text-light-beige-200 border-2 border-light-beige-500/50",
            },
            (ThemeVariant::ReducedMotion, contrast) => {
                match contrast {
                    ContrastLevel::Standard => CardThemeClasses {
                        default: "bg-dark-charcoal-500 text-light-beige-300 transition-none",
                        bordered: "bg-dark-charcoal-500 text-light-beige-300 border border-dark-charcoal-400 transition-none",
                        elevated: "bg-dark-charcoal-500 text-light-beige-300 shadow-xl shadow-dark-charcoal-900/20 transition-none",
                        glass: "bg-dark-charcoal-500/80 backdrop-blur-lg text-light-beige-300 border border-dark-charcoal-400/30 transition-none",
                    },
                    ContrastLevel::Enhanced => CardThemeClasses {
                        default: "bg-dark-charcoal-600 text-light-beige-200 border border-light-beige-600 transition-none",
                        bordered: "bg-dark-charcoal-600 text-light-beige-200 border-2 border-light-beige-500 transition-none",
                        elevated: "bg-dark-charcoal-600 text-light-beige-200 shadow-2xl shadow-dark-charcoal-900/40 border border-light-beige-600 transition-none",
                        glass: "bg-dark-charcoal-600/90 backdrop-blur-xl text-light-beige-200 border-2 border-light-beige-500/50 transition-none",
                    }
                }
            }
        }
    }
    
    /// Get input theme classes based on variant and contrast level
    pub fn get_input_theme_classes(variant: ThemeVariant, contrast: ContrastLevel) -> InputThemeClasses {
        match (variant, contrast) {
            (ThemeVariant::Dark, ContrastLevel::Standard) => InputThemeClasses {
                default: "bg-dark-charcoal-400 border border-dark-charcoal-300 text-light-beige-300 placeholder-light-beige-600",
                success: "bg-dark-charcoal-400 border border-forest-green-400 text-light-beige-300 placeholder-light-beige-600",
                warning: "bg-dark-charcoal-400 border border-vibrant-orange-500 text-light-beige-300 placeholder-light-beige-600",
                error: "bg-dark-charcoal-400 border border-vibrant-orange-500 text-light-beige-300 placeholder-light-beige-600",
            },
            (ThemeVariant::Dark, ContrastLevel::Enhanced) => InputThemeClasses {
                default: "bg-dark-charcoal-500 border-2 border-light-beige-500 text-light-beige-200 placeholder-light-beige-500",
                success: "bg-dark-charcoal-500 border-2 border-forest-green-300 text-light-beige-200 placeholder-light-beige-500",
                warning: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500",
                error: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500",
            },
            (ThemeVariant::HighContrast, _) => InputThemeClasses {
                default: "bg-dark-charcoal-500 border-2 border-light-beige-400 text-light-beige-200 placeholder-light-beige-500",
                success: "bg-dark-charcoal-500 border-2 border-forest-green-300 text-light-beige-200 placeholder-light-beige-500",
                warning: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500",
                error: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500",
            },
            (ThemeVariant::ReducedMotion, contrast) => {
                match contrast {
                    ContrastLevel::Standard => InputThemeClasses {
                        default: "bg-dark-charcoal-400 border border-dark-charcoal-300 text-light-beige-300 placeholder-light-beige-600 transition-none",
                        success: "bg-dark-charcoal-400 border border-forest-green-400 text-light-beige-300 placeholder-light-beige-600 transition-none",
                        warning: "bg-dark-charcoal-400 border border-vibrant-orange-500 text-light-beige-300 placeholder-light-beige-600 transition-none",
                        error: "bg-dark-charcoal-400 border border-vibrant-orange-500 text-light-beige-300 placeholder-light-beige-600 transition-none",
                    },
                    ContrastLevel::Enhanced => InputThemeClasses {
                        default: "bg-dark-charcoal-500 border-2 border-light-beige-400 text-light-beige-200 placeholder-light-beige-500 transition-none",
                        success: "bg-dark-charcoal-500 border-2 border-forest-green-300 text-light-beige-200 placeholder-light-beige-500 transition-none",
                        warning: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500 transition-none",
                        error: "bg-dark-charcoal-500 border-2 border-vibrant-orange-400 text-light-beige-200 placeholder-light-beige-500 transition-none",
                    }
                }
            }
        }
    }
}

/// Mobile-first responsive utilities
pub mod mobile_responsive {
    /// Get mobile-optimized touch target classes
    pub fn get_touch_target_classes() -> &'static str {
        "min-h-[44px] min-w-[44px] touch-manipulation"
    }
    
    /// Get mobile-optimized spacing classes
    pub fn get_mobile_spacing_classes() -> &'static str {
        "px-4 py-3 sm:px-6 sm:py-4"
    }
    
    /// Get mobile-optimized text classes
    pub fn get_mobile_text_classes() -> &'static str {
        "text-base sm:text-sm"
    }
    
    /// Get mobile-optimized button classes
    pub fn get_mobile_button_classes() -> &'static str {
        "px-6 py-3 text-base sm:px-4 sm:py-2 sm:text-sm"
    }
}