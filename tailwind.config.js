/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html,css}",
        "./dist/**/*.html",
    ],
    theme: {
        extend: {
            colors: {
                'neutral-900': '#1A1A1A', // Hintergrund
                'neutral-800': '#2A2A2A', // Sekundär-Hintergrund
                'green-500': '#6DBF4B',   // Primär-Akzent (Grün)
                'amber-600': '#A97449',   // Sekundär-Akzent (Holz)
                'neutral-100': '#EAEAEA', // Text Hauptfarbe
                'neutral-400': '#AAAAAA'  // Text Sekundär
            },
            fontFamily: {
                heading: ['Manrope', 'sans-serif'], // Primärschrift
                body: ['Manrope', 'sans-serif'],   // Primärschrift
                alt: ['Inter', 'sans-serif'],      // Alternativen
            }
        }
    },
    plugins: [
        require('@tailwindcss/forms'), // Für bessere Input-Feld-Stile
        require('@tailwindcss/typography'), // Für Typografie-Optimierungen
    ],
}