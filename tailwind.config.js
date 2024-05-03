/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      backgroundImage: {
        'westminster': "url('/assets/westminster palace.jpg')",
      },
      colors: {
        'lordscolour': '#ae3225',
        'commonscolour': '#1b9e9c'
      }
    },
  },
  plugins: [],
};
