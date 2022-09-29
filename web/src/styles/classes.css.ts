import { style } from '@vanilla-extract/css'
import { vars } from './vars.css'

const pageSpacing = style(
  {
    marginLeft: 'auto',
    marginRight: 'auto',
    paddingLeft: vars.spacing[6],
    paddingRight: vars.spacing[6]
  },
  'page-spacing'
)

const pageSizing = style(
  {
    maxWidth: '68rem'
  },
  'page-sizing'
)

export { pageSpacing, pageSizing }
