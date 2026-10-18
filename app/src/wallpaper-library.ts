export interface InstalledWallpaper {
  id: string;
  title: string;
  type: string;
  author: string;
  file: string;
  directory: string;
  previewUrl?: string;
}

export class WallpaperLibrary {
  private wallpapers: InstalledWallpaper[] = [];

  constructor() {
    this.initDefaultLibrary();
  }

  private initDefaultLibrary(): void {
    this.wallpapers = [
      {
        id: "sample-clock-particles",
        title: "Sample Interactive Clock & Particles",
        type: "web",
        author: "Wallpaper Engine Engine",
        file: "index.html",
        directory: "/wallpapers/sample-clock-particles",
      },
    ];
  }

  public getWallpapers(): InstalledWallpaper[] {
    return [...this.wallpapers];
  }

  public addWallpaper(wp: InstalledWallpaper): void {
    if (!this.wallpapers.some((w) => w.id === wp.id)) {
      this.wallpapers.push(wp);
    }
  }

  public renderLibraryView(
    onSelectWallpaper: (wp: InstalledWallpaper) => void,
    onImportWallpaper: () => void,
  ): HTMLElement {
    const root = document.createElement("div");
    root.className = "wallpaper-library-view";

    const header = document.createElement("div");
    header.className = "library-header";

    const title = document.createElement("h3");
    title.className = "library-title";
    title.textContent = "Wallpaper Library";

    const importBtn = document.createElement("button");
    importBtn.type = "button";
    importBtn.className = "inspector-button";
    importBtn.textContent = "+ Import Wallpaper";
    importBtn.addEventListener("click", onImportWallpaper);

    header.append(title, importBtn);
    root.appendChild(header);

    const list = document.createElement("div");
    list.className = "library-grid";

    for (const wp of this.wallpapers) {
      const card = document.createElement("div");
      card.className = "library-card";

      const cardTitle = document.createElement("div");
      cardTitle.className = "library-card-title";
      cardTitle.textContent = wp.title;

      const cardMeta = document.createElement("div");
      cardMeta.className = "library-card-meta";
      cardMeta.textContent = `${wp.type.toUpperCase()} · ${wp.author}`;

      card.append(cardTitle, cardMeta);
      card.addEventListener("click", () => onSelectWallpaper(wp));
      list.appendChild(card);
    }

    root.appendChild(list);
    return root;
  }
}
