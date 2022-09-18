import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const common = style(
  {
    position: 'relative',
    transition: `color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'common'
)

export { common }
