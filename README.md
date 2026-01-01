# SoundSwitch 🎧

<p align="center">
  <img src="public/app-icon.svg" width="128" height="128" alt="SoundSwitch Logo">
</p>

<p align="center">
  <strong>一款现代化、高性能的 Windows 音频设备切换工具</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Built%20by-Antigravity-blueviolet?style=flat-square" alt="Built by Antigravity">
  <img src="https://img.shields.io/badge/Tauri-v2-orange?style=flat-square&logo=tauri" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue.js-v3-green?style=flat-square&logo=vue.js" alt="Vue">
  <img src="https://img.shields.io/badge/Rust-Enabled-brown?style=flat-square&logo=rust" alt="Rust">
</p>

---

**SoundSwitch** 是一款专为 Windows 用户打造的音频输出设备管理工具。它极简、美观且高效，能够帮助你快速在耳机、扬声器和其他音频设备之间切换。

> 🤖 **特别说明**：本项目的所有代码、架构设计及 UI/UX/Icon 几乎都完全由 **Antigravity** AI 开发完成。

## ✨ 核心功能

- **⚡ 极速切换**：
  - **托盘操作**：左键点击系统托盘图标，即可在可用设备间循环切换。
  - **全局热键**：支持自定义全局快捷键（如 `Ctrl+Shift+S`），在任何应用中都能一键切换音频设备。
- **🎨 现代化界面**：
  - 基于 **TailwindCSS** 设计的精美 UI，支持流畅的微交互和动画。
  - 直观的设备列表，清晰展示当前活动设备。
- **🚀 高性能与轻量**：
  - 后端采用 **Rust** 编写，直接调用 Windows Core Audio API，响应迅速，资源占用极低。
  - 前端使用 **Tauri + Vue 3** 构建，提供原生般的流畅体验。
- **🛠️ 实用工具**：
  - **开机自启**：支持跟随系统启动，随时待命。
  - **智能托盘**：托盘图标会根据设备状态更新，右键菜单提供更多便捷选项。

## 📖 使用指南

### 1. 启动应用

安装并运行 SoundSwitch 后，它将自动最小化到系统托盘（屏幕右下角）。

### 2. 切换设备

- **方法 A（最快）**：直接**左键单击**托盘区的 SoundSwitch 图标，即可切换到下一个音频设备。
- **方法 B（可视化）**：右键点击托盘图标，选择 "Settings" 打开主界面，在列表中选择你想要的设备。
- **方法 C（快捷键）**：在设置中配置你的专属快捷键，按下即可切换。

### 3. 设置

在主界面点击右上角的设置图标（⚙️），你可以：

- ✅ 开启/关闭 **开机自大**（Launch at Login）。
- ⌨️ 设置或修改 **全局快捷键**。

## 🛠️ 开发与构建

如果你想自己在本地运行或修改本项目，请确保已安装 [Node.js](https://nodejs.org/) 和 [Rust](https://www.rust-lang.org/) 环境。

### 克隆项目

```bash
git clone https://github.com/varHarrie/SoundSwitch.git
cd SoundSwitch
```

### 安装依赖

```bash
npm install
```

### 开发模式（热重载）

```bash
npm run tauri dev
```

### 构建生产版本

```bash
npm run tauri build
```

## 📄 许可证

本项目开源，具体的许可协议请参考 LICENSE 文件。

---

<p align="center">Generated with ❤️ by Antigravity</p>
