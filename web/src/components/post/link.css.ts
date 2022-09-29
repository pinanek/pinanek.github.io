import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    color: vars.color.accentDefault,
    textDecoration: 'none',
    transition: `color ${vars.duration.default} ${vars.ease.inOut}`,

    selectors: {
      '&:hover': {
        textDecoration: 'underline'
      }
    }
  },
  'main'
)

export { main }
