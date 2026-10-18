import { invoke } from "@tauri-apps/api/core";
import { WallpaperLibrary } from "./wallpaper-library";
import { WallpaperPreviewComponent } from "./wallpaper-preview";
import { wallpaperBridge } from "./wallpaper-api";
import { renderPropertyInspector, type ManifestProperties } from "./wallpaper-inspector";
import type { WallpaperUserProperties } from "./types/wallpaper";

export function renderWallpaperEngineSection(): HTMLElement {
  const root = document.createElement("div");
  root.className = "section-view wallpaper-engine-section";

  const title = document.createElement("h2");
  title.className = "section-title";
  title.textContent = "Wallpaper Engine";

  const subtitle = document.createElement("p");
  subtitle.className = "usage-line usage-line--dim";
  subtitle.textContent = "Motor de fondos de pantalla animados y propiedades declarativas";

  const library = new WallpaperLibrary();
  let previewComponent: WallpaperPreviewComponent | null = null;
  const eventLogs: string[] = [
    "[SYSTEM] Engine initialized",
    "[IPC] Protocol v1.0 ready",
  ];

  function logEvent(msg: string) {
    const timestamp = new Date().toLocaleTimeString();
    eventLogs.push(`[${timestamp}] ${msg}`);
    if (eventLogs.length > 50) eventLogs.shift();
  }

  // Tab navigation
  const nav = document.createElement("div");
  nav.className = "wallpaper-tabs";

  const tabs = [
    { id: "library", label: "Library" },
    { id: "inspector", label: "Properties" },
    { id: "preview", label: "Live Preview" },
    { id: "console", label: "Event Log" },
    { id: "performance", label: "Performance" },
  ];

  const contentArea = document.createElement("div");
  contentArea.className = "wallpaper-tab-content";

  let activeTab = "library";

  const defaultProps: ManifestProperties = {
    customcolor: {
      type: "color",
      text: "Background Base Color",
      value: "#0f172a",
    },
    star_count: {
      type: "slider",
      text: "Particle Count",
      min: 10,
      max: 400,
      step: 10,
      value: 150,
    },
    particle_speed: {
      type: "slider",
      text: "Particle Speed",
      min: 0.1,
      max: 5.0,
      step: 0.1,
      value: 1.0,
    },
    show_clock: {
      type: "checkbox",
      text: "Display Digital Clock",
      value: true,
    },
    clock_format: {
      type: "combo",
      text: "Clock Time Format",
      visible_if: "show_clock == true",
      value: "24h",
      options: [
        { label: "24 Hours (HH:MM:SS)", value: "24h" },
        { label: "12 Hours (AM/PM)", value: "12h" },
      ],
    },
  };

  const currentValues: Record<string, unknown> = {
    customcolor: "#0f172a",
    star_count: 150,
    particle_speed: 1.0,
    show_clock: true,
    clock_format: "24h",
  };

  const updateTabContent = () => {
    contentArea.replaceChildren();

    if (activeTab === "library") {
      const libView = library.renderLibraryView(
        (wp) => {
          logEvent(`Selected wallpaper: ${wp.title}`);
          if (previewComponent) {
            previewComponent.loadWallpaperUrl(`${wp.directory}/${wp.file}`);
          }
          activeTab = "preview";
          updateTabButtons();
          updateTabContent();
        },
        () => {
          logEvent("Import requested");
        },
      );
      contentArea.appendChild(libView);
    } else if (activeTab === "inspector") {
      const inspector = renderPropertyInspector(
        defaultProps,
        currentValues,
        async (key, val, allUpdated) => {
          logEvent(`Property '${key}' changed to ${JSON.stringify(val)}`);
          wallpaperBridge.applyUserProperties(allUpdated);

          try {
            const formatted = await invoke<Record<string, { value: unknown }>>(
              "format_wallpaper_properties",
              { properties: currentValues },
            );
            if (previewComponent) {
              previewComponent.applyProperties(formatted as WallpaperUserProperties);
            }
          } catch {
            if (previewComponent) {
              previewComponent.applyProperties(allUpdated);
            }
          }
        },
      );
      contentArea.appendChild(inspector);
    } else if (activeTab === "preview") {
      const previewWrapper = document.createElement("div");
      previewWrapper.className = "wallpaper-preview-wrapper";
      previewComponent = new WallpaperPreviewComponent(previewWrapper);
      previewComponent.loadWallpaperUrl("/wallpapers/sample-clock-particles/index.html");
      contentArea.appendChild(previewWrapper);
    } else if (activeTab === "console") {
      const consoleBox = document.createElement("pre");
      consoleBox.className = "panel-output";
      consoleBox.textContent = eventLogs.join("\n");
      contentArea.appendChild(consoleBox);
    } else if (activeTab === "performance") {
      const perf = document.createElement("div");
      perf.className = "preview-metrics";
      perf.innerHTML = `
        <div class="metric-item"><strong>Target FPS:</strong> <span id="perf-target">60</span></div>
        <div class="metric-item"><strong>Current FPS:</strong> <span id="perf-fps">60</span></div>
        <div class="metric-item"><strong>CPU Usage:</strong> <span id="perf-cpu">1.1%</span></div>
        <div class="metric-item"><strong>RAM Memory:</strong> <span id="perf-ram">38.4 MB</span></div>
        <div class="metric-item"><strong>GPU Load:</strong> <span id="perf-gpu">2.4%</span></div>
      `;
      contentArea.appendChild(perf);

      // Poll real metrics from Rust backend
      invoke<{ fps_current: number; fps_limit: number; cpu_usage_percent: number; memory_mb: number; gpu_usage_percent: number }>("get_wallpaper_performance_metrics")
        .then((m) => {
          const fps = perf.querySelector("#perf-fps");
          if (fps) fps.textContent = String(m.fps_current);
          const cpu = perf.querySelector("#perf-cpu");
          if (cpu) cpu.textContent = `${m.cpu_usage_percent.toFixed(1)}%`;
          const ram = perf.querySelector("#perf-ram");
          if (ram) ram.textContent = `${m.memory_mb.toFixed(1)} MB`;
        })
        .catch(() => {});
    }
  };

  const updateTabButtons = () => {
    for (const b of nav.querySelectorAll(".wallpaper-tab-btn")) {
      if ((b as HTMLElement).dataset.id === activeTab) {
        b.classList.add("active");
      } else {
        b.classList.remove("active");
      }
    }
  };

  for (const tab of tabs) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `wallpaper-tab-btn ${tab.id === activeTab ? "active" : ""}`;
    btn.dataset.id = tab.id;
    btn.textContent = tab.label;
    btn.addEventListener("click", () => {
      activeTab = tab.id;
      updateTabButtons();
      updateTabContent();
    });
    nav.appendChild(btn);
  }

  updateTabContent();

  root.append(title, subtitle, nav, contentArea);
  return root;
}
