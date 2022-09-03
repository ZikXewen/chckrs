/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.tsx'],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: [
    // "after:content-['🟦']",
    "after:content-['⬛']",
    "after:content-['⬜']",
    "after:content-['⚪']",
    "after:content-['⚫']",
  ],
}
