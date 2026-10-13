# Orbitbar (app)

The Tauri 2 app: a Rust core and a vanilla TypeScript frontend. See the
[root README](../README.md) for installation and usage and
[docs/architecture.md](../docs/architecture.md) for architecture details.

## Stack

- **Tauri 2** (Rust and system webview): a transparent, frameless,
  always-on-top window. The bar renders in a webview and talks to Rust
  commands over `invoke()`.
- **Vanilla TypeScript and CSS**, no framework. Theme tokens arrive as CSS
  custom properties (`--ob-*`) computed from the Rust `ThemePalette`.

## Layout

```
app/
  index.html              entry page (bar, panel and menu containers)
  src/main.ts             loads config, applies theme, renders items, drag, collapse, menu
  src/section-view.ts     generic panel section view-model and rendering
  src/styles.css          bar, panel and menu styling driven by --ob-* properties
  src-tauri/
    src/lib.rs            Tauri commands, applying window placement, autostart sync
    src/placement.rs      pure window-placement rules (unit tested)
    src/config.rs         config schema, loader, theme palettes
    tauri.conf.json       window flags and bundle config
```

## Development

```sh
npm install
npm run tauri dev     # dev loop (Vite and reload)
npm run tauri build   # installers and bundles for current OS
```

## Checks

```sh
npx tsc --noEmit                                    # frontend type check (from app/)
npm run build                                       # type check and Vite build (from app/)
```

## Config

The per-user `config.json` is created on first launch. A minimal example:

```json
{
  "monitor": "primary",
  "margin": 8,
  "collapsed": false,
  "theme": "dark",
  "fontSize": 10.0,
  "autoStart": true,
  "items": [
    { "id": "home", "label": "home", "glyph": "0x2302", "action": "section:home", "tooltip": "Home / Dashboard" }
  ]
}
```

Two themes ship: `dark` (default) and `light`.
