<script setup>
import { ref } from 'vue';
import { generate_uuid } from 'uuid-wasm';
import PerformanceTest from './components/PerformanceTest.vue';
import BrowserInfo from './components/BrowserInfo.vue';

const activeTab = ref('generator');
const uuid = ref('');
const validationInput = ref('');
const validationResult = ref('');

const generateV4 = () => {
  uuid.value = generate_uuid();
};

const validateUuid = () => {
  // 简单的UUID格式验证
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  validationResult.value = uuidRegex.test(validationInput.value) ? 'Valid UUID' : 'Invalid UUID';
};
</script>

<template>
  <div class="container">
    <h1>UUID Tools</h1>
    
    <div class="tabs">
      <button 
        :class="{ active: activeTab === 'generator' }"
        @click="activeTab = 'generator'"
      >
        UUID Generator
      </button>
      <button 
        :class="{ active: activeTab === 'performance' }"
        @click="activeTab = 'performance'"
      >
        Performance Test
      </button>
      <button 
        :class="{ active: activeTab === 'browser' }"
        @click="activeTab = 'browser'"
      >
        Browser Info
      </button>
    </div>

          <div v-if="activeTab === 'generator'" class="tab-content">
            <div class="generator-section">
              <button @click="generateV4">Generate UUID v4</button>
              <div class="result">{{ uuid }}</div>
            </div>

      <div class="validation-section">
        <h3>UUID Validation</h3>
        <input v-model="validationInput" placeholder="Enter UUID to validate">
        <button @click="validateUuid">Validate</button>
        <div class="result" :class="{ valid: validationResult === 'Valid UUID', invalid: validationResult === 'Invalid UUID' }">
          {{ validationResult }}
        </div>
      </div>
    </div>

    <div v-else-if="activeTab === 'performance'" class="tab-content">
      <PerformanceTest />
    </div>
    <div v-else-if="activeTab === 'browser'" class="tab-content">
      <BrowserInfo />
    </div>
  </div>
</template>

<style>
.container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 20px;
  font-family: Arial, sans-serif;
}

.tabs {
  margin-bottom: 20px;
  border-bottom: 1px solid #ddd;
}

.tabs button {
  background-color: transparent;
  color: #333;
  padding: 10px 20px;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  margin: 0 5px;
}

.tabs button.active {
  border-bottom: 2px solid #4CAF50;
  color: #4CAF50;
}

.tab-content {
  padding: 20px 0;
}

button {
  background-color: #4CAF50;
  color: white;
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  margin: 5px;
}

button:hover {
  background-color: #45a049;
}

.result {
  margin-top: 10px;
  padding: 10px;
  background-color: white;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.validation-section {
  margin-top: 20px;
}

input {
  padding: 8px;
  width: 300px;
  margin-right: 10px;
}

.valid {
  color: green;
}

.invalid {
  color: red;
}

.tabs button:not(.active):hover {
  background-color: #f0f0f0;
}
</style>