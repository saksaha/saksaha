import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";
import path from "path";
import topLevelAwait from "vite-plugin-top-level-await";
import { vanillaExtractPlugin } from "@vanilla-extract/vite-plugin";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [
    solidPlugin(),
    vanillaExtractPlugin(),
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
    // wasm(),
  ],
  server: {
    port: 3000,
    fs: {
      allow: [path.resolve(__dirname, "../../")],
    },
  },
  build: {
    target: "esnext",
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      "@components": path.resolve(__dirname, "./src/components"),
    },
  },
  // worker: {
  //   format: "es",
  //   plugins: [wasm(), topLevelAwait()],
  // },
  optimizeDeps: {
    exclude: ["/dist/proof.js"],
  },
});
