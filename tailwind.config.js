module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "class", // 'media' or 'class'
    theme: {
        extend: {
            screens: {
                "xs": "475px",
            },
        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
};