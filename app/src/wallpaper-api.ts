import type {
  WallpaperEngineApi,
  WallpaperPropertyValue,
  WallpaperSystemInfo,
  WallpaperUserProperties,
} from "./types/wallpaper";

class WallpaperRuntimeBridge implements WallpaperEngineApi {
  private currentProperties: WallpaperUserProperties = {};
  private listeners: Map<string, Set<(data: unknown) => void>> = new Map();

  constructor() {
    this.initGlobalWindow();
  }

  private initGlobalWindow(): void {
    if (typeof window === "undefined") return;

    if (!window.wallpaperPropertyListener) {
      window.wallpaperPropertyListener = {
        applyUserProperties: (props) => {
          this.applyUserProperties(props);
        },
      };
    }

    window.wallpaper = this;
    window.wallpaperEngine = this;
  }

  public applyUserProperties(properties: WallpaperUserProperties): void {
    this.currentProperties = { ...this.currentProperties, ...properties };

    // Fire propertyChange listeners
    const set = this.listeners.get("propertyChange");
    if (set) {
      for (const cb of set) {
        try {
          cb(properties);
        } catch (err) {
          console.error("wallpaper-api: error in propertyChange listener", err);
        }
      }
    }

    // Call legacy listener if defined by third-party script
    if (
      window.wallpaperPropertyListener &&
      typeof window.wallpaperPropertyListener.applyUserProperties === "function" &&
      window.wallpaperPropertyListener.applyUserProperties !== this.applyUserProperties
    ) {
      try {
        window.wallpaperPropertyListener.applyUserProperties(properties);
      } catch (err) {
        console.error("wallpaper-api: error in window.wallpaperPropertyListener", err);
      }
    }
  }

  public on(event: string, callback: (data: unknown) => void): () => void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, new Set());
    }
    this.listeners.get(event)!.add(callback);

    return () => this.off(event, callback);
  }

  public off(event: string, callback: (data: unknown) => void): void {
    const set = this.listeners.get(event);
    if (set) {
      set.delete(callback);
    }
  }

  public emit(action: string, payload?: unknown): void {
    const set = this.listeners.get(action);
    if (set) {
      for (const cb of set) {
        cb(payload);
      }
    }
  }

  public getProperties(): WallpaperUserProperties {
    return { ...this.currentProperties };
  }

  public setProperty(name: string, value: WallpaperPropertyValue): void {
    const updated: WallpaperUserProperties = {
      [name]: { value },
    };
    this.applyUserProperties(updated);
  }

  public async getSystemInfo(): Promise<WallpaperSystemInfo> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      return await invoke<WallpaperSystemInfo>("get_wallpaper_environment_info");
    } catch {
      return {
        displayServer: "unknown",
        desktopEnvironment: "Browser Environment",
        supportsAnimatedWallpaper: true,
        recommendedAdapter: "Browser / Canvas",
        limitations: ["Running outside native Tauri desktop wrapper"],
      };
    }
  }

  public pause(): void {
    this.emit("pause", { paused: true });
  }

  public resume(): void {
    this.emit("resume", { paused: false });
  }
}

export const wallpaperBridge = new WallpaperRuntimeBridge();
