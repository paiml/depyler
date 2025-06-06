import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  plugins: [react(), wasm()],
  worker: {
    format: 'es',
    plugins: () => [wasm()]
  },
  build: {
    target: 'es2022',
    minify: false,
    rollupOptions: {
      external: ['/wasm/depyler_wasm.js']
    }
  },
  resolve: {
    alias: {
      '@/': new URL('./src/', import.meta.url).pathname
    }
  },
  server: {
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin'
    }
  }
});