type HtmlSvg = HTMLElement & SVGElement;

interface DataValue {
  element: HtmlSvg;
  ariaLabel: string;
}

interface Data {
  opened: DataValue;
  closed: DataValue;
}

type State = "opened" | "closed";

class MenuButton extends HTMLElement {
  private data: Data;

  constructor() {
    super();

    this.setAttribute("role", "button");

    document.documentElement.dataset.menuState = "closed";

    this.data = {
      opened: {
        element: this.querySelector("#menu-button-opened") as HtmlSvg,
        ariaLabel: "Close header",
      },
      closed: {
        element: this.querySelector("#menu-button-closed") as HtmlSvg,
        ariaLabel: "Open header",
      },
    };

    this.ariaLabel = this.data["opened"].ariaLabel;

    this.addEventListener("click", this.handleClick);
  }

  private handleClick() {
    const root = document.documentElement;

    const oldState = root.dataset.menuState as State;

    root.dataset.menuState = root.dataset.menuState === "opened" ? "closed" : "opened";
    const currentState = root.dataset.menuState as State;

    this.ariaPressed = (currentState === "opened").toString();
    this.ariaLabel = this.data[currentState].ariaLabel;
    this.data[currentState].element.removeAttribute("aria-hidden");
    this.data[oldState].element.setAttribute("aria-hidden", "true");
  }
}

export default MenuButton;
