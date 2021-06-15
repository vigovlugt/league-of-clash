const colors = require("tailwindcss/colors");

module.exports = {
    mode: "jit",
    purge: ["./src/**/*.{js,ts,jsx,tsx}"],
    darkMode: false, // or 'media' or 'class'
    theme: {
        extend: {
            colors: {
                primary: "#d0a755",
                "primary-dark": "#73571F",
                dark: "#202020",
                "light-dark": "#333333",
                light: "#d6d6d6",
                gray: colors.trueGray,
                "rank-platinum": "#4E9996",
                rank: {
                    challenger: "#F4C874",
                    grandmaster: "#CD4545",
                    master: "#9D48E0",
                    diamond: "#576BCE",
                    platinum: "#4E9996",
                    gold: "#CD8837",
                    silver: "#80989D",
                    bronze: "#8C523A",
                    iron: "#574D4F",
                },
            },
            fontFamily: {
                header: "Agency, sans-serif",
            },
        },
    },
    variants: {
        extend: {
            borderWidth: ["active"],
        },
    },
    plugins: [],
};
