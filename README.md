# WebAssembly UUID Generator

这是一个使用Rust和WebAssembly实现的UUID生成器。它提供了以下功能：

- 生成UUID v4（随机UUID）
- 生成Nil UUID（全零UUID）
- 验证UUID的有效性

## 功能特点

- 使用Rust的`uuid`库确保高质量的UUID生成
- WebAssembly实现确保高性能
- 简单直观的Web界面
- 包含UUID验证功能

## 运行说明

1. 确保已安装以下工具：
   - [Rust](https://rustup.rs/)
   - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
   - 任意现代浏览器

2. 构建项目：
   ```bash
   wasm-pack build --target web
   ```

3. 启动本地服务器（选择以下任一方式）：

   使用Python：
   ```bash
   # Python 3.x
   python -m http.server 8080
   ```

   使用Node.js：
   ```bash
   # 使用npx
   npx http-server
   ```

4. 在浏览器中访问：
   ```
   http://localhost:8080
   ```

## 使用方法

1. 点击"Generate UUID v4"按钮生成随机UUID
2. 点击"Generate Nil UUID"按钮生成全零UUID
3. 在验证框中输入UUID并点击"Validate"按钮检查其有效性

## 技术栈

- Rust
- WebAssembly (wasm-bindgen)
- HTML/JavaScript
- uuid crate

## 项目结构

```
.
├── src/
│   ├── lib.rs         # Rust实现代码
│   └── utils.rs       # 工具函数
├── pkg/               # 编译后的WebAssembly文件
├── Cargo.toml         # Rust项目配置
└── index.html         # Web界面
```

## 注意事项

- 确保浏览器支持WebAssembly
- 必须通过HTTP服务器访问，直接打开HTML文件将无法工作
- 首次加载可能需要几秒钟时间编译WebAssembly模块