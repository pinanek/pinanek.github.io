import * as path from "path";
import type { CollectionEntry } from "astro:content";

// A workaround for frontmatter images 😢
const postThumbnailImages = import.meta.glob<ImageMetadata>("/src/content/posts/**/images/*", { import: "default" });

async function getThumbnailImages(
  slug: string,
  data: CollectionEntry<"posts">["data"],
): Promise<Omit<ImageMetadata, "format">> {
  const thumbnailPath = path.join("/src/content/posts", slug, data.image.src).replace(/\\/g, "/");

  const imageData = postThumbnailImages[thumbnailPath];
  if (!imageData) throw Error("Post thumbnail image: Invalid image path!");

  return await imageData();
}

export { getThumbnailImages };
