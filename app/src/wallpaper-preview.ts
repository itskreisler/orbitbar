import type { WallpaperUserProperties } from "./types/wallpaper";

export interface WallpaperPerformance {
  fps: number;
  cpuPercent: number;
  memoryMb: number;
  gpuPercent: number;
}

export class WallpaperPreviewComponent {
  private container: HTMLElement;
  private iframe: HTMLIFrameElement | null = null;
  public currentUrl = "";
  private isPaused = false;
  private isMuted = false;
  private volume = 1.0;

  constructor(container: HTMLElement) {
    this.container = container;
  }

  public loadWallpaperUrl(url: string): void {
    this.currentUrl = url;
    this.container.replaceChildren();

    const frameWrapper = document.createElement("div");
    frameWrapper.className = "wallpaper-preview-frame-wrapper";

    this.iframe = document.createElement("iframe");
    this.iframe.className = "wallpaper-preview-iframe";
    this.iframe.src = url;
    this.iframe.setAttribute("sandbox", "allow-scripts allow-same-origin");

    frameWrapper.appendChild(this.iframe);
    this.container.appendChild(frameWrapper);
    this.renderControlsAndMetrics();
  }

  public applyProperties(properties: WallpaperUserProperties): void {
    if (!this.iframe || !this.iframe.contentWindow) return;

    try {
      if (
        this.iframe.contentWindow.wallpaperPropertyListener &&
        typeof this.iframe.contentWindow.wallpaperPropertyListener.applyUserProperties === "function"
      ) {
        this.iframe.contentWindow.wallpaperPropertyListener.applyUserProperties(properties);
      }
    } catch {
      // Cross-origin fallback via postMessage
      this.iframe.contentWindow.postMessage({ type: "applyUserProperties", properties }, "*");
    }
  }

  private renderControlsAndMetrics(): void {
    const controls = document.createElement("div");
    controls.className = "preview-controls";

    const playPauseBtn = document.createElement("button");
    playPauseBtn.className = "preview-btn";
    playPauseBtn.textContent = this.isPaused ? "▶ Resume" : "⏸ Pause";
    playPauseBtn.addEventListener("click", () => {
      this.isPaused = !this.isPaused;
      playPauseBtn.textContent = this.isPaused ? "▶ Resume" : "⏸ Pause";
      if (this.iframe && this.iframe.contentWindow && this.iframe.contentWindow.wallpaper) {
        if (this.isPaused) this.iframe.contentWindow.wallpaper.pause();
        else this.iframe.contentWindow.wallpaper.resume();
      }
    });

    const muteBtn = document.createElement("button");
    muteBtn.className = "preview-btn";
    muteBtn.textContent = this.isMuted ? "🔇 Unmute" : "🔊 Mute";
    muteBtn.addEventListener("click", () => {
      this.isMuted = !this.isMuted;
      muteBtn.textContent = this.isMuted ? "🔇 Unmute" : "🔊 Mute";
    });

    const volSlider = document.createElement("input");
    volSlider.type = "range";
    volSlider.min = "0";
    volSlider.max = "1";
    volSlider.step = "0.05";
    volSlider.value = String(this.volume);
    volSlider.title = "Volume";
    volSlider.addEventListener("input", () => {
      this.volume = Number.parseFloat(volSlider.value);
    });

    controls.append(playPauseBtn, muteBtn, volSlider);

    const metricsCard = document.createElement("div");
    metricsCard.className = "preview-metrics";
    metricsCard.innerHTML = `
      <div class="metric-item"><strong>FPS:</strong> <span id="metric-fps">60</span></div>
      <div class="metric-item"><strong>CPU:</strong> <span id="metric-cpu">1.2%</span></div>
      <div class="metric-item"><strong>RAM:</strong> <span id="metric-ram">42 MB</span></div>
      <div class="metric-item"><strong>GPU:</strong> <span id="metric-gpu">2.8%</span></div>
    `;

    this.container.append(controls, metricsCard);
  }

  public updateMetrics(metrics: WallpaperPerformance): void {
    const fps = this.container.querySelector("#metric-fps");
    if (fps) fps.textContent = String(metrics.fps);

    const cpu = this.container.querySelector("#metric-cpu");
    if (cpu) cpu.textContent = `${metrics.cpuPercent.toFixed(1)}%`;

    const ram = this.container.querySelector("#metric-ram");
    if (ram) ram.textContent = `${metrics.memoryMb.toFixed(0)} MB`;

    const gpu = this.container.querySelector("#metric-gpu");
    if (gpu) gpu.textContent = `${metrics.gpuPercent.toFixed(1)}%`;
  }
}
