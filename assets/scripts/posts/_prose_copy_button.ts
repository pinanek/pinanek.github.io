import { animate, spring } from "animejs";

const COPY_RESET_DELAY = 2_000;

const resetTimers = new WeakMap<HTMLButtonElement, number>();

const springEase = spring({
  stiffness: 500,
  damping: 50,
  mass: 1,
  velocity: 20,
});

function getCodeText(button: HTMLButtonElement): string | null {
  return button.closest(".prose-code-block")?.querySelector("pre")?.textContent ?? null;
}

async function copyText(text: string): Promise<void> {
  if (navigator.clipboard && globalThis.isSecureContext) {
    await navigator.clipboard.writeText(text);
    return;
  }

  const textarea = document.createElement("textarea");

  textarea.value = text;
  textarea.readOnly = true;

  Object.assign(textarea.style, {
    position: "fixed",
    inset: "0",
    opacity: "0",
    pointerEvents: "none",
  });

  document.body.append(textarea);
  textarea.select();

  const copied = document.execCommand("copy");

  textarea.remove();

  if (!copied) {
    throw new Error("Unable to copy code");
  }
}

function setTooltip(button: HTMLButtonElement, text: string): void {
  button.title = text;
  button.setAttribute("aria-label", text);
}

function setCopyState(button: HTMLButtonElement, copied: boolean): void {
  const copyIcon = button.querySelector<SVGElement>(".copy-icon");

  const checkIcon = button.querySelector<SVGElement>(".check-icon");

  button.dataset.copyState = copied ? "copied" : "idle";

  setTooltip(button, copied ? "Copied" : "Copy code");

  const hideIcon = copied ? copyIcon : checkIcon;

  const showIcon = copied ? checkIcon : copyIcon;

  if (hideIcon) {
    animate(hideIcon, {
      opacity: 0,
      scale: 0.65,
      ease: springEase,
    });
  }

  if (showIcon) {
    animate(showIcon, {
      opacity: [0, 1],
      scale: [0.5, 1],
      ease: springEase,
    });
  }
}

function clearResetTimer(button: HTMLButtonElement): void {
  const timer = resetTimers.get(button);

  if (timer === undefined) {
    return;
  }

  globalThis.clearTimeout(timer);
  resetTimers.delete(button);
}

function scheduleReset(button: HTMLButtonElement): void {
  const timer = globalThis.setTimeout(() => {
    setCopyState(button, false);
    resetTimers.delete(button);
  }, COPY_RESET_DELAY);

  resetTimers.set(button, timer);
}

document.addEventListener("click", async (event: MouseEvent) => {
  if (!(event.target instanceof Element)) {
    return;
  }

  const button = event.target.closest<HTMLButtonElement>("[data-copy-code]");

  if (!button || button.dataset.copyState === "copying") {
    return;
  }

  const code = getCodeText(button);

  if (code === null) {
    return;
  }

  clearResetTimer(button);

  button.dataset.copyState = "copying";

  button.disabled = true;

  try {
    await copyText(code);

    button.disabled = false;

    setCopyState(button, true);
    scheduleReset(button);
  } catch (error) {
    button.disabled = false;
    button.dataset.copyState = "idle";

    setTooltip(button, "Copy failed");

    console.error("Failed to copy code:", error);
  }
});
