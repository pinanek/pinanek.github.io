import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const size = style(
  {
    width: vars.spacing[6],
    height: vars.spacing[6]
  },
  'size'
)

const button = style(
  {
    padding: 0,
    border: 0,
    margin: 0,
    background: 'none',
    cursor: 'pointer'
  },
  'button'
)

const placeHolder = style(
  {
    display: 'inline-block'
  },
  'place-holder'
)

const svg = style(
  {
    fill: 'none'
  },
  'svg'
)

globalStyle(`${svg} path`, {
  fill: vars.color.textDefault,
  transition: `fill ${vars.duration.default} ${vars.ease.out}`
})

export { size, button, placeHolder, svg }
