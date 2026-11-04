document.addEventListener(
  "error",
  (event) => {
    if (event.target instanceof HTMLImageElement && event.target.matches(".prose-link__image")) {
      event.target.hidden = true;
    }
  },
  true,
);

for (const image of document.querySelectorAll(".prose-link__image")) {
  if (image.complete && image.naturalWidth === 0) {
    image.hidden = true;
  }
}
