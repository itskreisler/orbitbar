# Orbitbar

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

Orbitbar is a clean, minimal desktop bar template built with Tauri 2 (Rust backend + TypeScript/Vite frontend). It stays where you put it on your screen, always visible above your other windows, and serves as a modular boilerplate for building custom desktop widgets, side panels, and launcher tools. It runs on Windows, macOS and Linux.

| Dark | Light |
| --- | --- |
| ![Orbitbar in Dark theme](docs/images/orbitbar-dark.png) | ![Orbitbar in Light theme](docs/images/orbitbar-light.png) |

## What you get

- **A bar that is always there.** It starts at the right edge of your primary
  monitor, can be dragged anywhere on any monitor and stays where you drop it,
  collapses to a small tab, and opens a side panel and a context menu beside it. Only one Orbitbar runs at a time.
- **Customizable panel views.** Renders clean, modular section views and command output panels.
- **Local & lightweight.** Orbitbar relies purely on local configuration and optional shell commands. It needs no external account, sends no telemetry, and makes no network calls.
- **Light and dark themes**, switched live from the bar's menu.
- **One JSON config file** for items, themes, autostart and window placement.

## Download and install

Get the latest installers from the
[Releases page](https://github.com/montesgp/orbitbar/releases/latest) and pick
the file for your OS:

| OS | File to pick |
| --- | --- |
| Windows | `Orbitbar_<version>_x64-setup.exe` (recommended) or `Orbitbar_<version>_x64_en-US.msi` |
| macOS | `Orbitbar_<version>_universal.dmg` (Apple silicon and Intel) |
| Linux | `Orbitbar_<version>_amd64.AppImage` (run it directly), `.deb` (Debian, Ubuntu) or `.rpm` (Fedora, openSUSE) |

### First run: unsigned installers

The installers are not code-signed, so each OS asks for confirmation the
first time you open them:

- **Windows:** SmartScreen shows "Windows protected your PC". Choose
  **More info**, then **Run anyway**.
- **macOS:** Gatekeeper blocks the app on first launch. Right-click
  `Orbitbar.app` and choose **Open**, then confirm; or open **System Settings
  > Privacy & Security** and choose **Open Anyway**.
- **Linux:** make the AppImage executable and run it:
  `chmod +x Orbitbar_*.AppImage && ./Orbitbar_*.AppImage`.

### Per-OS details

| | Windows | macOS | Linux |
| --- | --- | --- | --- |
| Runtime | WebView2, preinstalled on Windows 10 and 11 | WKWebView, part of macOS | WebKitGTK 4.1 (`libwebkit2gtk-4.1-0`), installed as a dependency |
| Config | `%APPDATA%\com.orbitbar.app\config.json` | `~/Library/Application Support/com.orbitbar.app/config.json` | `~/.config/com.orbitbar.app/config.json` |
| Autostart | A per-user Registry Run entry | A LaunchAgent | An XDG autostart `.desktop` entry |

## Build from source

To build your own binary, install Node.js, [Rust](https://rustup.rs/) and the build prerequisites for your OS:

```sh
cd app
npm ci
npm run tauri build   # installer or app bundle for the current OS
```

Use `npm run tauri dev` instead to run with hot reload while you work on it.

Output installers land under `app/src-tauri/target/release/bundle/`.

## Using Orbitbar

- **Click a cell** to open its section view in the side panel. Click the same cell again, press Escape, or use the close button to close it.
- **Right-click the bar** (or the collapsed tab), or **left-click the settings cell**, for the menu: edit `config.json`, collapse or expand, reload the config without a restart, switch between Light and Dark, turn **Start with system** or **Show example action** on or off, or quit.
- **Drag the bar** to move it anywhere; it stays where you drop it, keeps inside the monitor's usable area (not under the taskbar) and comes back there after a restart.

## Configuration

Orbitbar creates `config.json` with defaults on first launch and rewrites it when you change a setting from the app.

| Field | Meaning |
| --- | --- |
| `theme` | `dark` (default) or `light`. Any other value resolves to `dark`. |
| `fontSize` | Base font size in pixels. |
| `monitor` | Monitor for the first placement (`primary` by default). |
| `margin` | Gap in pixels between the bar and the right screen edge when placed without saved position. |
| `position` | Where the bar was dropped: `{ "x": 1836, "y": 340 }`, its top-left corner in physical screen pixels. |
| `collapsed` | Whether the bar starts as the small tab. |
| `autoStart` | Start Orbitbar at login (on by default). |
| `showExamples` | Show example cells (items flagged `"example": true`). |
| `items` | The cells on the bar, each with `id`, `glyph`, `label`, `tooltip` and `action`. |

Item actions:

| Action | What it does |
| --- | --- |
| `section:<name>` | Opens a section view in the side panel (e.g., `section:home`, `section:custom`). |
| `toggle-autostart` | Turns autostart on or off and updates the OS entry. |
| `edit-config` | Opens the bar's menu; "Edit config" opens `config.json` in your default editor. |
| `open:<url>` | Opens an `http://` or `https://` URL in your default browser. |
| `run:<program> [args]` | Starts a program with its arguments when you click the cell. |
| `panel:<program> [args]` | Runs a program when you click the cell and shows its output in the side panel. |

Example config item:

```json
{ "id": "home", "glyph": "0x2302", "label": "home", "tooltip": "Home / Dashboard", "action": "section:home" }
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md).

## License

[MIT](LICENSE)
