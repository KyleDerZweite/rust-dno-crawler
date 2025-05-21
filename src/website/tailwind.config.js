/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html,css}",
        "./dist/**/*.html",
    ],
    theme: {
        extend: {
            colors: {
                'stone-black': '#1F1F1F',
                'wood-oak': '#8B5E3C',
                'wood-oak-light': '#A67B5B',
                'plant-green': '#4B7C4B',
                'plant-green-light': '#6B8E23',
                'neutral-gray': '#F5F5F5'
            },
            fontFamily: {
                heading: ['Merriweather', 'serif'],
                body: ['Inter', 'sans-serif']
            }
        }
    },
    plugins: [],
}