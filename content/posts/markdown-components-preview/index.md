---
title: Markdown components preview 😜
seoTitle: Hello
description: Just an example post 😎
categories: ['Example']
image:
  url: default-logo.png
  alt: Logo of pinanek23
isPublished: true
publishedDate: 06-01-2022
lastUpdated: 06-09-2022
---

## Headings

## Heading level 2

### Heading level 3

#### Heading level 4

## Paragraphs

Hello this is an paragraph, I'm trying to make it as long as possible to check its readability. Still read 🤔? You should scroll down to check more components I test 😁.

## Emphasis

This is **bold text**

This is _italic text_

This is **_italic text_**

This is ~~strike-through text~~

## Blockquotes

> This is a blockquote

> This is a blockquote
>
> With multiple paragraphs

> This is blockquote
>
> > With nested blockquote

## Links

This is a [Normal Link](/), and this is an [External Link](https://www.youtube.com/watch?v=dQw4w9WgXcQ).

## Images

![Hello](default-logo.png)

## Lists

This is an ordered list

1. First item
2. Second item
3. Third item
   1. Indented item
   2. Indented item
4. Fourth item

This is an unordered list

- First item
- Second item
- Third item
  - Indented item
  - Indented item
- Fourth item

## Code

### Inline code

`const inlineCode: string = 'Inline Code'` Normal inline code

`const inlineCode: string = 'Inline Code'`{lang=ts} Highlighted inline code

### Code block

```
This is a plain text code block
```

```text title=hihi.txt
This is a plain text code block
```

```tsx
import * as React from 'react'

interface Props {
  children: React.ReactNode
}

function CodeBlock({ children }: Props): JSX.Element {
  return (
    <div>
      <h2>This is an code block with syntax highlighting 😍</h2>
      {children}
    </div>
  )
}
```

```tsx title=src/codeblock.tsx
import * as React from 'react'

interface Props {
  children: React.ReactNode
}

function CodeBlock({ children }: Props): JSX.Element {
  return (
    <div>
      <h2>This is an code block with syntax highlighting 😍</h2>
      {children}
    </div>
  )
}
```
