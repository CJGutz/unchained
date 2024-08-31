/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.{html,js,ts}"],
  theme: {
    container: {
      center: true,
    },
    extend: {
      colors: {
        'light-red': '#FFDADA',
        'light-blue': '#DADFFF',
        'off-white': '#F8F8F8',
      },
      backgroundImage: {
        'gradient-radial':
          'radial-gradient(farthest-corner at 300px 400px, var(--tw-gradient-stops))',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
