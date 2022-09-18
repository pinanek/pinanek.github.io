import constants from '@/constants'

type ColorMode = 'system' | 'light' | 'dark'
type ColorScheme = 'light' | 'dark'

function setCurrentColorScheme(colorScheme: ColorScheme) {
  const root = document.documentElement

  if (colorScheme === 'dark') {
    root.classList.add(constants.colorMode.darkClass)
  } else {
    root.classList.remove(constants.colorMode.darkClass)

    if (root.classList.length === 0) {
      root.removeAttribute('class')
    }
  }
}

function updateColorMode() {
  const savedColorMode = localStorage.getItem('color-mode') as ColorMode | null
  const currentSystemColorScheme: ColorScheme = window.matchMedia(constants.colorMode.systemColorSchemeQuery).matches
    ? 'dark'
    : 'light'

  if (
    savedColorMode === null ||
    (savedColorMode !== 'system' && savedColorMode !== 'light' && savedColorMode === 'dark')
  ) {
    localStorage.setItem(constants.colorMode.storageKey, 'system')
    setCurrentColorScheme(currentSystemColorScheme)
  } else {
    if (savedColorMode === 'system') {
      setCurrentColorScheme(currentSystemColorScheme)
    } else {
      setCurrentColorScheme(savedColorMode)
    }
  }
}

export { setCurrentColorScheme, updateColorMode }
export type { ColorScheme, ColorMode }
