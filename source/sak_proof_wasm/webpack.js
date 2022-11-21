const fs = require("fs");
const path = require("path");
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

  console.log(stats.outputPath);

  const concatPath = path.join(__dirname, "dist/sak_proof_concat.js");

  const ws = fs.createWriteStream(concatPath);

  for (asset of stats.assets) {
    if (asset.name.endsWith(".js")) {
      console.log(1, asset.name);

      let p = path.join(stats.outputPath, asset.name);
      let file = fs.readFileSync(p, { encoding: "utf-8" });

      console.log(11, file);

      ws.write(file);
    }
  }
});
