import type { OutputFormat } from '@astrojs/image/dist/loaders'

const image: Image = {
  formats: ['webp', 'png'],
  widths: [320, 480, 640, 960, 1280, 1600, 1920, 2560, 3840]
}

interface Image {
  formats: OutputFormat[]
  widths: number[]
}

export default image
