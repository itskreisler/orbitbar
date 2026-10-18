# Limitaciones Verificadas por Entorno de Escritorio

## Resumen de Compatibilidad

| Sistema Operativo / Servidor | Soporte Fondo Estático | Soporte Fondo Animado | Adaptador Recomendado | Limitaciones Notables |
| --- | --- | --- | --- | --- |
| **Windows (10 / 11)** | ✅ Completo | ✅ Completo | WorkerW / `SystemParametersInfoW` | El modo de alto contraste o temas modificados de terceros pueden alterar el z-order de WorkerW. |
| **Linux (X11)** | ✅ Completo | ✅ Completo | X11 Root Window / xwinwrap | Iconos gestionados por administradores de escritorio (Nautilus/Nemo) pueden cubrir el root window si no se configura la transparencia de capas. `feh` solo soporta imágenes estáticas. |
| **Linux (Wayland)** | ✅ Completo | ⚠️ Limitado por compositor | Wayland Layer Shell (`wlr-layer-shell`) / DBus API | El modelo de seguridad de Wayland impide la libre reasignación de padres de ventana (`SetParent`). Requiere protocolos específicos de layer-shell. `feh` no admite wallpapers animados. |
| **macOS** | ⚠️ Configuración Reservada | ⚠️ No implementado | NSWindowLevel | Configuración conservada en estado reservado y comentada en CI. |

## Detalles Técnicos por Plataforma

### Windows
- **Mecanismo:** Envío del mensaje no documentado `0x052C` a la ventana `Progman` mediante `SendMessageTimeoutW` para forzar la creación de la superficie `WorkerW` detrás de los iconos del escritorio.
- **Fondo estático:** `SystemParametersInfoW` con `SPI_SETDESKWALLPAPER`.

### Linux (X11)
- **Mecanismo:** Dibujado directo en el Root Window (`XSetWindowBackground`) o capa de ventana inferior utilizando utilidades tipo `xwinwrap`.
- **Fondo estático:** Integración con `gsettings` (`org.gnome.desktop.background.picture-uri`) y fallback `feh --bg-fill`.

### Linux (Wayland)
- **Mecanismo:** Implementación de extensiones `wlr-layer-shell-unstable-v1` asignando la ventana en la capa `ZWLR_LAYER_SHELL_V1_LAYER_BACKGROUND`.

### macOS
- **Mecanismo:** La configuración de compilación de macOS y los targets correspondientes se mantienen comentados en `.github/workflows/release.yml` según las directrices del proyecto.
