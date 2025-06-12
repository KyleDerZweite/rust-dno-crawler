/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./src/**/*.{rs,html,css,js,ts,jsx,tsx}",
        "./dist/**/*.html"
    ],
    theme: {
        extend: {
            colors: {
                /**
                 * @group Base Dark Theme
                 * @description Primary background and structural color.
                 * Used for page backgrounds, section containers, and subtle UI elements.
                 * Shades range from very light (for slight variations) to deepest black.
                 */
                'dark-charcoal': {
                    100: '#4A4A4A', // For subtle variations, light text on dark-charcoal-500+ backgrounds
                    200: '#3D3D3D', // Hover states on darker dark-charcoal elements
                    300: '#303030', // Borders on dark-charcoal sections
                    400: '#282828', // Input field backgrounds, very subtle card backgrounds
                    500: '#1C1C1C', // Base dark charcoal for main sections, cards, default input backgrounds
                    600: '#171717', // Slightly darker sections, modal backgrounds
                    700: '#121212', // Main section backgrounds, deeper card backgrounds
                    800: '#0D0D0D', // Primary page background, deepest sections
                    900: '#080808', // Deepest background for stark contrast, footer/header background
                },

                /**
                 * @group Green Accents
                 * @description Primary natural green accent.
                 * Used for main calls to action (CTAs), primary icons, and vibrant natural elements.
                 * Provides a direct connection to the lush greenery in the image.
                 */
                'forest-green': {
                    100: '#AEC1B1', // Very light green for subtle highlights or graphic elements
                    200: '#93AD99', // Lighter accent text or subtle hover effects
                    300: '#789982', // Accent text, link default state
                    400: '#5D856B', // More prominent accent text, borders, ghost button text
                    500: '#3B533E', // Base primary accent: buttons, primary icons, active states
                    600: '#324736', // Button hover states, darker accent elements
                    700: '#293B2E', // Button active/pressed states, deep green illustrations
                    800: '#202F26', // Disabled button backgrounds, very dark green details
                    900: '#17231E', // Deepest green for illustrations, shadows
                },

                /**
                 * @group Green Accents (Muted)
                 * @description Secondary, softer green accent.
                 * Used for subtle decorative elements, less prominent links, visited states,
                 * and background patterns where a strong green isn't desired.
                 * It offers a slightly different, more subdued green tone compared to `forest-green`.
                 *
                 * @necessity This color offers a distinct, more desaturated green than `forest-green`.
                 * While some very light shades of `forest-green` might overlap, `muted-olive` provides a
                 * consistent range for a softer, more subtle green presence. It's useful for adding
                 * variety without making every green element "pop" like `forest-green`.
                 */
                'muted-olive': {
                    100: '#C5D1BD', // Very light olive for soft patterns or texture
                    200: '#A9BBA3', // Subtle background elements or inactive states
                    300: '#8EAA89', // Decorative accents, light dividers
                    400: '#739470', // Less prominent accent icons, subtle borders
                    500: '#6D7C66', // Base muted olive: secondary decorative elements, visited links
                    600: '#5A6A54', // Slightly darker muted olive for hover on soft elements
                    700: '#475842', // Deeper decorative elements
                    800: '#344630', // Background details
                    900: '#21341E', // Very dark muted olive for depth
                },

                /**
                 * @group Brown/Amber Accents
                 * @description Warm, earthy accent color with an amber/terracotta feel.
                 * Used for secondary CTAs, badges, warnings, and elements that need a warm,
                 * inviting pop of color reminiscent of terracotta pots.
                 */
                'amber-brown': {
                    100: '#F5D1B8', // Very light amber for highlights or background textures
                    200: '#EBB394', // Lighter accent, perhaps for subtle UI highlights
                    300: '#E19570', // Accent text, softer highlights
                    400: '#D7774C', // More prominent accent text, borders
                    500: '#C37D57', // Base secondary accent: secondary buttons, badge backgrounds
                    600: '#B06845', // Button hover states, darker amber elements
                    700: '#9D5333', // Button active states, deep amber illustrations
                    800: '#8A3E21', // Disabled button backgrounds, very dark amber details
                    900: '#77290F', // Deepest amber for illustrations, shadows
                },

                /**
                 * @group Brown Accents (Sienna)
                 * @description Deeper, richer brown accent.
                 * Used for elements that derive from wooden textures, subtle shadows,
                 * and a more grounded, less vibrant brown accent than `amber-brown`.
                 *
                 * @necessity This color provides a distinct, true brown tone that `amber-brown` (which leans more orange) does not fully cover.
                 * It's essential for replicating the deeper, natural wood tones from the image and offers more grounding accents.
                 */
                'sienna-brown': {
                    100: '#E1B89B', // Light sienna for subtle textures
                    200: '#C99E82', // Soft highlights on brown elements
                    300: '#B18469', // Decorative elements, soft borders
                    400: '#996A50', // More prominent decorative accents
                    500: '#8B5C3D',  // Base sienna: wooden elements, deeper textured accents
                    600: '#784B30', // Deeper wooden elements, shadows
                    700: '#653A23', // Darker sienna for depth
                    800: '#522916', // Very dark sienna for fine details
                    900: '#3F1809', // Deepest sienna, for shadows or very dark wood
                },

                /**
                 * @group Light Elements
                 * @description Primary light color for text and highlights.
                 * Essential for readability against dark backgrounds, mimicking natural light sources.
                 */
                'light-beige': {
                    100: '#FFFFFF', // Pure white (use sparingly for stark highlights if needed)
                    200: '#F7F1EB', // Softest white for subheadings or subtle text on dark backgrounds
                    300: '#EEE7E1', // Primary heading text, main content text for high readability
                    400: '#E5DED7', // General body text, slightly less prominent than headings
                    500: '#DCC09B', // Base light beige: for secondary text, subtle accents, default button text on dark buttons
                    600: '#C3A984', // Slightly darker beige for hover states on light elements
                    700: '#AA926D', // More muted light beige for disabled text or subtle shadows on light elements
                    800: '#917B56', // Darker beige for specific contrast needs
                    900: '#78643F', // Deepest beige, almost a muted light brown, for subtle elements
                },
            },
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
    ]
}