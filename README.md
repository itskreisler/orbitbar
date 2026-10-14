<div align="center">

<img src="app/src-tauri/icons/icon.png" alt="Orbitbar Logo" width="96" height="96" />

# Orbitbar

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Tauri 2](https://img.shields.io/badge/Tauri-v2-FFC13B?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)](https://github.com/montesgp/orbitbar/releases/latest)

⭐ If you like this project, star it on GitHub!

[Overview](#overview) • [Features](#features) • [Download & Install](#download--install) • [Usage](#usage) • [Configuration](#configuration) • [Build from Source](#build-from-source)

<br/>

| Dark Theme | Light Theme |
| --- | --- |
| ![Orbitbar in Dark theme](docs/images/orbitbar-dark.png) | ![Orbitbar in Light theme](docs/images/orbitbar-light.png) |

</div>

## Overview

**Orbitbar** is a clean, minimal desktop bar template built with [Tauri 2](https://tauri.app) (Rust backend + TypeScript/Vite frontend). It sits persistently on your screen above other windows and serves as a modular boilerplate for building custom desktop widgets, side panels, and launcher tools across Windows, macOS, and Linux.

## Features

- **Always Visible Bar:** Stays above other windows, collapses to a minimal tab, drag and place anywhere on screen with automatic workspace clamping.
- **Side Panel & Commands:** Renders clean modular section views and command output panels directly beside the bar.
- **Local & Lightweight:** Operates purely on local configuration without accounts, network calls, or telemetry.
- **Theme Support:** Switch between Light and Dark themes live with instant CSS custom property updates.
- **Single Config File:** Pure JSON configuration (`config.json`) for items, layout, autostart, and themes.
- **Single Instance:** Built-in single instance lock ensuring only one Orbitbar instance runs at a time.

## Download & Install

Get the latest installer binaries from the [Releases page](https://github.com/montesgp/orbitbar/releases/latest):

| OS | Recommended Package |
| --- | --- |
| **Windows** | `Orbitbar_<version>_x64-setup.exe` (or `Orbitbar_<version>_x64_en-US.msi`) |
| **macOS** | `Orbitbar_<version>_universal.dmg` (Universal binary for Apple Silicon & Intel) |
| **Linux** | `Orbitbar_<version>_amd64.AppImage`, `.deb` (Debian/Ubuntu), or `.rpm` (Fedora/openSUSE) |

> [!IMPORTANT]
> **First run: Unsigned Installers**
>
> The installers are currently not code-signed, requiring initial OS confirmation:
> - **Windows:** SmartScreen displays *"Windows protected your PC"*. Select **More info** $\rightarrow$ **Run anyway**.
> - **macOS:** Gatekeeper blocks unsigned apps on first launch. Right-click `Orbitbar.app` and choose **Open**, or go to **System Settings** $\rightarrow$ **Privacy & Security** $\rightarrow$ **Open Anyway**.
> - **Linux:** Make the AppImage executable: `chmod +x Orbitbar_*.AppImage && ./Orbitbar_*.AppImage`.

### Platform Specifications

| Feature | Windows | macOS | Linux |
| --- | --- | --- | --- |
| **Web Runtime** | WebView2 | WKWebView | WebKitGTK 4.1 (`libwebkit2gtk-4.1-0`) |
| **Config Location** | `%APPDATA%\com.orbitbar.app\config.json` | `~/Library/Application Support/com.orbitbar.app/config.json` | `~/.config/com.orbitbar.app/config.json` |
| **Autostart Mechanism** | Per-user Registry Run entry | LaunchAgent | XDG autostart `.desktop` entry |

## Usage

- **Click a cell:** Opens its corresponding section view in the side panel. Click the same cell again, press `Escape`, or click the panel close button to dismiss.
- **Bar Context Menu:** Right-click the bar (or left-click the settings cell) to edit `config.json`, toggle collapse, reload config, switch Light/Dark theme, or toggle autostart.
- **Repositioning:** Click and drag the bar to place it anywhere on screen. Dropping saves its physical screen coordinates to `config.json`.

## Configuration

Orbitbar automatically generates a default `config.json` on first launch and updates it when settings change via the context menu.

### Config Options

| Field | Description | Default |
| --- | --- | --- |
| `theme` | Theme mode (`dark` or `light`). | `"dark"` |
| `fontSize` | Base UI font size in pixels. | `13` |
| `monitor` | Monitor for initial placement (`primary` or monitor index). | `"primary"` |
| `margin` | Pixel gap between bar and screen edge when default placed. | `16` |
| `position` | Physical pixel top-left corner position object (`{ "x": 1836, "y": 340 }`). | `null` |
| `collapsed` | Whether the bar starts as a collapsed tab. | `false` |
| `autoStart` | Launch Orbitbar automatically on system boot. | `true` |
| `showExamples` | Display example items flagged with `"example": true`. | `true` |
| `items` | Array of item cell configuration objects on the bar. | `[...]` |

### Action Protocol

| Action Format | Description |
| --- | --- |
| `section:<name>` | Opens a custom view component in the side panel (e.g. `section:home`). |
| `toggle-autostart` | Toggles system autostart configuration. |
| `edit-config` | Opens `config.json` in your system default text editor. |
| `open:<url>` | Opens the target URL in your default browser. |
| `run:<program> [args]` | Executes a background system binary or script. |
| `panel:<program> [args]` | Executes a command and captures/renders its output in the side panel. |

> [!TIP]
> Example item entry for `config.json`:
> ```json
> {
>   "id": "home",
>   "glyph": "0x2302",
>   "label": "home",
>   "tooltip": "Home / Dashboard",
>   "action": "section:home"
> }
> ```

## Build from Source

### Prerequisites

Ensure you have [Node.js](https://nodejs.org) (v18+), [Rust](https://rustup.rs), and OS build tools installed (e.g., `build-essential` and `libwebkit2gtk-4.1-dev` on Linux).

### Build Commands

```bash
# Navigate to the app directory
cd app

# Install dependencies
npm ci

# Run in development mode with live reload
npm run tauri dev

# Build production bundle / installer
npm run tauri build
```

Production installers and bundles are generated in `app/src-tauri/target/release/bundle/`.
