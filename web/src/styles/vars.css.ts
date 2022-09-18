import { createGlobalThemeContract, createGlobalTheme } from '@vanilla-extract/css'
import { generateColorVarName, getColorTokens } from './utils/color'
import tokens from './tokens'

const { color, breakpoint, ...other } = tokens

const commonColorTokens = getColorTokens(color.common)
const commonColorVars = createGlobalThemeContract(commonColorTokens.light, generateColorVarName)
createGlobalTheme(':root', commonColorVars, commonColorTokens.light)
createGlobalTheme(`:root.dark`, commonColorVars, commonColorTokens.dark)

const vars = {
  color: commonColorVars,
  breakpoint,
  ...other
}

export { vars }
