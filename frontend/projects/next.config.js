const path = require('path');

const nextConfig = {
    webpack: config =>{
        config.resolve.alias["@/*"] = path.resolve(__dirname,"./src/app/*");
        return config;
    }
};

module.exports = nextConfig;
