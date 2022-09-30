type Color<T extends Record<string, string[]>> = { [key in keyof T]: string }

interface ColorTokens<T extends Record<string, string[]>> {
  light: Color<T>
  dark: Color<T>
}

/**
 * Transform input colors with format:
 *
 * ```ts
 * const colors = {
 *   exampleColor: ['#lightColor', '#darkColor']
 *   // ...
 * }
 * ```
 *
 * to
 *
 * ```ts
 * const outputTokens = {
 *   light: {
 *     exampleColor: '#lightColor'
 *     // ...
 *   }
 *   dark: {
 *     exampleColor: '#darkColor'
 *     // ...
 *   }
 * }
 * ```
 */
function getColorTokens<T extends Record<string, string[]>>(colors: T): ColorTokens<T> {
  const initialValue: {
    light: Record<string, string>
    dark: Record<string, string>
  } = {
    light: {},
    dark: {}
  }

  const colorTokens = Object.entries(colors).reduce((saved, [key, value]) => {
    saved.light[key] = value[0]
    saved.dark[key] = value[1]

    return saved
  }, initialValue) as ColorTokens<T>

  return colorTokens
}

/** Transform input color token names to kebab-case with a prefix `color-` */
function generateColorVarName(_value: string | null, path: string[]): string {
  return `color-${getColorVarName(path)}`
}

/** Transform input color token names to kebab-case with a prefix `shiki-` */
function generateShikiColorVarName(_value: string | null, path: string[]): string {
  return `shiki-${getColorVarName(path)}`
}

function getColorVarName(path: string[]): string {
  return path
    .join('-')
    .replace(/((?<=[a-z\d])[A-Z]|(?<=[A-Z\d])[A-Z](?=[a-z]))/g, '-$1') // camelCase to kebab case
    .toLowerCase()
}

export { getColorTokens, generateColorVarName, generateShikiColorVarName }
