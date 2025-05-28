/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html,css,js,ts,jsx,tsx}",
        "./dist/**/*.html"
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                // Dark Mode Farben (Standard) - Design Guide konform
                'neutral': {
                    900: '#1A1A1A', // Hintergrund
                    800: '#2A2A2A', // Sekundär-Hintergrund
                    700: '#3A3A3A', // Input-Felder
                    600: '#4A4A4A', // Border/Divider
                    400: '#AAAAAA', // Text Sekundär
                    100: '#EAEAEA', // Text Hauptfarbe
                },
                'green': {
                    500: '#4B7C4B', // Primär-Akzent
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
            },
            fontFamily: {
                'sans': ['Manrope', 'Inter', 'system-ui', 'sans-serif'],
                'heading': ['Manrope', 'Inter', 'sans-serif'],
                'body': ['Manrope', 'Inter', 'sans-serif']
            },
            fontSize: {
                '5xl': ['3rem', { lineHeight: '1.1', fontWeight: '700' }],
                '3xl': ['1.875rem', { lineHeight: '1.2', fontWeight: '600' }],
                'base': ['1rem', { lineHeight: '1.5', fontWeight: '400' }],
                'sm': ['0.875rem', { lineHeight: '1.4', fontWeight: '300' }],
            },
            borderRadius: {
                'xl': '0.75rem',
                '2xl': '1rem',
            },
            spacing: {
                '18': '4.5rem', // 72px
                '22': '5.5rem', // 88px
                '26': '6.5rem', // 104px
            },
            screens: {
                'xs': '475px',
            },
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ],
}