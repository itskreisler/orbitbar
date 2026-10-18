# Auditoría del Repositorio Existente (Orbitbar / Wallpaper Engine)

Fecha de auditoría: 2025

## 1. Estructura General del Proyecto

El repositorio corresponde a **Orbitbar** (v0.2.0), una aplicación de escritorio multiplataforma desarrollada sobre **Tauri 2** (backend en Rust) y **TypeScript/Vite** (frontend HTML/CSS/JS).

### Árbol principal de directorios:

```text
.
├── .github/
│   └── workflows/          # CI y pipelines de release (Windows, macOS, Linux)
├── app/
│   ├── src/                # Frontend (TypeScript, CSS, HTML)
│   ├── src-tauri/          # Core backend en Rust y configuración de Tauri
│   ├── index.html          # Punto de entrada HTML
│   ├── package.json        # Configuración de dependencias NPM
│   ├── tsconfig.json       # Configuración de TypeScript
│   └── vite.config.ts      # Configuración del empaquetador Vite
├── docs/
│   ├── architecture.md     # Documentación de arquitectura existente
│   └── audit.md            # Este informe de auditoría
├── AGENTS.md               # Instrucciones de agentes y convenciones del repositorio
└── README.md               # Manual de usuario y configuración
```

## 2. Inspección de Archivos de Configuración

### `app/src-tauri/Cargo.toml`
- **Nombre:** `orbitbar` / `orbitbar_lib`
- **Edición Rust:** 2021
- **Dependencias clave:**
  - `tauri` (v2)
  - `tauri-plugin-autostart` (v2)
  - `tauri-plugin-opener` (v2)
  - `tauri-plugin-single-instance` (v2)
  - `serde`, `serde_json` (v1)
  - `dirs` (v7), `chrono` (v0.4), `rusqlite` (v0.40)
  - Windows target: `windows-sys` (Win32_UI_WindowsAndMessaging)
  - Dev dependencies: `tempfile` (v3)

### `app/src-tauri/tauri.conf.json`
- **Identifier:** `com.orbitbar.app`
- **Ventana principal ("main"):**
  - Dimensiones iniciales: 72x400
  - `decorations: false`, `transparent: true`, `alwaysOnTop: true`, `skipTaskbar: true`, `resizable: false`.
- **Bundling targets:** All (AppImage, deb, rpm para Linux; exe, msi para Windows; dmg universal para macOS).

### `app/package.json`
- **Framework:** TypeScript + Vite
- **Dependencias:** `@tauri-apps/api` (^2), `@tauri-apps/plugin-autostart` (^2.6.0), `@tauri-apps/plugin-opener` (^2)
- **Scripts:** `dev`, `build` (`tsc && vite build`), `preview`, `tauri`.

## 3. Frontend, Comandos Rust y Eventos Tauri Existentes

### Backend Rust (`app/src-tauri/src/`)
- `main.rs`: Punto de entrada binario que llama a `lib::run()`.
- `lib.rs`: Definición de la aplicación Tauri, gestor de eventos de inicio e invocación de comandos:
  - `get_config`: Carga configuración y paleta de temas.
  - `save_config`: Persiste la configuración del usuario.
  - `get_config_path`: Retorna la ruta completa a `config.json`.
  - `run_command`: Ejecuta aplicaciones/rutas configuradas mediante la acción `run:`.
  - `run_panel_command`: Ejecuta comandos de panel (`panel:`) capturando la salida en segundo plano con timeout y límite de búfer.
  - `place_window`: Reposiciona y redistribuye la ventana según espacio disponible en monitores sin mover la barra en pantalla.
  - `plan_window`: Calcula la disposición prevista antes del redimensionado.
  - `quit_app`: Cierre seguro de la aplicación mediante `app.exit(0)`.
- `config.rs`: Estructura `AppConfig`, temas `dark` y `light`, persistencia en disco por OS.
- `placement.rs`: Algoritmos puros de posicionamiento de ventanas y alineación respecto a monitores.
- `launch.rs`: Resolución e higienización de ejecutables y argumentos para acciones `run:` y `panel:`.
- `panel.rs`: Captura aislada de comandos CLI para el panel lateral.

