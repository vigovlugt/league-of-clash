module.exports = {
    future: {
        webpack5: true,
    },
    webpack: (config) => {
        config.experiments = {
            syncWebAssembly: true,
        };

        return config;
    },
};
