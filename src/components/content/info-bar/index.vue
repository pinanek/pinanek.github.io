<template>
  <aside
    :class="{
      note: variant === 'note',
      success: variant === 'success',
      warning: variant === 'warning',
      error: variant === 'error'
    }"
  >
    <div class="title">
      <InfoBarIcon :variant="variant" />
      <span>{{ title ?? capitalizeFirstLetter(variant) }}</span>
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

defineProps<Props>()

function capitalizeFirstLetter(str: string) {
  return str[0].toUpperCase() + str.slice(1)
}
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
