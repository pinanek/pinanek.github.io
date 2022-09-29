import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    margin: '1em 0',
    lineHeight: vars.lineHeight.postParagraph,
    transition: `color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'main'
)

export { main }
