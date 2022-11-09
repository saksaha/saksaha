// import pkg from "./package.json" assert { type: "json" };
// import { babel } from "@rollup/plugin-babel";
// import commonjs from "@rollup/plugin-commonjs";
// import json from "@rollup/plugin-json";
// import { nodeResolve } from "@rollup/plugin-node-resolve";
// import { terser } from "rollup-plugin-terser";
// import css from "rollup-plugin-import-css";
import withSolid from "rollup-preset-solid";

const withSolid1 = withSolid.default;

export default withSolid1();

// const extensions = [".js", ".jsx", ".ts", ".tsx"];

// const plugins = [
//   json(),
//   nodeResolve({ extensions }),
//   commonjs(),
//   babel({
//     babelHelpers: "bundled",
//     presets: ["@babel/preset-react", "@babel/preset-typescript"],
//     extensions,
//   }),
//   terser(),
//   css(),
// ];

// export default [
//   {
//     input: "src/index.tsx",
//     external: [
//       Object.keys(pkg.dependencies || {}),
//       Object.keys(pkg.peerDependencies || {}),
//     ].flat(),
//     output: [
//       {
//         file: pkg.module,
//         format: "esm",
//       },
//       {
//         file: pkg.main,
//         format: "cjs",
//       },
//     ],
//     plugins,
//   },
//   //
//   // {
//   //   input: "src/index.tx",
//   //   output: [
//   //     {
//   //       name: pkg.name,
//   //       file: pkg.browser,
//   //       format: "umd",
//   //     },
//   //   ],
//   //   plugins,
//   // },
// ];
