/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html,css,js,ts,jsx,tsx}",
        "./dist/**/*.html",
        "./public/**/*.html",
        "./index.html"
    ],
    darkMode: 'class', // Ermöglicht Dark/Light Mode Toggle
    theme: {
        extend: {
            colors: {
                // Dark Mode Farben (Standard) - Design Guide konform
                'neutral': {
                    900: '#1A1A1A', // Hintergrund
                    800: '#2A2A2A', // Sekundär-Hintergrund
                    700: '#3A3A3A', // Input-Felder (angepasst für besseren Kontrast)
                    600: '#4A4A4A', // Border/Divider
                    400: '#AAAAAA', // Text Sekundär
                    100: '#EAEAEA', // Text Hauptfarbe
                },
                'green': {
                    500: '#6DBF4B', // Primär-Akzent
                    600: '#5BA63F', // Hover-State
                    400: '#89D76A', // Light Mode Variante
                },
                'amber': {
                    600: '#A97449', // Sekundär-Akzent (Holz)
                    700: '#8B5E3C', // Hover-State
                    400: '#D4A373', // Light Mode Variante
                },
                // Light Mode Farben
                'gray': {
                    100: '#F7F7F7', // Light Mode Hintergrund
                    200: '#ECECEC', // Light Mode Sekundär-Hintergrund
                    300: '#D1D5DB', // Light Mode Borders
                },
                // Legacy Farben (zur Kompatibilität)
                'stone-black': '#1F1F1F',
                'wood-oak': '#8B5E3C',
                'wood-oak-light': '#A67B5B',
                'plant-green': '#4B7C4B',
                'plant-green-light': '#6B8E23',
                'neutral-gray': '#F5F5F5'
            },
            fontFamily: {
                // Design Guide konforme Schriftarten
                'sans': ['Manrope', 'Inter', 'system-ui', 'sans-serif'],
                'heading': ['Manrope', 'Inter', 'sans-serif'],
                'body': ['Manrope', 'Inter', 'sans-serif']
            },
            fontSize: {
                // Design Guide konforme Größen mit optimierten Line-Heights
                '5xl': ['3rem', { lineHeight: '1.1', fontWeight: '700' }], // Hero-Titel
                '3xl': ['1.875rem', { lineHeight: '1.2', fontWeight: '600' }], // Sektionen
                'base': ['1rem', { lineHeight: '1.5', fontWeight: '400' }], // Fließtext
                'sm': ['0.875rem', { lineHeight: '1.4', fontWeight: '300' }], // Kleintext/Label
            },
            borderRadius: {
                'xl': '0.75rem',
                '2xl': '1rem',
            },
            spacing: {
                // 8px Grid-System erweitert
                '18': '4.5rem', // 72px
                '22': '5.5rem', // 88px
                '26': '6.5rem', // 104px
            },
            boxShadow: {
                'inner': 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)',
                'lg-hover': '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
                // Dark Mode optimierte Schatten
                'dark-lg': '0 10px 15px -3px rgba(0, 0, 0, 0.3), 0 4px 6px -2px rgba(0, 0, 0, 0.2)',
            },
            animation: {
                'fade-in': 'fadeIn 0.3s ease-in-out',
                'slide-up': 'slideUp 0.3s ease-out',
                'pulse-green': 'pulseGreen 2s infinite',
            },
            keyframes: {
                fadeIn: {
                    '0%': { opacity: '0' },
                    '100%': { opacity: '1' },
                },
                slideUp: {
                    '0%': { transform: 'translateY(10px)', opacity: '0' },
                    '100%': { transform: 'translateY(0)', opacity: '1' },
                },
                pulseGreen: {
                    '0%, 100%': { boxShadow: '0 0 0 0 rgba(109, 191, 75, 0.7)' },
                    '70%': { boxShadow: '0 0 0 10px rgba(109, 191, 75, 0)' },
                },
            },
            // Screen-Reader und Accessibility
            screens: {
                'xs': '475px', // Extra kleine Geräte
            },
        },
    },
    plugins: [
        require('@tailwindcss/forms'), // Für bessere Input-Feld-Stile
        require('@tailwindcss/typography'), // Für Typografie-Optimierungen
        // Optionale Plugins falls verfügbar:
        // require('@tailwindcss/aspect-ratio'),
    ],
}