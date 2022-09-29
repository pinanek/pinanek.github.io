import { style } from '@vanilla-extract/css'
import { calc } from '@vanilla-extract/css-utils'
import { vars } from '@/styles'

const main = style(
  {
    position: 'absolute',
    left: calc.multiply(vars.spacing[8], -1),
    top: '50%',
    transform: 'translateY(-50%)',
    width: vars.spacing[6],
    height: vars.spacing[6],
    opacity: 0,
    color: vars.color.textDefault,
    transitionProperty: 'opacity, color',
    transitionDuration: vars.duration.default,
    transitionTimingFunction: vars.ease.inOut,

    selectors: {
      '&:hover': {
        opacity: 1
      },

      ':where(h2, h3, h4):hover &': {
        opacity: 1
      }
    }
  },
  'main'
)

const icon = style(
  {
    fill: 'none',
    stroke: 'currentcolor',
    strokeWidth: 2,
    strokeLinecap: 'round'
  },
  'icon'
)

export { main, icon }
