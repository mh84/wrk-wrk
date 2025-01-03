const colors = require('tailwindcss/colors');

module.exports = {
  mode: 'jit',
  content: [
    './**/*.{html,css,rs}',
  ],
  theme: {
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: colors.black,
      white: colors.white,
      gray: colors.gray,
      blue: colors.sky,
      green: colors.emerald,
      yellow: colors.amber,
      red: colors.red,
      slate: colors.slate,
    },
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
