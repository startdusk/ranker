const colors = require('tailwindcss/colors')

module.exports = {
  content: ['./index.html', './src/**/*.{vue,js,ts,tsx,jsx}'],
  theme: {
    extend: {
      colors: {
        orange: colors.orange,
        purple: colors.purple,
      },
    },
  },
  plugins: [],
}