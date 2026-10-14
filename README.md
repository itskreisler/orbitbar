<div align="center">

<img src="app/src-tauri/icons/icon.png" width="96" alt="Orbitbar logo" />

# Orbitbar

*Always-on screen bar for monitoring AI coding agent token usage and costs*

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)
[![Tauri 2](https://img.shields.io/badge/Tauri-2.0-blue?style=flat-square&logo=tauri&logoColor=white)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Rust-1.77+-orange?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue?style=flat-square&logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)](#platform-support)

⭐ If you like this project, star it on GitHub!

[Overview](#overview) • [Features](#features) • [Download & Install](#download-and-install) • [Usage](#usage) • [Configuration](#configuration) • [Build from Source](#build-from-source)

</div>

<br>

| Dark Theme | Light Theme |
| --- | --- |
| ![Orbitbar in Dark theme](docs/images/orbitbar-dark.png) | ![Orbitbar in Light theme](docs/images/orbitbar-light.png) |

---

## Overview

**Orbitbar** is a desktop bar that stays attached to your screen edge, remaining visible above other windows. It provides real-time visibility into the token usage and cost of your local AI coding agents—including **Claude Code**, **Codex**, and **OpenCode**—overall, per project, and across customizable time ranges (Today, 7 days, 30 days, or current month).

Built with Rust and Tauri 2, Orbitbar is lightweight, respects user privacy by storing and reading data exclusively on your local machine, and requires no account or cloud services.

## Features

- **Always Accessible** - Positioned on your screen edge, draggable across monitors, collapse-able to a compact tab, and automatically adjusts panel placement based on available screen space.
- **Agent & Project Breakdown** - Tracks output tokens and costs per agent and per git repository project over multiple time windows.
- **Privacy-First & Local Only** - Reads existing session logs created by your agents directly on disk. No network calls, telemetry, or external accounts required.
- **Configurable Pricing & Overrides** - Includes built-in model pricing tables with optional user override files (`pricing.json`).
- **Extensible Actions** - Execute local programs, view command outputs inside the usage panel, or launch web links directly from configurable bar cells.
- **Light & Dark Themes** - Switch themes instantly from the context menu.

## Download and Install

Download the installer for your operating system from the latest [Releases](https://github.com/montesgp/orbitbar/releases/latest):

| Operating System | Recommended Installer | Alternative Packages |
| --- | --- | --- |
| **Windows** | `Orbitbar_<version>_x64-setup.exe` | `Orbitbar_<version>_x64_en-US.msi` |
| **macOS** | `Orbitbar_<version>_universal.dmg` | *(Supports Apple Silicon and Intel)* |
| **Linux** | `Orbitbar_<version>_amd64.AppImage` | `.deb` (Debian/Ubuntu), `.rpm` (Fedora/RHEL) |

> [!NOTE]
> AI agents are completely optional. Orbitbar automatically detects installed agents (Claude Code, Codex, OpenCode) and marks uninstalled ones accordingly.

> [!IMPORTANT]
> **First Run Instructions (Unsigned Binaries)**
> - **Windows:** SmartScreen may display a warning. Select **More info** $\rightarrow$ **Run anyway**.
> - **macOS:** Right-click `Orbitbar.app` and choose **Open**, or go to **System Settings > Privacy & Security** and click **Open Anyway**.
> - **Linux:** Make AppImage executable via `chmod +x Orbitbar_*.AppImage && ./Orbitbar_*.AppImage`, or install packages using your system package manager (`sudo apt install ./Orbitbar_*.deb`).

## Platform Support

| OS | Desktop Runtime | Configuration File Path | Autostart Mechanism |
| --- | --- | --- | --- |
| **Windows** | WebView2 *(Preinstalled on Win 10/11)* | `%APPDATA%\com.orbitbar.app\config.json` | Registry Run Entry |
| **macOS** | WKWebView | `~/Library/Application Support/com.orbitbar.app/config.json` | LaunchAgent |
| **Linux** | WebKitGTK 4.1 (`libwebkit2gtk-4.1-0`) | `~/.config/com.orbitbar.app/config.json` | XDG Autostart `.desktop` entry |

## Usage

- **Inspect Usage:** Left-click an agent cell on the bar to open its detail panel. Toggle time periods (Today, 7 days, 30 days, or current month) directly inside the open panel.
- **Open Menu:** Right-click the bar (or collapsed tab) or left-click the settings cell to access settings, switch themes, open config/pricing files, or exit.
- **Reposition:** Drag the bar anywhere across your monitors. Orbitbar snaps to usable screen bounds and saves its coordinates automatically.
- **Collapse/Expand:** Click the chevron icon at the top of the bar to collapse it into a discrete tab.

### Usage Panel Example

```text
Claude Code                                 Today ▾
------------------------------------------------------
output tokens   211,300      cost  ~$53.04 (API-equiv.)
------------------------------------------------------
project              output tokens        cost
my-app                     180,400       ~$45.10
api-service                 30,900        ~$7.94
```

> [!TIP]
> Cost entries marked with `~$X (API-equiv.)` are calculated using public API rates based on token usage. Plain `$X` represents costs reported directly by tools like OpenCode.

## Configuration

Orbitbar creates a default `config.json` file upon first launch.

### Main Configuration Fields

| Field | Type | Description |
| --- | --- | --- |
| `theme` | `string` | Theme setting: `"dark"` (default) or `"light"`. |
| `fontSize` | `number` | Base font size in pixels. |
| `monitor` | `string` | Target monitor for default placement (`"primary"` by default). |
| `margin` | `number` | Edge margin in pixels when positioned at screen edge. |
| `position` | `object` | `{ "x": number, "y": number }` coordinates of physical position. |
| `collapsed` | `boolean` | Starts application as a collapsed tab if `true`. |
| `autoStart` | `boolean` | Enables launch on system startup. |
| `showExamples` | `boolean` | Toggles display of example items. |
| `items` | `array` | Custom bar cell configurations. |

### Cell Actions

Configured bar items (`items`) support the following actions:

| Action Scheme | Behavior |
| --- | --- |
| `agent-usage` | Opens usage panel showing all agents. |
| `agent-usage:<agent>` | Opens usage panel filtered to specific agent (`claude`, `codex`, or `opencode`). |
| `run:<program> [args]` | Launches a program in background/detached mode. |
| `panel:<program> [args]` | Runs a command and displays standard output inside the panel. |
| `open:<url>` | Opens specified `http://` or `https://` URL in default browser. |
| `toggle-autostart` | Toggles system autostart state. |
| `edit-config` | Opens context menu to edit `config.json`. |

### Custom Pricing (`pricing.json`)

To override or add pricing entries for specific models, select **Open pricing file** from the context menu or create `pricing.json` alongside `config.json`.

```json
{
  "models": {
    "claude-3-7-sonnet-20250219": {
      "input_cost_per_m": 3.0,
      "output_cost_per_m": 15.0
    }
  }
}
```

## Data Sources

Orbitbar parses agent data directly from local storage paths:

| Agent | Source Path | Access Method |
| --- | --- | --- |
| **Claude Code** | `~/.claude/projects/**/*.jsonl` | Read-only JSONL logs |
| **Codex** | `~/.codex/sessions/**/*.jsonl` | Read-only JSONL logs |
| **OpenCode** | `~/.local/share/opencode/opencode.db` | Read-only SQLite database |

## Extensions

Orbitbar includes optional extensions for terminal integrations like **Herdr TUI** and **OmniRoute**. See [`extensions/README.md`](extensions/README.md) for details.

## Build from Source

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (v1.77+)

#### Linux Dependencies (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

### Build Instructions

```bash
# Navigate to app directory
cd app

# Install dependencies
npm ci

# Run in development mode with hot reload
npm run tauri dev

# Build production installers
npm run tauri build
```

Built installers are placed in `app/src-tauri/target/release/bundle/`.

## Roadmap

Planned updates and features are tracked on the [GitHub Issues](https://github.com/montesgp/orbitbar/issues?q=is%3Aissue+label%3Aenhancement) page. Upcoming milestones include in-app release update notifications and automated updates.
