/**
 * View-model and rendering logic for generic sections/pages in the side panel.
 * Designed as a clean template for building custom UI sections.
 */

export interface SectionItem {
  label: string;
  value: string;
  detail?: string;
}

export interface SectionCardViewModel {
  title: string;
  description?: string;
  badge?: string;
  items?: SectionItem[];
}

export interface SectionViewModel {
  id: string;
  title: string;
  subtitle?: string;
  cards: SectionCardViewModel[];
}

/**
 * Checks whether an action string is a section panel action (e.g., "section:home").
 */
export function isSectionAction(action: string): boolean {
  return action === "section" || action.startsWith("section:");
}

/**
 * Extracts the section name from a "section:<name>" action.
 */
export function sectionFromAction(action: string): string {
  const idx = action.indexOf(":");
  if (idx === -1) return "home";
  return action.slice(idx + 1) || "home";
}

/**
 * Returns a template view model for a given section ID.
 */
export function buildSectionViewModel(sectionId: string): SectionViewModel {
  if (sectionId === "custom") {
    return {
      id: "custom",
      title: "Custom Section",
      subtitle: "Template for custom content",
      cards: [
        {
          title: "Custom Feature",
          description: "This is a custom section card ready for your business logic.",
          badge: "Active",
          items: [
            { label: "Status", value: "Ready" },
            { label: "Type", value: "Template Component" },
          ],
        },
      ],
    };
  }

  // Default / "home" section template
  return {
    id: sectionId || "home",
    title: sectionId === "home" ? "Dashboard" : sectionId.toUpperCase(),
    subtitle: "Welcome to your template application",
    cards: [
      {
        title: "Overview",
        description: "Your base Orbitbar application template is set up and clean.",
        items: [
          { label: "Framework", value: "Tauri 2 + Vite + TS" },
          { label: "State", value: "Clean Base Template" },
        ],
      },
    ],
  };
}

/**
 * Renders a SectionViewModel into a container DOM element.
 */
export function renderSectionView(vm: SectionViewModel): HTMLElement {
  const container = document.createElement("div");
  container.className = "section-view";

  const header = document.createElement("div");
  header.className = "section-header";

  const title = document.createElement("h2");
  title.className = "section-title";
  title.textContent = vm.title;
  header.appendChild(title);

  if (vm.subtitle) {
    const subtitle = document.createElement("p");
    subtitle.className = "section-subtitle";
    subtitle.textContent = vm.subtitle;
    header.appendChild(subtitle);
  }

  container.appendChild(header);

  for (const cardVm of vm.cards) {
    const card = document.createElement("div");
    card.className = "section-card";

    const cardTitle = document.createElement("div");
    cardTitle.className = "section-card-title";
    cardTitle.textContent = cardVm.title;

    if (cardVm.badge) {
      const badge = document.createElement("span");
      badge.className = "section-badge";
      badge.textContent = cardVm.badge;
      cardTitle.appendChild(badge);
    }
    card.appendChild(cardTitle);

    if (cardVm.description) {
      const desc = document.createElement("div");
      desc.className = "section-card-desc";
      desc.textContent = cardVm.description;
      card.appendChild(desc);
    }

    if (cardVm.items && cardVm.items.length > 0) {
      const itemsList = document.createElement("div");
      itemsList.className = "section-items";
      for (const item of cardVm.items) {
        const row = document.createElement("div");
        row.className = "section-item-row";

        const label = document.createElement("span");
        label.className = "section-item-label";
        label.textContent = item.label;
        row.appendChild(label);

        const val = document.createElement("span");
        val.className = "section-item-value";
        val.textContent = item.value;
        row.appendChild(val);

        itemsList.appendChild(row);
      }
      card.appendChild(itemsList);
    }

    container.appendChild(card);
  }

  return container;
}
