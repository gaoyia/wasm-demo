<script setup>
import { get_browser_info } from 'uuid-wasm';
import { show_alert_message, get_random_number } from 'uuid-wasm';

const showAlert = () => {
  show_alert_message('This is an alert called from Rust WASM!');
};
const showRandomNumber = async () => {
  const randomStr = await get_random_number();
  alert(randomStr); // 显示如："JS随机数: 0.1234"
};
const showBrowserInfo = async () => {
  try {
    const info = await get_browser_info();
    const newWindow = window.open('', '_blank');
    
    newWindow.document.write(`
      <html>
        <head><title>Browser Info</title></head>
        <body>
          <h1>Browser Information</h1>
          <table border="1" style="border-collapse: collapse; width: 100%;">
            <tr><th style="padding: 8px; text-align: left;">Property</th><th style="padding: 8px; text-align: left;">Value</th></tr>
            <tr><td style="padding: 8px;">Current URL</td><td style="padding: 8px;">${info.url}</td></tr>
            <tr><td style="padding: 8px;">Page Title</td><td style="padding: 8px;">${info.title}</td></tr>
            <tr><td style="padding: 8px;">Screen Size</td><td style="padding: 8px;">${info.screenWidth}x${info.screenHeight}</td></tr>
            <tr><td style="padding: 8px;">Page Load Time</td><td style="padding: 8px;">${info.loadTime}ms</td></tr>
          </table>
        </body>
      </html>
    `);
    newWindow.document.close();
  } catch (error) {
    alert('获取浏览器信息失败: ' + error);
  }
};
</script>

<template>
  <div class="browser-info">
    <h2>Browser Information</h2>
    <p>Click the buttons below to interact with browser features.</p>
    <div class="button-group">
      <button @click="showBrowserInfo">Show Browser Info</button>
      <button @click="showAlert">Show Alert</button>
      <button @click="showRandomNumber">Get Random Number</button>
    </div>
  </div>
</template>

<style scoped>
.button-group {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}
</style>

<style scoped>
.browser-info {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

button {
  padding: 10px 20px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  margin-top: 20px;
}

button:hover {
  background-color: #45a049;
}
</style>