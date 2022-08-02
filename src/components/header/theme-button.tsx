/** @jsxImportSource solid-js */

import { createSignal, onMount, Show, type Component } from 'solid-js'
import classnames from 'clsx'
import Styles from './header-button.module.scss'

type ColorScheme = 'light' | 'dark' | 'none'

const iconPaths = {
  light:
    'M11.996 19.01a.75.75 0 0 1 .743.649l.007.102v1.5a.75.75 0 0 1-1.493.101l-.007-.101v-1.5a.75.75 0 0 1 .75-.75Zm6.022-2.072 1.06 1.06a.75.75 0 1 1-1.06 1.061l-1.06-1.06a.75.75 0 0 1 1.06-1.061Zm-10.983 0a.75.75 0 0 1 0 1.06L5.974 19.06a.75.75 0 0 1-1.06-1.06l1.06-1.061a.75.75 0 0 1 1.06 0ZM12 6.475a5.525 5.525 0 1 1 0 11.05 5.525 5.525 0 0 1 0-11.05Zm0 1.5a4.025 4.025 0 1 0 0 8.05 4.025 4.025 0 0 0 0-8.05Zm9.25 3.293a.75.75 0 0 1 .102 1.493l-.102.007h-1.5a.75.75 0 0 1-.102-1.493l.102-.007h1.5Zm-17-.029a.75.75 0 0 1 .102 1.494l-.102.006h-1.5a.75.75 0 0 1-.102-1.493l.102-.007h1.5Zm1.64-6.37.084.072 1.06 1.06a.75.75 0 0 1-.976 1.134l-.084-.073-1.06-1.06a.75.75 0 0 1 .976-1.134Zm13.188.072a.75.75 0 0 1 .073.977l-.073.084-1.06 1.06a.75.75 0 0 1-1.133-.976l.072-.084 1.06-1.061a.75.75 0 0 1 1.061 0ZM12 1.99a.75.75 0 0 1 .743.648l.007.102v1.5a.75.75 0 0 1-1.493.101l-.007-.102v-1.5a.75.75 0 0 1 .75-.75Z',
  dark: 'M20.026 17.001c-2.762 4.784-8.879 6.423-13.663 3.661a9.964 9.964 0 0 1-3.234-2.983.75.75 0 0 1 .365-1.131c3.767-1.348 5.785-2.911 6.956-5.146 1.232-2.353 1.551-4.93.689-8.464a.75.75 0 0 1 .769-.926 9.961 9.961 0 0 1 4.457 1.327C21.149 6.1 22.788 12.217 20.025 17Zm-8.248-4.903c-1.25 2.388-3.31 4.099-6.817 5.499a8.492 8.492 0 0 0 2.152 1.766 8.501 8.501 0 1 0 8.502-14.725 8.485 8.485 0 0 0-2.792-1.016c.647 3.384.23 6.044-1.045 8.476Z'
}

const preferDark = '(prefers-color-scheme: dark)'
const themeStorageKey = 'color-mode'

const ThemeButton: Component = () => {
  const [colorMode, setColorMode] = createSignal<ColorScheme>('none')
  const label = () => (colorMode() === 'dark' ? 'Use light color mode' : 'Use dark color mode')
  const actualColorMode = () => (colorMode() === 'dark' ? 'dark' : 'light')

  function handleStorageChange() {
    const savedColorMode = localStorage.getItem(themeStorageKey)
    if (savedColorMode !== 'light' && savedColorMode !== 'dark' && savedColorMode !== 'system') return

    if (savedColorMode === 'system') setColorMode(window.matchMedia(preferDark).matches ? 'dark' : 'light')
    else setColorMode(savedColorMode)
  }

  function handlePreferColorModeChange() {
    const savedColorMode = localStorage.getItem(themeStorageKey)
    if (savedColorMode !== 'system' && savedColorMode !== undefined) return

    setColorMode(window.matchMedia(preferDark).matches ? 'dark' : 'light')
  }

  function handleToggleTheme() {
    const root = document.documentElement

    setColorMode((value) => {
      const newValue = value === 'light' ? 'dark' : 'light'
      localStorage.setItem(themeStorageKey, newValue)
      newValue === 'dark' ? root.classList.add('color-mode-dark') : root.classList.remove('color-mode-dark')

      return newValue
    })
  }

  onMount(() => {
    handleStorageChange()
    window.addEventListener('storage', handleStorageChange)
    window.matchMedia(preferDark).addEventListener('change', handlePreferColorModeChange)
  })

  return (
    <Show when={colorMode() !== 'none'} fallback={<span class={classnames(Styles.size, Styles.placeHolder)} />}>
      <button
        class={classnames(Styles.size, Styles.button)}
        aria-label={label()}
        onclick={handleToggleTheme}
        type="button"
      >
        <svg class={Styles.svg} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d={iconPaths[actualColorMode()]} />
        </svg>
      </button>
    </Show>
  )
}

export default ThemeButton
