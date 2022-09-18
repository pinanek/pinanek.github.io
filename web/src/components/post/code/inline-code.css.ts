import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    padding: `${vars.spacing[1]} ${vars.spacing[2]}`,
    borderRadius: vars.radii.md,
    fontFamily: vars.font.mono,
    transition: `background-color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'main'
)

globalStyle(`${main} span:where(span.line > span)`, {
  transition: `color ${vars.duration.default} ${vars.ease.inOut}`
})

export { main }
