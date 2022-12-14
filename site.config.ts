import type { OutputFormat as BuiltImageFormat } from "@astrojs/image/dist/loaders";

const siteConfig: SiteConfig = {
  url: "https://pinanek.github.io/",
  name: "pinanek's blog",
  description: "My little garden 🌻 with knowledge and experience of what I have gained 😁",
  socialImage: {
    src: "/images/default-logo.png",
    alt: "Logo of pinanek",
  },

  imageOptimization: {
    formats: ["png", "webp"],
    widths: [320, 480, 640, 960, 1280, 1600, 1920, 2560, 3840],
  },
};

interface SiteConfig {
  /** Site deployed URL */
  url: string;

  /** Site name, used for SEO and title */
  name: string;

  /** Site description */
  description: string;

  /** Site default social image  */
  socialImage: {
    src: string;
    alt: string;
  };

  /** `@astrojs/image` options  */
  imageOptimization: {
    /** Optimized images formats */
    formats: BuiltImageFormat[];

    /** Preset of optimized images output widths */
    widths: number[];
  };
}

export default siteConfig;
