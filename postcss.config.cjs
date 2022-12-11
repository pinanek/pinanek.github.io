const presetEnv = require("postcss-preset-env");

module.exports = {
  plugins: [
    presetEnv({
      autoprefixer: false,
      features: {
        "custom-properties": false,
        "custom-selectors": true,
        "custom-media-queries": {
          importFrom: ["./src/styles/media.css"],
        },
      },
    }),
  ],
};
