import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    margin: '1em 0'
  },
  'main'
)

const image = style(
  {
    borderRadius: vars.radii.md,
    margin: 'auto'
  },
  'image'
)

export { main, image }
