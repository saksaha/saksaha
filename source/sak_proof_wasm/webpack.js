const fs = require("fs");
const webpack = require("webpack");
const webpackConfig = require("./webpack.config");

webpack(webpackConfig, (err, stats) => {
  if (err) {
    console.log(err);
    return;
  }

  stats = stats.toJson({
    assets: true,
  });

  for (asset of stats.assets) {
    console.log(1, asset.name);
  }
});
