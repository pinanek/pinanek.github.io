import { animate, spring, svg } from "animejs";

type ColorMode = "system" | "light" | "dark";
type ResolvedColorMode = "light" | "dark";

const COLOR_MODE_KEY = "garden-color-mode";

const mediaQuery = globalThis.matchMedia("(prefers-color-scheme: dark)");

const springEase = spring({
  stiffness: 300,
  damping: 40,
  mass: 1,
  velocity: 0,
});

interface ColorModeElements {
  icon: SVGPathElement;
  sun: SVGPathElement;
  moon: SVGPathElement;
  rays: SVGGElement;
}

interface Animations {
  icon?: ReturnType<typeof animate>;
  rays?: ReturnType<typeof animate>;
}

const animations = new WeakMap<HTMLButtonElement, Animations>();

function isColorMode(value: unknown): value is ColorMode {
  return value === "system" || value === "light" || value === "dark";
}

function getColorMode(): ColorMode {
  const mode = localStorage.getItem(COLOR_MODE_KEY);

  return isColorMode(mode) ? mode : "system";
}

function resolveColorMode(mode: ColorMode): ResolvedColorMode {
  if (mode === "system") {
    return mediaQuery.matches ? "dark" : "light";
  }

  return mode;
}

function getElements(button: HTMLButtonElement): ColorModeElements | null {
  const icon = button.querySelector<SVGPathElement>("[data-theme-icon]");

  const sun = button.querySelector<SVGPathElement>("[data-theme-sun-target]");

  const moon = button.querySelector<SVGPathElement>("[data-theme-moon-target]");

  const rays = button.querySelector<SVGGElement>("[data-theme-rays]");

  if (!icon || !sun || !moon || !rays) {
    return null;
  }

  return {
    icon,
    sun,
    moon,
    rays,
  };
}

function updateLabel(button: HTMLButtonElement, isDark: boolean): void {
  const label = isDark ? "Switch to light mode" : "Switch to dark mode";

  button.title = label;

  button.setAttribute("aria-label", label);

  button.setAttribute("aria-pressed", String(isDark));
}

function updateButton(button: HTMLButtonElement, isDark: boolean, shouldAnimate: boolean): void {
  const elements = getElements(button);

  if (!elements) {
    return;
  }

  const { icon, sun, moon, rays } = elements;

  const target = isDark ? moon : sun;

  const scale = isDark ? 0 : 1;

  if (!shouldAnimate) {
    icon.setAttribute("d", target.getAttribute("d") ?? "");

    rays.style.opacity = String(scale);

    rays.style.transform = `scale(${scale})`;

    updateLabel(button, isDark);

    return;
  }

  const currentIsDark = button.getAttribute("aria-pressed") === "true";

  if (currentIsDark === isDark) {
    updateLabel(button, isDark);
    return;
  }

  const previous = animations.get(button);

  previous?.icon?.cancel();
  previous?.rays?.cancel();

  const iconAnimation = animate(icon, {
    d: svg.morphTo(target, 0),
    ease: springEase,
  });

  const raysAnimation = animate(rays, {
    opacity: scale,
    scale,
    ease: springEase,
  });

  animations.set(button, {
    icon: iconAnimation,
    rays: raysAnimation,
  });

  updateLabel(button, isDark);
}

function applyColorMode(mode: ColorMode, shouldAnimate = true): void {
  const resolved = resolveColorMode(mode);

  const isDark = resolved === "dark";

  document.documentElement.classList.toggle("dark", isDark);

  document.documentElement.style.colorScheme = resolved;

  const buttons = document.querySelectorAll<HTMLButtonElement>("[data-color-mode-button]");

  for (const button of buttons) {
    updateButton(button, isDark, shouldAnimate);
  }
}

function toggleColorMode(): void {
  const isDark = document.documentElement.classList.contains("dark");

  const mode: ResolvedColorMode = isDark ? "light" : "dark";

  localStorage.setItem(COLOR_MODE_KEY, mode);

  applyColorMode(mode);
}

function initializeColorMode(): void {
  applyColorMode(getColorMode(), false);

  document.addEventListener("click", (event) => {
    if (!(event.target instanceof Element)) {
      return;
    }

    if (!event.target.closest("[data-color-mode-button]")) {
      return;
    }

    toggleColorMode();
  });

  globalThis.addEventListener("storage", (event) => {
    if (
      event.storageArea !== localStorage ||
      (event.key !== COLOR_MODE_KEY && event.key !== null)
    ) {
      return;
    }

    applyColorMode(isColorMode(event.newValue) ? event.newValue : "system");
  });

  mediaQuery.addEventListener("change", () => {
    if (getColorMode() === "system") {
      applyColorMode("system");
    }
  });
}

initializeColorMode();
