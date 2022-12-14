import { setColorScheme, type ColorMode } from "@/utils/color-mode";

type HtmlSvg = HTMLElement & SVGElement;

type Data = {
  [key in ColorMode]: {
    next: ColorMode;
    element: HtmlSvg;
    title: string;
  };
};

class ThemeButton extends HTMLElement {
  private data: Data;

  constructor() {
    super();
    this.setAttribute("role", "button");

    this.dataset.colorMode = "system";

    this.data = {
      system: {
        next: "dark",
        element: this.querySelector("#theme-button-system-icon") as HtmlSvg,
        title: "Use light color mode",
      },
      light: {
        next: "system",
        element: this.querySelector("#theme-button-light-icon") as HtmlSvg,
        title: "Use dark color mode",
      },
      dark: {
        next: "light",
        element: this.querySelector("#theme-button-dark-icon") as HtmlSvg,
        title: "Use system preference color scheme",
      },
    };

    this.handleStorageChange();
    window.addEventListener("storage", this.handleStorageChange);
    this.addEventListener("click", this.handleClick);
  }

  private setColorMode(colorMode: ColorMode) {
    this.dataset.colorMode = colorMode;

    const currentColorModeData = this.data[colorMode];

    currentColorModeData.element.removeAttribute("aria-hidden");
    this.data[currentColorModeData.next].element.setAttribute("aria-hidden", "true");
    this.title = currentColorModeData.title;
  }

  private handleStorageChange() {
    const savedColorMode = localStorage.getItem("color-mode");

    if (savedColorMode !== "light" && savedColorMode !== "dark" && savedColorMode !== "system") return;

    this.setColorMode(savedColorMode);
  }

  private handleClick() {
    let newColorMode: ColorMode;

    switch (this.dataset.colorMode) {
      case "system":
        newColorMode = "light";
        break;
      case "light":
        newColorMode = "dark";
        break;
      default:
        newColorMode = "system";
        break;
    }

    localStorage.setItem("color-mode", newColorMode);

    let newColorScheme = newColorMode;
    if (newColorScheme === "system") {
      newColorScheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }

    setColorScheme(newColorScheme);

    this.setColorMode(newColorMode);
  }
}

export default ThemeButton;
