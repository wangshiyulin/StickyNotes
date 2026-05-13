# 构建说明

## 前置
- Rust (https://rustup.rs)
- Windows: MSVC 工具链；或在 Linux/macOS 用 `cargo xwin` 交叉编译

## 编译
```
cd src-tauri
cargo build --release           # 本机平台
# 或交叉编译 Windows：
cargo install cargo-xwin
cargo xwin build --release --target x86_64-pc-windows-msvc
```
产物：`src-tauri/target/<target>/release/stickynotes(.exe)`

## 修改前端
直接编辑 `dist/index.html`（单文件，原生 HTML/CSS/JS），无需打包步骤。
