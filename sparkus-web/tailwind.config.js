/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      animation: {
        marquee: "marquee var(--duration) linear infinite",
        "marquee-vertical": "marquee-vertical var(--duration) linear infinite",
      },
      keyframes: {
        marquee: {
          from: { transform: "translateX(100%)" }, // 开始时，内容从右边进入
          to: { transform: "translateX(-100%)" },   // 结束时，内容离开左边
        },
        "marquee-vertical": {
          from: { transform: "translateY(100%)" },
          to: { transform: "translateY(-100%)" },
        },
      },
      // animation: {
      //   marquee: "marquee var(--duration) linear infinite",
      //   "marquee-vertical": "marquee-vertical var(--duration) linear infinite",
      // },
      // keyframes: {
      //   marquee: {
      //     from: { transform: "translateX(0)" },
      //     to: { transform: "translateX(calc(-100% - var(--gap)))" },
      //   },
      //   "marquee-vertical": {
      //     from: { transform: "translateY(0)" },
      //     to: { transform: "translateY(calc(-100% - var(--gap)))" },
      //   },
      // },
    },
  },
};