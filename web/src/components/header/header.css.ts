import { style } from '@vanilla-extract/css'
import { vars } from '@/styles'
import { OPENED_HEADER_CLASS } from './menu-button'

const wrapper = style(
  {
    position: 'sticky',
    top: 0,
    zIndex: 100,
    backgroundColor: vars.color.pageBackground,
    transition: `background-color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'wrapper'
)

const container = style(
  {
    display: 'flex',
    justifyContent: 'space-between',
    gap: vars.spacing[6],
    flexWrap: 'wrap',
    paddingTop: vars.spacing[6],
    paddingBottom: vars.spacing[6],

    '@media': {
      [`${vars.breakpoint.md}`]: {
        gap: vars.spacing[12],
        alignItems: 'center'
      }
    }
  },
  'container'
)

const logo = style(
  {
    color: vars.color.textDefault,
    textDecoration: 'none',
    transition: `color ${vars.duration.default} ${vars.ease.inOut}`
  },
  'logo'
)

const items = style(
  {
    display: 'none',
    flexDirection: 'column',
    justifyContent: 'space-between',
    flexGrow: 1,
    flexBasis: '100%',
    order: 1,
    gap: vars.spacing[6],
    paddingLeft: vars.spacing[2],

    selectors: {
      [`.${OPENED_HEADER_CLASS} &`]: {
        display: 'flex'
      }
    },

    '@media': {
      [`${vars.breakpoint.md}`]: {
        order: 0,
        display: 'flex',
        flexBasis: 0,
        paddingLeft: 0,
        alignItems: 'center',
        flexDirection: 'row'
      }
    }
  },
  'items'
)

const headerItems = style(
  {
    display: 'flex',
    flexDirection: 'column',
    gap: vars.spacing[6],

    '@media': {
      [`${vars.breakpoint.md}`]: {
        alignItems: 'center',
        flexDirection: 'row'
      }
    }
  },
  'header-items'
)

const menuButton = style(
  {
    display: 'inline-block',

    '@media': {
      [`${vars.breakpoint.md}`]: {
        display: 'none !important'
      }
    }
  },
  'menu-button'
)

export { wrapper, container, logo, items, headerItems, menuButton }
