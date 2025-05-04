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
    distPath: {
      root: 'dist'
    }
  },
  html:{
    mountId: 'app'
  },
  server: {
    port: 8080
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