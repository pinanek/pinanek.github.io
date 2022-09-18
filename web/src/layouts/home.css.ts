import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const headerTopSpacing = style(
  {
    height: vars.spacing[16]
  },
  'header-top-spacing'
)

const headerBottomSpacing = style(
  {
    height: vars.spacing[12]
  },
  'header-bottom-spacing'
)

export { headerTopSpacing, headerBottomSpacing }
