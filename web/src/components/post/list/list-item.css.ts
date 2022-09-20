import { globalStyle, style } from '@vanilla-extract/css'
import { vars } from '@/styles'
import { main as orderListStyle } from './ordered-list.css'

const main = style({}, 'main')

globalStyle(`${main}:not(li li)`, {
  lineHeight: `${vars.lineHeight.postParagraph}`,
  transition: `color ${vars.duration.default} ${vars.ease.inOut}`
})

globalStyle(`${orderListStyle} ${main}::marker`, {
  fontWeight: vars.fontWeight.bold
})

export { main }
