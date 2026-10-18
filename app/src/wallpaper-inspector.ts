import type { WallpaperUserProperties } from "./types/wallpaper";

export interface PropertyOpt {
  label: string;
  value: unknown;
}

export interface PropertyDef {
  type: string;
  text: string;
  description?: string;
  value?: unknown;
  min?: number;
  max?: number;
  step?: number;
  options?: PropertyOpt[];
  visible_if?: string;
  enabled?: boolean;
}

export interface ManifestProperties {
  [key: string]: PropertyDef;
}

export function renderPropertyInspector(
  properties: ManifestProperties,
  currentValues: Record<string, unknown>,
  onChange: (key: string, value: unknown, allUpdated: WallpaperUserProperties) => void,
): HTMLElement {
  const container = document.createElement("div");
  container.className = "wallpaper-inspector";

  for (const [key, prop] of Object.entries(properties)) {
    if (prop.visible_if && !evaluateCondition(prop.visible_if, currentValues)) {
      continue; // Skip hidden properties based on conditional visibility
    }

    const fieldGroup = document.createElement("div");
    fieldGroup.className = "inspector-field";

    const label = document.createElement("label");
    label.className = "inspector-label";
    label.textContent = prop.text || key;
    if (prop.description) {
      label.title = prop.description;
    }
    fieldGroup.appendChild(label);

    const val = currentValues[key] !== undefined ? currentValues[key] : prop.value;

    const inputEl = createPropertyInput(prop, val, (newVal) => {
      currentValues[key] = newVal;
      const formatted: WallpaperUserProperties = {};
      for (const [k, v] of Object.entries(currentValues)) {
        formatted[k] = { value: v as any };
      }
      onChange(key, newVal, formatted);
    });

    fieldGroup.appendChild(inputEl);
    container.appendChild(fieldGroup);
  }

  return container;
}

function evaluateCondition(condition: string, values: Record<string, unknown>): boolean {
  try {
    const keys = Object.keys(values);
    const vals = Object.values(values);
    const fn = new Function(...keys, `return Boolean(${condition});`);
    return fn(...vals);
  } catch {
    return true;
  }
}

function createPropertyInput(
  prop: PropertyDef,
  val: unknown,
  onValChange: (newVal: unknown) => void,
): HTMLElement {
  const wrapper = document.createElement("div");
  wrapper.className = "input-wrapper";

  switch (prop.type.toLowerCase()) {
    case "slider":
    case "range":
    case "number": {
      const numInput = document.createElement("input");
      numInput.type = prop.type === "slider" ? "range" : "number";
      numInput.min = String(prop.min ?? 0);
      numInput.max = String(prop.max ?? 100);
      numInput.step = String(prop.step ?? 1);
      numInput.value = String(val ?? prop.min ?? 0);

      const valDisplay = document.createElement("span");
      valDisplay.className = "num-val";
      valDisplay.textContent = numInput.value;

      numInput.addEventListener("input", () => {
        const n = Number.parseFloat(numInput.value);
        valDisplay.textContent = numInput.value;
        onValChange(n);
      });

      wrapper.append(numInput, valDisplay);
      break;
    }

    case "checkbox": {
      const cb = document.createElement("input");
      cb.type = "checkbox";
      cb.checked = Boolean(val);
      cb.addEventListener("change", () => onValChange(cb.checked));
      wrapper.appendChild(cb);
      break;
    }

    case "color": {
      const colorInput = document.createElement("input");
      colorInput.type = "color";
      colorInput.value = typeof val === "string" && val.startsWith("#") ? val : "#3498db";
      colorInput.addEventListener("input", () => onValChange(colorInput.value));
      wrapper.appendChild(colorInput);
      break;
    }

    case "combo": {
      const select = document.createElement("select");
      select.className = "inspector-select";
      if (prop.options) {
        for (const opt of prop.options) {
          const optionEl = document.createElement("option");
          optionEl.value = String(opt.value);
          optionEl.textContent = opt.label;
          if (opt.value === val) optionEl.selected = true;
          select.appendChild(optionEl);
        }
      }
      select.addEventListener("change", () => onValChange(select.value));
      wrapper.appendChild(select);
      break;
    }

    case "textarea": {
      const ta = document.createElement("textarea");
      ta.className = "inspector-textarea";
      ta.value = String(val ?? "");
      ta.addEventListener("input", () => onValChange(ta.value));
      wrapper.appendChild(ta);
      break;
    }

    case "button": {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "inspector-button";
      btn.textContent = prop.text || "Trigger Action";
      btn.addEventListener("click", () => onValChange(true));
      wrapper.appendChild(btn);
      break;
    }

    case "section":
    case "label": {
      const heading = document.createElement("div");
      heading.className = "inspector-section-header";
      heading.textContent = prop.text;
      wrapper.appendChild(heading);
      break;
    }

    default: {
      const txt = document.createElement("input");
      txt.type = "text";
      txt.className = "inspector-text";
      txt.value = String(val ?? "");
      txt.addEventListener("input", () => onValChange(txt.value));
      wrapper.appendChild(txt);
      break;
    }
  }

  return wrapper;
}
