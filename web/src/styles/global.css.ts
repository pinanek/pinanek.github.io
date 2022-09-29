import { globalStyle } from '@vanilla-extract/css'
import { vars } from './vars.css'
import './reset.css'

globalStyle('body', {
  color: vars.color.textDefault,
  backgroundColor: vars.color.pageBackground,
  fontFamily: vars.font.sans,
  fontWeight: vars.fontWeight.normal,
  transition: `background-color ${vars.duration.default} ${vars.ease.inOut}`
})
