# Guía para Desarrolladores de Wallpapers

## Introducción

Este motor ofrece compatibilidad con fondos de pantalla de **Wallpaper Engine** y proporciona una API JavaScript moderna para fondos interactivos HTML5, CSS y JavaScript.

## Estructura de un Wallpaper

Cada wallpaper reside en su propio directorio con la siguiente estructura:

```text
my-wallpaper/
├── project.json      # Manifiesto declarativo de propiedades y metadatos
├── index.html        # Punto de entrada HTML
├── style.css         # Estilos y variables CSS
└── script.js         # Lógica interactiva e integración con la API JS
```

## Manifiesto `project.json`

```json
{
  "schema_version": 1,
  "title": "Interactive Starfield",
  "type": "web",
  "file": "index.html",
  "description": "Fondo interactivo con reloj y partículas",
  "author": "Desarrollador",
  "version": "1.0.0",
  "properties": {
    "customcolor": {
      "type": "color",
      "text": "Color de Fondo",
      "value": "#0f172a"
    },
    "star_count": {
      "type": "slider",
      "text": "Cantidad de Partículas",
      "min": 10,
      "max": 500,
      "step": 10,
      "value": 150
    },
    "show_clock": {
      "type": "checkbox",
      "text": "Mostrar Reloj",
      "value": true
    },
    "clock_format": {
      "type": "combo",
      "text": "Formato de Hora",
      "visible_if": "show_clock == true",
      "value": "24h",
      "options": [
        { "label": "24 Horas", "value": "24h" },
        { "label": "12 Horas", "value": "12h" }
      ]
    }
  }
}
```

## Los 22 Tipos de Propiedades Declarativas

1. `color`
2. `slider`
3. `checkbox`
4. `combo`
5. `textinput`
6. `textarea`
7. `number`
8. `file`
9. `directory`
10. `date`
11. `time`
12. `datetime`
13. `font`
14. `keybind`
15. `colorgradient`
16. `vector2`
17. `vector3`
18. `range`
19. `button`
20. `section`
21. `label`
22. `custom`

## API JavaScript y Escuchadores

### 1. Contrato Estándar Wallpaper Engine

```javascript
window.wallpaperPropertyListener = {
  applyUserProperties(properties) {
    if (properties.customcolor) {
      document.documentElement.style.setProperty("--wallpaper-bg", properties.customcolor.value);
    }
    if (properties.star_count) {
      updateStarCount(properties.star_count.value);
    }
  }
};
```

### 2. API Moderna `window.wallpaper`

```javascript
// Registrar eventos
const unsubscribe = window.wallpaper.on("propertyChange", (props) => {
  console.log("Propiedades actualizadas:", props);
});

// Modificar una propiedad dinámicamente
window.wallpaper.setProperty("star_count", 300);

// Consultar información del entorno/sistema
window.wallpaper.getSystemInfo().then((info) => {
  console.log("Servidor gráfico:", info.displayServer);
});

// Control de pausa y reanudación
window.wallpaper.on("pause", () => {
  stopAnimationLoop();
});
```
