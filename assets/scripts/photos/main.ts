import mediumZoom from "medium-zoom/dist/pure";

const zoom = mediumZoom({
  margin: 24,
  scrollOffset: 40,
});

const zoomableImages = document.querySelectorAll<HTMLImageElement>(
  "[data-photo-zoomable]",
);

for (const image of zoomableImages) {
  const attachZoom = () => {
    if (image.naturalWidth > 0) {
      zoom.attach(image);
    }
  };

  if (image.complete) {
    attachZoom();
  } else {
    image.addEventListener("load", attachZoom, { once: true });
  }
}

const gallery = document.querySelector<HTMLElement>(".photos-page__grid");

if (gallery) {
  const items = Array.from(
    gallery.querySelectorAll<HTMLElement>(".photos-page__item"),
  );

  const layout = () => {
    const styles = getComputedStyle(gallery);
    const rowHeight = Number.parseFloat(styles.gridAutoRows);
    const rowGap = Number.parseFloat(styles.rowGap);

    if (!rowHeight) return;

    for (const item of items) {
      const span = Math.ceil(
        (item.getBoundingClientRect().height + rowGap) / (rowHeight + rowGap),
      );

      item.style.gridRowEnd = `span ${span}`;
    }
  };

  gallery.classList.add("photos-page__grid--masonry");

  const observer = new ResizeObserver(layout);
  observer.observe(gallery);

  for (const item of items) {
    observer.observe(item);
  }

  for (const image of gallery.querySelectorAll("img")) {
    image.addEventListener("load", layout);
  }

  layout();
}
