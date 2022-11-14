const path = require("path");
const { VanillaExtractPlugin } = require("@vanilla-extract/webpack-plugin");

module.exports = {
  mode: "development",
  devtool: "source-map",
  entry: {
    main: path.join(__dirname, "src/index.tsx"),
  },
  devServer: {
    static: path.join(__dirname, "dist"),
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
  plugins: [new VanillaExtractPlugin()],
};
