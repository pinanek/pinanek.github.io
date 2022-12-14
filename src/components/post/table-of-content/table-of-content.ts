class TableOfContent extends HTMLElement {
  private headings: Map<string, HTMLAnchorElement>;
  private currentSelectedId: string;

  constructor() {
    super();

    this.headings = new Map(Array.from(this.querySelectorAll("a")).map((element) => [element.id, element]));

    const idFromLocation = window.location.hash === "" ? "" : `heading-${window.location.hash.slice(1)}`;
    this.currentSelectedId = this.headings.get(idFromLocation) ? idFromLocation : "";

    const postContent = document.getElementById("post-content-container") as HTMLElement;

    const setCurrent: IntersectionObserverCallback = (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          const currentSelectedHeading = this.headings.get(this.currentSelectedId);
          if (currentSelectedHeading) {
            currentSelectedHeading.dataset.selected = "false";
          }

          const newSelectedHeadingId = `heading-${entry.target.id}`;
          const newSelectedHeading = this.headings.get(newSelectedHeadingId);
          if (newSelectedHeading) {
            newSelectedHeading.dataset.selected = "true";
            this.currentSelectedId = newSelectedHeadingId;
          }

          break;
        }
      }
    };

    const observerOptions: IntersectionObserverInit = {
      rootMargin: "-10% 0% -45% 0%",
      threshold: 1,
    };

    const headingsObserver = new IntersectionObserver(setCurrent, observerOptions);

    postContent.querySelectorAll("h2[id], h3[id], h4[id]").forEach((heading) => headingsObserver.observe(heading));
  }
}

export default TableOfContent;
