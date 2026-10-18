export type WallpaperPropertyValue =
  | string
  | number
  | boolean
  | [number, number]
  | [number, number, number]
  | string[]
  | Record<string, unknown>;

export interface WallpaperPropertyPayload {
  value: WallpaperPropertyValue;
}

export type WallpaperUserProperties = Record<string, WallpaperPropertyPayload>;

export interface WallpaperPropertyListener {
  applyUserProperties?: (properties: WallpaperUserProperties) => void;
  applyGeneralProperties?: (properties: Record<string, WallpaperPropertyPayload>) => void;
  userPropertiesChange?: (properties: WallpaperUserProperties) => void;
}

export interface WallpaperSystemInfo {
  displayServer: "windowsnative" | "x11" | "wayland" | "macosnative" | "unknown";
  desktopEnvironment: string;
  compositor?: string;
  supportsAnimatedWallpaper: boolean;
  recommendedAdapter: string;
  limitations: string[];
}

export interface WallpaperEngineApi {
  on: (event: string, callback: (data: unknown) => void) => () => void;
  off: (event: string, callback: (data: unknown) => void) => void;
  emit: (action: string, payload?: unknown) => void;
  getProperties: () => WallpaperUserProperties;
  setProperty: (name: string, value: WallpaperPropertyValue) => void;
  getSystemInfo: () => Promise<WallpaperSystemInfo>;
  pause: () => void;
  resume: () => void;
}

declare global {
  interface Window {
    wallpaperPropertyListener?: WallpaperPropertyListener;
    wallpaper?: WallpaperEngineApi;
    wallpaperEngine?: WallpaperEngineApi;
  }
}
