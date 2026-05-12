StickyNotes — Simple pinned sticky notes
================================

Quick Start
--------
1. Extract all files to any directory.
2. Double-click StickyNotes.exe to run it (on the first launch of Windows, you may be prompted that WebView2 is required).
Included in Windows 10/11; please install if missing:
(https://developer.microsoft.com/microsoft-edge/webview2/)

Main functions
--------
- Pinned to the top of all windows for quick note-taking
- Light/Dark Theme
- Save the notebook as JSON, with customizable storage location.
- Supports local backup, export, and import (merge/overwrite)
- Customize window size, remember window position
- To close the window, you can choose to "minimize to background" or "close the program".

Data location
--------
- Note data: Default is %USERPROFILE%\StickyNotes\notes.json, which can be modified in "Settings → Note Location".
- Configuration file: %APPDATA%\StickyNotes\config.json

source code
------
This compressed package also contains the complete source code (src-tauri/, dist/).
Built on Tauri 1.6 + Rust, it's a single executable file of only ~1.8 MB.

License: MIT




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

License: MIT
