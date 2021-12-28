module.exports = {
  content: [
      "./src/**/*.{js,jsx,ts,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      container: {
        center: true,
      },
      borderWidth: {
        '1': '1px',
      },
      screens: {
        'xs': { 'max': '475px' },
      },
      gridTemplateColumns: {
        'narrow-3': 'grid-template-columns: repeat(3, minmax(auto, 1fr))',
      },
    },
  },
  plugins: [],
}
