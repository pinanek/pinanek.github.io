function initColorMode() {
  const root = document.documentElement
  const savedColorMode = localStorage.getItem('color-mode')
  if (
    savedColorMode === 'dark' ||
    ((savedColorMode === 'system' || savedColorMode === undefined) &&
      window.matchMedia('(prefers-color-scheme: dark)').matches)
  ) {
    root.classList.add('color-mode-dark')
  } else {
    root.classList.remove('color-mode-dark')
  }
}

function handleColorMode() {
  type ColorMode = 'system' | 'light' | 'dark'

  function setDarkColorMode(isDarkColorMode: boolean) {
    isDarkColorMode
      ? document.documentElement.classList.add('color-mode-dark')
      : document.documentElement.classList.remove('color-mode-dark')
  }

  function updateColorMode() {
    const savedColorMode = localStorage.getItem('color-mode') as ColorMode | undefined
    const isPreferDarkColorMode = window.matchMedia('(prefers-color-scheme: dark)').matches
    if (
      savedColorMode === undefined ||
      !(savedColorMode === 'system' || savedColorMode === 'light' || savedColorMode === 'dark')
    ) {
      localStorage.setItem('color-mode', 'system')
      setDarkColorMode(isPreferDarkColorMode)
    } else {
      if (savedColorMode === 'system') {
        setDarkColorMode(isPreferDarkColorMode)
      } else {
        setDarkColorMode(savedColorMode === 'dark')
      }
    }
  }

  updateColorMode()
  window.addEventListener('storage', updateColorMode)
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateColorMode)
}

export { initColorMode, handleColorMode }
