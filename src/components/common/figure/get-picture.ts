import siteConfig from "~/site.config";
import { getPicture as getPictureData } from "@astrojs/image";
import { getPlaiceholder } from "plaiceholder";
import type { HTMLAttributes } from "astro/types";
import type { IGetCSSReturn } from "plaiceholder/dist/css";

interface GetPictureOptions {
  src: string;
  alt: string;
  width: number;
  height: number;

  /**
   * If true, max generated images' width is original width
   *
   * @default true
   * */
  preserveOriginalWidth?: boolean;
}

interface GetPictureOutput {
  img: HTMLAttributes<"img">;
  sources: {
    type: string;
    srcset: string;
  }[];
  sizes: string;
  placeholder: IGetCSSReturn;
}

async function getPicture({
  src,
  width,
  height,
  alt,
  preserveOriginalWidth = true,
}: GetPictureOptions): Promise<GetPictureOutput> {
  let outputWidths: number[];

  if (preserveOriginalWidth) {
    outputWidths = [];

    for (let i = 0; i < siteConfig.imageOptimization.widths.length; i++) {
      const outputWidth = siteConfig.imageOptimization.widths[i];

      if (width <= outputWidth) {
        outputWidths.push(width);
        break;
      }

      outputWidths.push(outputWidth);

      if (siteConfig.imageOptimization.widths.length === i + 1 && outputWidth < width) {
        outputWidths.push(width);
        break;
      }
    }
  } else {
    outputWidths = siteConfig.imageOptimization.widths;
  }

  const { sources, image: img } = await getPictureData({
    src,
    alt,
    widths: outputWidths,
    formats: siteConfig.imageOptimization.formats,
    aspectRatio: `${width}:${height}`,
  });

  const maxWidth = outputWidths[outputWidths.length - 1];
  const sizes = `(min-width: ${maxWidth}px) ${maxWidth}px, 100vw`;

  const placeholder = await getPlaceHolder(src);

  return {
    img,
    sources,
    sizes,
    placeholder,
  };
}

async function getPlaceHolder(src: string): Promise<IGetCSSReturn> {
  let baseDir;

  if (import.meta.env.MODE === "production") {
    // Set base dir to build output folder
    baseDir = "_dist";
  } else {
    baseDir = "src";
  }

  const { css: placeholder } = await getPlaiceholder(src.replace("/@astroimage", ""), { dir: baseDir, size: 10 });

  return placeholder;
}

export { getPicture };
export type { GetPictureOptions };
