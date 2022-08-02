<template>
  <aside :class="variant">
    <div class="title">
      <InfoBarIcon :variant="variant" />
      <span>{{ title }}</span>
    </div>
    <div>
      <slot />
    </div>
  </aside>
</template>

<script setup lang="ts">
interface Props {
  title?: string
  variant: 'note' | 'success' | 'warning' | 'error'
}

function capitalizeFirstLetter(str: string) {
  return str[0].toUpperCase() + str.slice(1)
}

const props = defineProps<Props>()

const title = props.title ?? capitalizeFirstLetter(props.variant)
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

aside {
  border-radius: $radii-md;
  padding: $spacing-0_5 $spacing-6;
  margin: 1em 0;
  transition: background-color $duration-default $ease-in-out;
}

.title {
  display: flex;
  gap: $spacing-3;
  align-items: center;
  margin: 1em 0;

  span {
    font-weight: $font-weight-bold;
    line-height: calc(1em + 0.75rem);
    transition: color $duration-default $ease-in-out;
  }
}

.note {
  background-color: var(--color-content-info-bar-note-background);
}

.success {
  background-color: var(--color-content-info-bar-success-background);
}

.warning {
  background-color: var(--color-content-info-bar-warning-background);
}

.error {
  background-color: var(--color-content-info-bar-error-background);
}
</style>
