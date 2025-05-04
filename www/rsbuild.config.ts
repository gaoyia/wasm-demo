import { defineConfig } from '@rsbuild/core';

export default defineConfig({
  source: {
    entry: {
      index: './index.js'
    }
  },
  output: {
    distPath: {
      root: 'dist'
    }
  },
  server: {
    port: 8080
  }
});