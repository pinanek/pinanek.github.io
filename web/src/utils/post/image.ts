import constants from '@/constants'
import { getPicture as astroGetPicture } from '@astrojs/image'

interface GetPictureOptions {
  src: string
  width: number
  height: number
}

interface GetPictureOutput {
  image: astroHTML.JSX.ImgHTMLAttributes
  sources: {
    type: string
    srcset: string
  }[]
  sizes: string
}

async function getPicture({ src, width, height }: GetPictureOptions): Promise<GetPictureOutput> {
  const outputWidths: number[] = []

  for (let i = 0; i < constants.image.widths.length; i++) {
    const outputWidth = constants.image.widths[i]

    if (width <= outputWidth) {
      outputWidths.push(width)
      break
    }

    outputWidths.push(outputWidth)

    if (constants.image.widths.length === i + 1 && outputWidth < width) {
      outputWidths.push(width)
      break
    }
  }

  const { sources, image } = await astroGetPicture({
    src,
    widths: outputWidths,
    formats: constants.image.formats,
    aspectRatio: `${width}:${height}`
  })

  const maxWidth = outputWidths[outputWidths.length - 1]
  const sizes = `(min-width: ${maxWidth}px) ${maxWidth}px, 100vw`

  return {
    image,
    sources,
    sizes
  }
}

export { getPicture }
