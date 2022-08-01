<template>
  <div class="main" :class="className" v-bind="props">
    <div v-if="lang !== 'text' || title !== undefined" class="info">
      <span v-if="title !== undefined" class="title">{{ title }}</span>
      <span v-if="lang !== 'text' || title !== undefined" class="lang">{{ langNames[lang] }}</span>
    </div>
    <pre><slot/></pre>
  </div>
</template>

<script setup lang="ts">
import { useAttrs } from 'vue'

const { lang, title, className, ...props } = useAttrs() as {
  lang: string
  title?: string
  className: string
  [x: string]: string | undefined
}

const langNames: Record<string, string> = {
  py: 'Python',
  cpp: 'C++',
  csharp: 'C#',
  php: 'PHP',
  js: 'Javascript',
  ts: 'Typescript',
  tsx: 'TSX',
  bash: 'Bash',
  diff: 'Diff',
  json: 'JSON',
  jsonc: 'JSONC',
  yaml: 'Yaml',
  text: 'Text'
}
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
