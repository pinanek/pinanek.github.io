import { recipe } from '@vanilla-extract/recipes'
import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'

const common = style(
  {
    width: vars.spacing[6],
    height: vars.spacing[6],
    fill: 'none'
  },
  'common'
)

const path = recipe(
  {
    base: {
      transition: `fill ${vars.duration.default} ${vars.ease.inOut}`
    },

    variants: {
      variant: {
        note: {
          fill: vars.color.accentDefault
        },
        success: {
          fill: vars.color.infoBarSuccessIcon
        },
        warning: {
          fill: vars.color.infoBarWarningIcon
        },
        error: {
          fill: vars.color.infoBarErrorIcon
        }
      }
    }
  },
  'path'
)

export { common, path }
