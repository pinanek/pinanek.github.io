import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    fontSize: '2rem',
    fontWeight: vars.fontWeight.extraBold
  },
  'main'
)

export { main }
