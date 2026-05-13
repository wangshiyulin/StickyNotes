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
- Delete items that have been moved to the recycle bin; these can be restored or permanently deleted (including "empty the recycle bin").
- The "Start automatically on boot" switch has been added back in the settings.
- Note Categories/Tags — Tag notes (work, life, to-do), and filter by tag.
- Sticky Note Color Marking — Each sticky note can have a customizable background color (yellow/pink/blue/green) for visual categorization.
- Creation/Modification Time Display — A timestamp is displayed at the bottom of each note.
- Add a "Done" button to each note; once completed, it will move to the archive page.
- System tray icon — Remains in the taskbar tray after being closed; click to restore window.
- Start automatically on boot - Check the box to start with system in settings

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
- 删除先入回收站，可恢复或彻底删除（含「清空回收站」）
- 设置里加回了「开机自启」开关
- 便签分类/标签 — 给便签打标签（工作、生活、待办），可按标签筛选
- 便签颜色标记 — 每条便签可选背景色（黄/粉/蓝/绿），视觉分类
- 创建/修改时间显示 — 每条便签底部显示时间戳
- 给每个便签添加一个完成的按钮，完成之后会移动到归档页面
- 系统托盘图标 — 关闭后在任务栏托盘常驻，单击恢复窗口
- 开机自启 — 设置里勾选随系统启动

数据位置
--------
- 笔记数据：默认 %USERPROFILE%\StickyNotes\notes.json，可在「设置 → 笔记位置」修改
- 配置文件：%APPDATA%\StickyNotes\config.json

源代码
------
本压缩包同时包含完整源代码（src-tauri/、dist/）。
基于 Tauri 1.6 + Rust 构建，仅 ~1.8 MB 单文件可执行。

License: MIT
