const colors = require("tailwindcss/colors");

module.exports = {
    mode: "jit",
    purge: ["./src/**/*.{js,ts,jsx,tsx}"],
    darkMode: false, // or 'media' or 'class'
    theme: {
        extend: {
            colors: {
                primary: "#d0a755",
                dark: "#202020",
                "light-dark": "#333333",
                light: "#d6d6d6",
                gray: colors.trueGray,
            },
            fontFamily: {
                header: "Agency, sans-serif",
            },
        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
};
