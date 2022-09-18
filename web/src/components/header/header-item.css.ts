import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    position: 'relative',
    color: vars.color.textDefault,
    fontWeight: vars.fontWeight.semiBold,
    textDecoration: 'none',
    transition: `color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'main'
)

const indicator = style(
  {
    position: 'absolute',
    left: 0,
    bottom: -4,
    height: 3,
    borderRadius: 1,
    width: 0,
    backgroundColor: vars.color.textDefault,
    transitionProperty: 'width, background-color',
    transitionDuration: vars.duration.default,
    transitionTimingFunction: vars.ease.inOut,

    selectors: {
      [`${main}:hover &`]: {
        width: '100%'
      }
    }
  },
  'indicator'
)

export { main, indicator }
