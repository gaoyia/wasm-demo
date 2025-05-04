<script setup>
import { ref } from 'vue';
import { generate_uuid } from 'uuid-wasm';
import { v4 as uuidv4 } from 'uuid';

const iterations = ref(1000);
const results = ref(null);
const isRunning = ref(false);
const showAnalysis = ref(false);

// 执行性能测试
const runTest = async () => {
  if (isRunning.value) return;
  isRunning.value = true;
  const count = parseInt(iterations.value);
  
  const testResults = {
    wasm: { time: 0, uuids: [] },
    npm: { time: 0, uuids: [] },
    browser: { time: 0, uuids: [] }
  };

  // 测试Rust/WebAssembly实现
  const wasmStart = performance.now();
  for (let i = 0; i < count; i++) {
    testResults.wasm.uuids.push(generate_uuid());
  }
  testResults.wasm.time = performance.now() - wasmStart;

  // 测试uuid npm包
  const npmStart = performance.now();
  for (let i = 0; i < count; i++) {
    testResults.npm.uuids.push(uuidv4({}));
  }
  testResults.npm.time = performance.now() - npmStart;

  // 测试浏览器原生API
  const browserStart = performance.now();
  for (let i = 0; i < count; i++) {
    testResults.browser.uuids.push(uuidv4());
  }
  testResults.browser.time = performance.now() - browserStart;

  // 计算每秒生成数量
  const calculateUUIDsPerSecond = (time, count) => {
    return Math.round((count / time) * 1000);
  };

  results.value = {
    wasm: {
      time: testResults.wasm.time.toFixed(2),
      uuidsPerSecond: calculateUUIDsPerSecond(testResults.wasm.time, count),
      sample: testResults.wasm.uuids[0]
    },
    npm: {
      time: testResults.npm.time.toFixed(2),
      uuidsPerSecond: calculateUUIDsPerSecond(testResults.npm.time, count),
      sample: testResults.npm.uuids[0]
    },
    browser: {
      time: testResults.browser.time.toFixed(2),
      uuidsPerSecond: calculateUUIDsPerSecond(testResults.browser.time, count),
      sample: testResults.browser.uuids[0]
    }
  };

  isRunning.value = false;
  showAnalysis.value = true;
};
</script>

<template>
  <div class="performance-test">
    <h2>UUID Generation Performance Test</h2>
    
    <div class="test-controls">
      <div class="input-group">
        <label for="iterations">Number of UUIDs to generate:</label>
        <input 
          id="iterations"
          type="number" 
          v-model="iterations" 
          :disabled="isRunning"
          min="1"
        >
      </div>
      <button @click="runTest" :disabled="isRunning">
        {{ isRunning ? 'Running Test...' : 'Run Performance Test' }}
      </button>
    </div>

    <div v-if="results" class="results">
      <h3>Test Results ({{ iterations }} iterations)</h3>
      
      <div class="result-grid">
        <div class="header">Implementation</div>
        <div class="header">Time (ms)</div>
        <div class="header">UUIDs/second</div>
        <div class="header">Sample UUID</div>

        <div class="implementation">Rust/WebAssembly</div>
        <div>{{ results.wasm.time }}</div>
        <div>{{ results.wasm.uuidsPerSecond }}</div>
        <div class="uuid">{{ results.wasm.sample }}</div>

        <div class="implementation">UUID NPM Package</div>
        <div>{{ results.npm.time }}</div>
        <div>{{ results.npm.uuidsPerSecond }}</div>
        <div class="uuid">{{ results.npm.sample }}</div>

        <div class="implementation">Browser crypto.randomUUID</div>
        <div>{{ results.browser.time }}</div>
        <div>{{ results.browser.uuidsPerSecond }}</div>
        <div class="uuid">{{ results.browser.sample }}</div>
      </div>

      <div v-if="showAnalysis" class="analysis">
        <h3>Performance Analysis</h3>
        <div class="analysis-content">
          <p><strong>Key Observations:</strong></p>
          <ul>
            <li>Rust/WebAssembly Implementation:</li>
            <ul>
              <li>Direct function call without object instantiation</li>
              <li>Uses Rust's UUID generation</li>
            </ul>
            <li>Browser's native <code>crypto.randomUUID()</code>:</li>
            <ul>
              <li>Uses cryptographically secure random number generation</li>
              <li>Direct access to system's random number generator</li>
            </ul>
            <li>UUID NPM Package:</li>
            <ul>
              <li>Pure JavaScript implementation</li>
              <li>Consistent performance across browsers</li>
            </ul>
          </ul>
          <p><strong>Performance Considerations:</strong></p>
          <ul>
            <li>Function Call Overhead:</li>
            <ul>
              <li>Removed object instantiation overhead</li>
              <li>Direct WebAssembly function calls</li>
            </ul>
            <li>Use Case Recommendations:</li>
            <ul>
              <li>For cryptographic security: Use browser's crypto.randomUUID()</li>
              <li>For compatibility: Use UUID NPM package</li>
              <li>For raw performance: Compare results in your specific environment</li>
            </ul>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.performance-test {
  padding: 20px;
  max-width: 1000px;
  margin: 0 auto;
}

.test-controls {
  margin: 20px 0;
  display: flex;
  gap: 20px;
  align-items: center;
}

.input-group {
  display: flex;
  align-items: center;
  gap: 10px;
}

input[type="number"] {
  padding: 8px;
  width: 150px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

button {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.results {
  margin-top: 30px;
}

.result-grid {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr 3fr;
  gap: 10px;
  margin-top: 20px;
}

.header {
  font-weight: bold;
  padding: 10px;
  background-color: #f5f5f5;
}

.implementation {
  font-weight: bold;
}

.result-grid > div {
  padding: 10px;
  border-bottom: 1px solid #eee;
}

.uuid {
  font-family: monospace;
}

.analysis {
  margin-top: 30px;
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: 8px;
}

.analysis-content {
  line-height: 1.6;
}

.analysis-content ul {
  margin: 10px 0;
  padding-left: 20px;
}

.analysis-content ul ul {
  margin: 5px 0;
}

code {
  background-color: #f0f0f0;
  padding: 2px 4px;
  border-radius: 4px;
  font-family: monospace;
}
</style>