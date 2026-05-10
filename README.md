StickyNotes — 简约置顶便签
================================

快速开始
--------
1. 解压全部文件到任意目录
2. 双击 StickyNotes.exe 运行（首次启动 Windows 可能提示需要 WebView2 运行时，
   Windows 10/11 自带；若缺失请安装：
   https://developer.microsoft.com/microsoft-edge/webview2/）

主要功能
--------
- 置顶在所有窗口最上层，随手记录
- 浅色 / 深色主题
- 笔记本地保存为 JSON，可自定义存储位置
- 支持本地备份、导出、导入（合并 / 覆盖）
- 自定义窗口大小、记忆窗口位置
- 关闭窗口可选「最小化到后台」或「关闭程序」

数据位置
--------
- 笔记数据：默认 %USERPROFILE%\StickyNotes\notes.json，可在「设置 → 笔记位置」修改
- 配置文件：%APPDATA%\StickyNotes\config.json

源代码
------
本压缩包同时包含完整源代码（src-tauri/、dist/）。
基于 Tauri 1.6 + Rust 构建，仅 ~1.8 MB 单文件可执行。

从源码构建（需 Rust + Windows 工具链或 cargo-xwin）：

    cd src-tauri
    cargo build --release --target x86_64-pc-windows-msvc

License: MIT
