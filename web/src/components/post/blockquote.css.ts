import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    borderLeft: `${vars.spacing[1]} solid ${vars.color.blockquoteIndicator}`,
    paddingLeft: vars.spacing[4],
    margin: '1em 0',
    transition: `border-left-color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'main'
)

export { main }
