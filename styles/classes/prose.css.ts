import { globalStyle, style } from '@vanilla-extract/css'
import { calc } from '@vanilla-extract/css-utils'
import { vars } from '@/styles/vars.css'

const prose = style({
  transitionProperty: 'color',
  transitionDuration: vars.duration.default,
  transitionTimingFunction: vars.ease.inOut
})

// Headings
globalStyle(`${prose} h2, h3, h4`, {
  position: 'relative'
})

globalStyle(`${prose} h2`, {
  margin: '1.5em 0 0.5em',
  fontSize: '1.75em'
})

globalStyle(`${prose} h3`, {
  margin: '1.5rem 0 0.5rem',
  fontSize: '1.375em'
})

globalStyle(`${prose} h4`, {
  margin: '1em 0',
  fontSize: '1.125em'
})

globalStyle(`${prose} .heading-link`, {
  position: 'absolute',
  left: calc.negate(vars.spacing[8]),
  top: '50%',
  transform: 'translateY(-50%)',
  width: vars.spacing[6],
  height: vars.spacing[6],
  opacity: 0,
  color: vars.color.accentDefault,
  transitionProperty: 'opacity, color',
  transitionDuration: vars.duration.default,
  transitionTimingFunction: vars.ease.inOut
})

globalStyle(`${prose} :where(h2, h3, h4):hover .heading-link, .heading-link:hover`, {
  opacity: 1
})

globalStyle(`${prose} .heading-link svg`, {
  fill: 'none',
  stroke: 'currentcolor',
  strokeWidth: 2,
  strokeLinecap: 'round'
})

// Paragraphs
globalStyle(`${prose} p`, {
  margin: '1em 0',
  lineHeight: 'calc(1em + 0.75rem)'
})

globalStyle(`${prose} ul, ol`, {
  margin: '1em 0',
  lineHeight: 'calc(1em + 0.75rem)'
})

globalStyle(`${prose} li :where(ul, ol)`, {
  margin: '0.25em 0',
  paddingInlineStart: '1.25em'
})

globalStyle(`${prose} li::marker`, {
  fontWeight: vars.fontWeight.bold
})

// Code
globalStyle(`${prose} code`, {
  fontFamily: vars.font.mono
})

export { prose }
