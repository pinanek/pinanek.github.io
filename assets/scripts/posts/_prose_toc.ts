interface TocEntry {
  link: HTMLAnchorElement;
  heading: HTMLElement;
  item: HTMLElement;
  pathStart: number;
  pathEnd: number;
}

interface TocGeometry {
  entry: TocEntry;
  x: number;
  top: number;
  bottom: number;
}

function initializeToc(): void {
  const root = document.querySelector<HTMLElement>(".prose-toc");

  if (!root || root.dataset.initialized) {
    return;
  }

  const body = root.querySelector<HTMLElement>(".prose-toc__body");

  const list = root.querySelector<HTMLOListElement>(".prose-toc__list");

  const svg = root.querySelector<SVGSVGElement>(".prose-toc__path");

  const path = root.querySelector<SVGPathElement>(".prose-toc__path-active");

  if (!body || !list || !svg || !path) {
    return;
  }

  /*
   * Preserve the null narrowing inside nested functions.
   */
  const tocRoot = root;
  const tocBody = body;
  const tocList = list;
  const tocSvg = svg;
  const tocPath = path;

  const entries = getEntries(tocList);

  if (entries.length === 0) {
    return;
  }

  tocRoot.dataset.initialized = "true";

  const visibleEntries = new Set<TocEntry>();

  const entryByHeading = new Map<Element, TocEntry>(entries.map((entry) => [entry.heading, entry]));

  let pathLength = 0;

  function getCssNumber(name: string, fallback: number): number {
    const value = Number.parseFloat(getComputedStyle(tocRoot).getPropertyValue(name));

    return Number.isFinite(value) ? value : fallback;
  }

  function commitPath(commands: Array<string | number>): number {
    tocPath.setAttribute("d", commands.join(" "));

    return tocPath.getTotalLength();
  }

  function getGeometry(): TocGeometry[] {
    const listRect = tocList.getBoundingClientRect();

    const offset = getCssNumber("--toc-path-offset", 2);

    return entries.map((entry) => {
      const rect = entry.link.getBoundingClientRect();

      const style = getComputedStyle(entry.link);

      const paddingTop = Number.parseFloat(style.paddingBlockStart) || 0;

      const paddingBottom = Number.parseFloat(style.paddingBlockEnd) || 0;

      return {
        entry,

        x: rect.left - listRect.left + offset,

        top: rect.top - listRect.top + paddingTop,

        bottom: rect.bottom - listRect.top - paddingBottom,
      };
    });
  }

  function appendConnector(
    commands: Array<string | number>,
    current: TocGeometry,
    next: TocGeometry,
  ): void {
    const deltaX = next.x - current.x;

    if (Math.abs(deltaX) < 0.5) {
      commands.push("V", next.top);

      return;
    }

    const gap = next.top - current.bottom;

    const middleY = current.bottom + gap / 2;

    const direction = Math.sign(deltaX);

    const radius = Math.min(
      getCssNumber("--toc-path-radius", 6),
      Math.abs(deltaX) / 2,
      Math.max(gap / 2, 0),
    );

    if (radius < 0.5) {
      commands.push("V", middleY, "H", next.x, "V", next.top);

      return;
    }

    const horizontalStart = current.x + direction * radius;

    const horizontalEnd = next.x - direction * radius;

    commands.push("V", middleY - radius, "Q", current.x, middleY, horizontalStart, middleY);

    if (Math.abs(horizontalEnd - horizontalStart) > 0.01) {
      commands.push("H", horizontalEnd);
    }

    commands.push("Q", next.x, middleY, next.x, middleY + radius, "V", next.top);
  }

  function buildPath(): void {
    const rect = tocList.getBoundingClientRect();

    if (rect.width <= 0 || rect.height <= 0) {
      return;
    }

    tocSvg.removeAttribute("viewBox");

    tocSvg.setAttribute("width", String(tocBody.clientWidth));

    tocSvg.setAttribute("height", String(rect.height));

    const geometry = getGeometry();
    const first = geometry[0];

    if (!first) {
      return;
    }

    const commands: Array<string | number> = ["M", first.x, first.top];

    first.entry.pathStart = commitPath(commands);

    commands.push("V", first.bottom);

    first.entry.pathEnd = commitPath(commands);

    let current = first;

    for (let index = 1; index < geometry.length; index++) {
      const next = geometry[index];

      if (!next) {
        continue;
      }

      appendConnector(commands, current, next);

      next.entry.pathStart = commitPath(commands);

      commands.push("V", next.bottom);

      next.entry.pathEnd = commitPath(commands);

      current = next;
    }

    pathLength = current.entry.pathEnd;

    updateActivePath();
  }

  function getCurrentEntry(): TocEntry {
    const offset = Math.min(globalThis.innerHeight * 0.25, 180);

    let current = entries[0]!;

    for (const entry of entries) {
      if (entry.heading.getBoundingClientRect().top > offset) {
        break;
      }

      current = entry;
    }

    return current;
  }

  function updateActivePath(): void {
    if (pathLength <= 0) {
      return;
    }

    const active = entries.filter((entry) => visibleEntries.has(entry));

    if (active.length === 0) {
      active.push(getCurrentEntry());
    }

    const first = active[0];

    if (!first) {
      return;
    }

    const activeSet = new Set(active);

    const start = Math.min(...active.map((entry) => entry.pathStart));

    const end = Math.max(...active.map((entry) => entry.pathEnd));

    for (const entry of entries) {
      const isActive = activeSet.has(entry);

      entry.item.toggleAttribute("data-active", isActive);

      if (entry === first) {
        entry.link.setAttribute("aria-current", "location");
      } else {
        entry.link.removeAttribute("aria-current");
      }
    }

    const length = Math.max(end - start, 0);

    tocPath.style.opacity = length > 0 ? "1" : "0";

    tocPath.style.strokeDasharray = `0 ${start} ${length} ${pathLength + 1}`;
  }

  const requestUpdate = frameThrottle(updateActivePath);

  const requestBuild = frameThrottle(buildPath);

  const intersectionObserver = new IntersectionObserver((changes) => {
    for (const change of changes) {
      const entry = entryByHeading.get(change.target);

      if (!entry) {
        continue;
      }

      if (change.isIntersecting) {
        visibleEntries.add(entry);
      } else {
        visibleEntries.delete(entry);
      }
    }

    requestUpdate();
  });

  for (const entry of entries) {
    intersectionObserver.observe(entry.heading);
  }

  const resizeObserver = new ResizeObserver(requestBuild);

  resizeObserver.observe(tocList);

  globalThis.addEventListener("scroll", requestUpdate, {
    passive: true,
  });

  buildPath();

  void document.fonts.ready.then(requestBuild);
}

function getEntries(list: HTMLOListElement): TocEntry[] {
  const links = list.querySelectorAll<HTMLAnchorElement>('a[href^="#"]');

  return Array.from(links).flatMap((link) => {
    const href = link.getAttribute("href");

    if (!href || href === "#") {
      return [];
    }

    let id: string;

    try {
      id = decodeURIComponent(href.slice(1));
    } catch {
      return [];
    }

    const heading = document.getElementById(id);

    const item = link.closest<HTMLElement>(".prose-toc__item");

    if (!heading || !item) {
      return [];
    }

    return [
      {
        link,
        heading,
        item,
        pathStart: 0,
        pathEnd: 0,
      },
    ];
  });
}

function frameThrottle(callback: () => void): () => void {
  let pending = false;

  return () => {
    if (pending) {
      return;
    }

    pending = true;

    requestAnimationFrame(() => {
      pending = false;
      callback();
    });
  };
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializeToc, {
    once: true,
  });
} else {
  initializeToc();
}
