// tailwind.config.js
module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: false, // 或者 'media' 或 'class'
  theme: {
    extend: {
      colors: {
        vibrant: {
          light: '#f0e130',
          DEFAULT: '#ffd700',
          dark: '#c5b358',
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
