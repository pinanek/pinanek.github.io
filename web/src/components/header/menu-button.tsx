import { createSignal, onMount, Show, splitProps } from 'solid-js'
import classnames from 'clsx'
import type { JSX, Component } from 'solid-js'
import * as styles from './header-button.css'

type Props = JSX.ButtonHTMLAttributes<HTMLButtonElement>

const iconPaths = {
  navigation:
    'M2.753 18h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5a.75.75 0 0 1-.102-1.493L2.753 18h18.5-18.5Zm0-6.497h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5a.75.75 0 0 1-.102-1.493l.102-.007h18.5-18.5Zm-.001-6.5h18.5a.75.75 0 0 1 .102 1.493l-.102.007h-18.5A.75.75 0 0 1 2.65 5.01l.102-.007h18.5-18.5Z',
  dismiss:
    'm4.397 4.554.073-.084a.75.75 0 0 1 .976-.073l.084.073L12 10.939l6.47-6.47a.75.75 0 1 1 1.06 1.061L13.061 12l6.47 6.47a.75.75 0 0 1 .072.976l-.073.084a.75.75 0 0 1-.976.073l-.084-.073L12 13.061l-6.47 6.47a.75.75 0 0 1-1.06-1.061L10.939 12l-6.47-6.47a.75.75 0 0 1-.072-.976l.073-.084-.073.084Z'
}

const OPENED_HEADER_CLASS = 'mobile-header-menu-open'

const MenuButton: Component<Props> = (props) => {
  const [isMounted, setMounted] = createSignal(false)
  const [isMenuOpened, setMenuOpened] = createSignal(false)
  const displayIcon = () => (isMenuOpened() ? 'dismiss' : 'navigation')
  const label = () => (isMenuOpened() ? 'Close header menu' : 'Open header menu')

  const [, restProps] = splitProps(props, ['class', 'children'])

  function handleToggleMenu() {
    const body = document.body

    setMenuOpened((value) => {
      const newValue = !value

      if (newValue) body.classList.add(OPENED_HEADER_CLASS)
      else {
        body.classList.remove(OPENED_HEADER_CLASS)

        if (body.classList.length === 0) {
          body.removeAttribute('class')
        }
      }

      return newValue
    })
  }

  onMount(() => setMounted(true))

  return (
    <Show
      when={isMounted()}
      fallback={
        <svg
          class={classnames(styles.size, styles.svg, styles.placeHolder, props.class)}
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path d={iconPaths['navigation']} />
        </svg>
      }
    >
      <button
        class={classnames(styles.button, styles.size, props.class)}
        aria-label={label()}
        aria-pressed={isMenuOpened()}
        onclick={handleToggleMenu}
        type="button"
        {...restProps}
      >
        <svg class={styles.svg} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path d={iconPaths[displayIcon()]} />
        </svg>
      </button>
    </Show>
  )
}

export default MenuButton
export { OPENED_HEADER_CLASS }
