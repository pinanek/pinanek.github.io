import { createGlobalThemeContract, createGlobalTheme } from '@vanilla-extract/css'
import constants from '@/constants'
import { generateColorVarName, getColorTokens } from './utils/color'
import tokens from './tokens'

const { color, breakpoint, ...other } = tokens

const commonColorTokens = getColorTokens(color.common)
const commonColorVars = createGlobalThemeContract(commonColorTokens.light, generateColorVarName)
createGlobalTheme(':root', commonColorVars, commonColorTokens.light)
createGlobalTheme(`:root.${constants.colorMode.darkClass}`, commonColorVars, commonColorTokens.dark)

const shikiColorTokens = getColorTokens(color.shiki)
const shikiColorVars = createGlobalThemeContract(shikiColorTokens.light, generateColorVarName)
createGlobalTheme('article', shikiColorVars, shikiColorTokens.light)
createGlobalTheme(`:root.${constants.colorMode.darkClass} article`, shikiColorVars, shikiColorTokens.dark)

const vars = {
  color: commonColorVars,
  breakpoint,
  ...other
}

export { vars }
