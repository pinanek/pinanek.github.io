type ColorMode = "system" | "light" | "dark";

/** Display color mode */
type ColorScheme = "light" | "dark";

/**
 * Used when updates local storage or system color scheme
 *
 * See `/src/components/head/scripts.astro`
 */
function updateColorMode() {
  const savedColorMode = localStorage.getItem("color-mode") as ColorMode | null;
  const currentSystemColorScheme: ColorScheme = window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";

  if (
    savedColorMode === null ||
    (savedColorMode !== "system" && savedColorMode !== "light" && savedColorMode !== "dark")
  ) {
    localStorage.setItem("color-mode", "system");
    setColorScheme(currentSystemColorScheme);
  } else {
    const currentColorScheme = savedColorMode === "system" ? currentSystemColorScheme : savedColorMode;
    setColorScheme(currentColorScheme);
  }
}

function setColorScheme(colorScheme: ColorScheme) {
  document.documentElement.setAttribute("color-scheme", colorScheme);
}

export { updateColorMode, setColorScheme };
export type { ColorMode, ColorScheme };
