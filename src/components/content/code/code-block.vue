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
import { languageNames, type Language } from '@/markdown/highlight-languages'

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
}

.info {
  display: grid;
  grid-template-columns: 1fr auto;
  color: #d4d4d4;
  padding: $spacing-2 $spacing-4;
  background-color: #2c2c2c;
  border-radius: $radii-md;

  .lang {
    grid-column: 2;
  }
}

pre {
  padding: $spacing-2 $spacing-4;
  overflow: auto;

  :deep(code) {
    font-family: $font-mono;
  }
}
</style>
