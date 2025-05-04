import { defineConfig } from '@rsbuild/core';
import { pluginVue } from '@rsbuild/plugin-vue';

export default defineConfig({
  plugins: [pluginVue()],
  source: {
    entry: {
      index: './index.js'
    }
  },
  output: {
    assetPrefix: '/wasm-demo',
    distPath: {
      root: 'dist'
    }
  },
  html:{
    mountId: 'app'
  },
  server: {
    port: 8080,
    base: '/wasm-demo'
  },
  tools: {
    webpack: {
      config: {
        experiments: {
          asyncWebAssembly: true
        }
      }
    }
  }
});