### Frontend TypeScript (`app/src/`)
- `main.ts`: Lógica de renderizado del DOM, escuchadores de eventos IPC, posicionamiento, menús contextuales y paneles laterales.
- `placement-view.ts`: Operaciones auxiliares de conversión entre píxeles físicos y CSS.
- `section-view.ts`: Modelos de vista para secciones predefinidas del panel.
- `panel-output-view.ts`: Formateador de salida para comandos de terminal ejecutados.
- `styles.css`: Estilos visuales basados en variables CSS (`--ob-*`).

## 4. Scripts de Compilación y Empaquetado

- **Instalación de dependencias frontend:** `npm ci` (dentro de `app/`).
- **Compilación frontend:** `npm run build` (`tsc && vite build`).
- **Desarrollo:** `npm run tauri dev`.
- **Empaquetado nativo:** `npm run tauri build` (genera binarios e instaladores en `app/src-tauri/target/release/bundle/`).
- **Pruebas backend:** `cargo test` (dentro de `app/src-tauri/`).

## 5. Pruebas Automatizadas Existentes

- Ejecución de `cargo test`: **95 pruebas unitarias exitosas** (probando configuración, autostart, resolución de comandos, captura de panel y matemáticas de posicionamiento).
- Ejecución de `npm run build`: **Exitoso** (compilación TypeScript y generación de bundle Vite sin errores).

## 6. Targets Evaluados

- **Windows:** Target soportado activamente con `windows-sys` para manipular posiciones de ventanas nativas (`SetWindowPos`).
- **Linux:** Target soportado activamente mediante WebKitGTK / GTK3 y `tao` (X11 / Wayland).
- **macOS:** Target preservado y comentado en los flujos de CI/CD, listo para habilitación cuando se compila en entornos darwin.

## 7. Limitaciones Arquitectónicas Detectadas

1. **Monolito de UI simple:** La plantilla actual está configurada para una ventana principal de barra flotante (`main`) con panel lateral extensible.
2. **Sin aislamiento de contenido de terceros:** El frontend actual ejecuta código plano en una única vista web view.
3. **Falta de capa de integración nativa con el fondo de escritorio:** La ventana principal tiene la bandera `alwaysOnTop: true`, opuesta a un wallpaper que debe ser dibujado detrás de los iconos del escritorio (`WorkerW` en Windows, `layer-shell` / `root window` en Linux).
4. **Sin motor de renderizado ni pipeline multimedia:** No existe aún abstracción para reproducción de vídeo en bucle, canvas de partículas, ni protocolo bidireccional de propiedades.

## 8. Arquitectura Propuesta Compatible

Sin romper la estructura ni eliminar las funcionalidades de Orbitbar, el motor de wallpapers se integra mediante el nuevo módulo `app/src-tauri/src/wallpaper/` y componentes frontend dedicados:

1. **`wallpaper::manifest`**: Parser y validador de manifiestos declarativos (`project.json` / `wallpaper.json`).
2. **`wallpaper::properties`**: Modelo de datos declarativo para las 22 propiedades configurables con esquemas versionados y visibilidad condicional.
3. **`wallpaper::protocol`**: Protocolo JSON de mensajes IPC versionado (`version`, `id`, `type`, `name`, `payload`).
4. **`wallpaper::desktop`**: Adaptadores específicos para Windows (`WorkerW` / `SystemParametersInfoW`) y Linux (X11 / Wayland compositor check & background layering).
5. **`wallpaper::runtime` & `media`**: Administrador de ciclo de vida de wallpapers HTML, imágenes y vídeo en bucle, con límites de recursos (FPS, CPU/RAM budgets).
6. **API Pública JS y Editor Integrado:** Inyección de `window.wallpaperPropertyListener.applyUserProperties` y `window.wallpaper`, acompañada por la extensión de la barra/panel de Orbitbar con visor de biblioteca, editor de manifiestos, inspector de propiedades e indicadores de rendimiento.
