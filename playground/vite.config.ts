import { defineConfig } from "npm:vite@^5.2.0";
import react from "npm:@vitejs/plugin-react@^4.2.1";
import wasm from "npm:vite-plugin-wasm@^3.3.0";

export default defineConfig({
  plugins: [react(), wasm()],
  worker: {
    format: "es",
    plugins: () => [wasm()],
  },
  build: {
    target: "es2022",
    minify: false,
  },
  resolve: {
    alias: {
      "@/": new URL("./src/", import.meta.url).pathname,
    },
  },
  server: {
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
  optimizeDeps: {
    exclude: ["depyler_wasm"],
  },
});
