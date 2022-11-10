const pkg = require("./package.json");
const css = require("rollup-plugin-import-css");
const { babel } = require("@rollup/plugin-babel");
const commonjs = require("@rollup/plugin-commonjs").default;
const withSolid = require("rollup-preset-solid").default;
const json = require("@rollup/plugin-json").default;
const { nodeResolve } = require("@rollup/plugin-node-resolve");
const { terser } = require("rollup-plugin-terser");

const extensions = [".js", ".jsx", ".ts", ".tsx"];

const plugins = [
  json(),
  nodeResolve({ extensions }),
  commonjs(),
  babel({
    babelHelpers: "bundled",
    presets: ["@babel/preset-typescript"],
    extensions,
  }),
  terser(),
  css(),
];

module.exports = withSolid([
  {
    input: "src/index.tsx",
    external: [
      Object.keys(pkg.dependencies || {}),
      Object.keys(pkg.peerDependencies || {}),
    ].flat(),
    output: [
      {
        file: pkg.module,
        format: "esm",
      },
      {
        file: pkg.main,
        format: "cjs",
      },
    ],
    plugins,
  },
]);
