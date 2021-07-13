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
                darker: "#151515",
                "light-dark": "#333333",
                light: "#d6d6d6",
                gray: colors.trueGray,
                "rank-platinum": "#4E9996",
                negative: "#DE2F2F",
                positive: "#20BF55",
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
                winrate: {
                    shiggo: "#ff4e50",
                    meh: "#fcb1b2",
                    "okay-dark": colors.gray[900],
                    good: "#7ea4f4",
                    great: "#3273fa",
                    volxd: "#ff9b00",
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
