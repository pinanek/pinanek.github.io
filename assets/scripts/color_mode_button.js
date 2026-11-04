const COLOR_MODE_KEY = "garden-color-mode";

const mediaQuery = globalThis.matchMedia("(prefers-color-scheme: dark)");

let springEase;

const animations = new WeakMap();

function isColorMode(value) {
  return value === "system" || value === "light" || value === "dark";
}

function getColorMode() {
  const mode = localStorage.getItem(COLOR_MODE_KEY);

  return isColorMode(mode) ? mode : "system";
}

function resolveColorMode(mode) {
  if (mode === "system") {
    return mediaQuery.matches ? "dark" : "light";
  }

  return mode;
}

function getElements(button) {
  const icon = button.querySelector("[data-theme-icon]");

  const sun = button.querySelector("[data-theme-sun-target]");

  const moon = button.querySelector("[data-theme-moon-target]");

  const rays = button.querySelector("[data-theme-rays]");

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

function updateLabel(button, isDark) {
  const label = isDark ? "Switch to light mode" : "Switch to dark mode";

  button.title = label;

  button.setAttribute("aria-label", label);

  button.setAttribute("aria-pressed", String(isDark));
}

async function updateButton(button, isDark, shouldAnimate) {
  const elements = getElements(button);

  if (!elements) {
    return;
  }

  const { icon, sun, moon, rays } = elements;

  const target = isDark ? moon : sun;

  const scale = isDark ? 0 : 1;

  if (!shouldAnimate) {
    const previous = animations.get(button);

    previous?.icon?.cancel();
    previous?.rays?.cancel();

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

  updateLabel(button, isDark);

  try {
    const { animate, spring, svg } = await import("animejs");

    if (document.documentElement.classList.contains("dark") !== isDark) {
      return;
    }

    springEase ??= spring({
      stiffness: 300,
      damping: 40,
      mass: 1,
      velocity: 0,
    });

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
  } catch {
    if (document.documentElement.classList.contains("dark") === isDark) {
      updateButton(button, isDark, false);
    }
  }
}

function applyColorMode(mode, shouldAnimate = true) {
  const resolved = resolveColorMode(mode);

  const isDark = resolved === "dark";

  document.documentElement.classList.toggle("dark", isDark);

  document.documentElement.style.colorScheme = resolved;

  const buttons = document.querySelectorAll("[data-color-mode-button]");

  for (const button of buttons) {
    updateButton(button, isDark, shouldAnimate);
  }
}

function toggleColorMode() {
  const isDark = document.documentElement.classList.contains("dark");

  const mode = isDark ? "light" : "dark";

  localStorage.setItem(COLOR_MODE_KEY, mode);

  applyColorMode(mode);
}

function initializeColorMode() {
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

    applyColorMode(isColorMode(event.newValue) ? event.newValue : "system", false);
  });

  mediaQuery.addEventListener("change", () => {
    if (getColorMode() === "system") {
      applyColorMode("system", false);
    }
  });
}

initializeColorMode();
