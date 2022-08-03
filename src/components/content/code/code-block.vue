<template>
  <div class="main" :class="className" v-bind="props">
    <div v-if="lang !== 'text' || title" class="info">
      <span v-if="title" class="title">{{ title }}</span>
      <span v-if="lang !== 'text' || title" class="lang">{{ languageName }}</span>
    </div>
    <pre><slot/></pre>
  </div>
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'
import { languageNames, type Language } from '@/markdown/highlighter/languages'

const { lang, title, className, ...props } = useAttrs() as {
  lang: Language
  title?: string
  className: string
  [x: string]: string | undefined
}

const languageName = languageNames[lang]
</script>

<script lang="ts">
export default {
  inheritAttrs: false
}
</script>

<style scoped lang="scss">
@use 'src/styles/tokens' as *;

.main {
  margin: 1em 0;
  border-radius: $radii-md;
  font-family: $font-mono;
  overflow: hidden;
  transition: background-color $duration-default $ease-in-out;
}

.info {
  display: grid;
  grid-template-columns: 1fr auto;
  padding: $spacing-2 $spacing-4;
  background-color: var(--shiki-color-header-background);
  border-radius: $radii-md;
  transition-property: color, background-color;
  transition-duration: $duration-default;
  transition-timing-function: $ease-in-out;

  .lang {
    grid-column: 2;
  }
}

pre {
  padding: $spacing-2 $spacing-4;
  overflow: auto;

  :deep(code) {
    font-family: $font-mono;

    span.line > span {
      transition: color $duration-default $ease-in-out;
    }
  }
}
</style>
