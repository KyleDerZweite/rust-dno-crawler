/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./src/**/*.{rs,html,css,js,ts,jsx,tsx}",
        "./dist/**/*.html"
    ],
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}