import path from "path";
import { defineConfig, loadEnv } from "vite";
import solidPlugin from "vite-plugin-solid";
import { vanillaExtractPlugin } from "@vanilla-extract/vite-plugin";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default () => {
  process.env = { ...process.env, POWER: "1" };

  return defineConfig({
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
        "@components": path.resolve(__dirname, "./src/components"),
      },
    },
    plugins: [
      solidPlugin(),
      vanillaExtractPlugin(),
      //
      wasm(),
      topLevelAwait(),
      {
        name: "configure-response-headers",
        configureServer: (server) => {
          server.middlewares.use((_req, res, next) => {
            res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
            res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
            next();
          });
        },
      },
    ],
    server: {
      port: 3000,
      fs: {
        allow: [path.resolve(__dirname, "../../../../")],
      },
    },
    build: {
      target: "esnext",
    },
  });
};
