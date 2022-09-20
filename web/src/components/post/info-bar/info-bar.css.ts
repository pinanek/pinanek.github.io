import { recipe } from '@vanilla-extract/recipes'
import { vars } from '@/styles'
import { globalStyle, style } from '@vanilla-extract/css'

const main = recipe(
  {
    base: {
      borderRadius: vars.radii.md,
      padding: `${vars.spacing['0.5']} ${vars.spacing[6]}`,
      margin: '1em 0',
      transition: `background-color ${vars.duration.default} ${vars.ease.inOut}`
    },

    variants: {
      variant: {
        note: {
          backgroundColor: vars.color.infoBarNoteBackground
        },
        success: {
          backgroundColor: vars.color.infoBarSuccessBackground
        },
        warning: {
          backgroundColor: vars.color.infoBarWarningBackground
        },
        error: {
          backgroundColor: vars.color.infoBarErrorBackground
        }
      }
    }
  },
  'main'
)

const titleContainer = style(
  {
    display: 'flex',
    gap: vars.spacing[3],
    alignItems: 'center',
    margin: '1em 0'
  },
  'title-container'
)

globalStyle(`${titleContainer} span`, {
  fontWeight: vars.fontWeight.bold,
  lineHeight: vars.lineHeight.postParagraph,
  transition: `color ${vars.duration.default} ${vars.ease.inOut}`
})

export { main, titleContainer }
