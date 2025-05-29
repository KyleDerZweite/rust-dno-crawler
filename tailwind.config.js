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
                sage: {
                    50: '#f9fafb',
                    100: '#f3f4f6',
                    200: '#e5e7eb',
                    300: '#d1d5db',
                    400: '#9ca3af',
                    500: '#6b7280',
                    600: '#4b5563',
                    700: '#374151',
                    800: '#1f2937',
                    900: '#111827',
                },
                moss: {
                    50: '#ecfdf5',
                    100: '#d1fae5',
                    200: '#a7f3d0',
                    300: '#6ee7b7',
                    400: '#34d399',
                    500: '#10b981',
                    600: '#059669',
                    700: '#047857',
                    800: '#065f46',
                    900: '#064e3b',
                },
                bark: {
                    50: '#fefbf3',
                    100: '#fef3c7',
                    200: '#fde68a',
                    300: '#fcd34d',
                    400: '#fbbf24',
                    500: '#f59e0b',
                    600: '#d97706',
                    700: '#b45309',
                    800: '#92400e',
                    900: '#78350f',
                },
                stone: {
                    50: '#fafaf9',
                    100: '#f5f5f4',
                    200: '#e7e5e4',
                    300: '#d6d3d1',
                    400: '#a8a29e',
                    500: '#78716c',
                    600: '#57534e',
                    700: '#44403c',
                    800: '#292524',
                    900: '#1c1917',
                }
            },
            animation: {
                'gentle-float': 'gentle-float 6s ease-in-out infinite',
                'bloom-in': 'bloom-in 0.3s ease-out',
                'leaf-sway': 'leaf-sway 3s ease-in-out infinite',
                'grow-bloom': 'grow-bloom 0.3s ease-out',
            },
            keyframes: {
                'gentle-float': {
                    '0%, 100%': {
                        transform: 'translateX(-50%) translateY(0px)'
                    },
                    '50%': {
                        transform: 'translateX(-50%) translateY(-3px)'
                    },
                },
                'bloom-in': {
                    '0%': {
                        opacity: '0',
                        transform: 'translateX(-50%) scale(0.95) translateY(-10px)'
                    },
                    '100%': {
                        opacity: '1',
                        transform: 'translateX(-50%) scale(1) translateY(0px)'
                    },
                },
                'leaf-sway': {
                    '0%, 100%': {
                        transform: 'rotate(0deg) scale(1)'
                    },
                    '25%': {
                        transform: 'rotate(1deg) scale(1.02)'
                    },
                    '75%': {
                        transform: 'rotate(-1deg) scale(1.02)'
                    },
                },
                'grow-bloom': {
                    '0%': {
                        transform: 'scale(1)',
                        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)'
                    },
                    '50%': {
                        transform: 'scale(1.05)',
                        boxShadow: '0 10px 15px -3px rgba(16, 185, 129, 0.3)'
                    },
                    '100%': {
                        transform: 'scale(1.02)',
                        boxShadow: '0 10px 15px -3px rgba(16, 185, 129, 0.2)'
                    },
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
                '4xl': '2rem',
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