import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const main = style(
  {
    margin: '1em 0',
    borderRadius: vars.radii.md,
    overflow: 'hidden',
    transition: `background-color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'main'
)

const header = style(
  {
    display: 'grid',
    gridTemplateColumns: '1fr auto',
    padding: `${vars.spacing[2]} ${vars.spacing[6]}`,
    borderRadius: vars.radii.md,
    backgroundColor: vars.color.codeBlockHeaderBackground,
    transitionProperty: 'color, background-color',
    transitionDuration: vars.duration.default,
    transitionTimingFunction: vars.ease.inOut
  },
  'header'
)

const headerLang = style(
  {
    gridColumn: 2
  },
  'header-lang'
)

const pre = style(
  {
    padding: `${vars.spacing[2]} ${vars.spacing[4]}`,
    overflow: 'auto'
  },
  'pre'
)

globalStyle(`${pre} code`, {
  fontFamily: vars.font.mono
})

globalStyle(`${pre} span:where(span.line > span)`, {
  transition: `color ${vars.duration.default} ${vars.ease.inOut}`
})

export { main, header, headerLang, pre }
