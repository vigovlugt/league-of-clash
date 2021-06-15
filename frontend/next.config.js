module.exports = {
    webpack: (config) => {
        config.experiments = {
            syncWebAssembly: true,
        };

        return config;
    },
};
