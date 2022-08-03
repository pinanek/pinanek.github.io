import type * as Shiki from 'shiki'
import type { IRawThemeSetting } from 'vscode-textmate'

const tokens: IRawThemeSetting[] = [
  {
    settings: {
      foreground: '#011'
    }
  },
  {
    scope: ['meta.embedded', 'source.groovy.embedded', 'string meta.image.inline.markdown'],
    settings: {
      foreground: '#011'
    }
  },
  {
    scope: 'emphasis',
    settings: {
      fontStyle: 'italic'
    }
  },
  {
    scope: 'strong',
    settings: {
      fontStyle: 'bold'
    }
  },
  {
    scope: 'header',
    settings: {
      foreground: '#000080'
    }
  },
  {
    scope: 'comment',
    settings: {
      foreground: '#007'
    }
  },
  {
    scope: 'constant.language',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: [
      'constant.numeric',
      'variable.other.enummember',
      'keyword.operator.plus.exponent',
      'keyword.operator.minus.exponent'
    ],
    settings: {
      foreground: '#006'
    }
  },
  {
    scope: 'constant.regexp',
    settings: {
      foreground: '#646695'
    }
  },
  {
    scope: 'entity.name.tag',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'entity.name.tag.css',
    settings: {
      foreground: '#012'
    }
  },
  {
    scope: 'entity.other.attribute-name',
    settings: {
      foreground: '#008'
    }
  },
  {
    scope: [
      'entity.other.attribute-name.class.css',
      'entity.other.attribute-name.class.mixin.css',
      'entity.other.attribute-name.id.css',
      'entity.other.attribute-name.parent-selector.css',
      'entity.other.attribute-name.pseudo-class.css',
      'entity.other.attribute-name.pseudo-element.css',
      'source.css.less entity.other.attribute-name.id',
      'entity.other.attribute-name.scss'
    ],
    settings: {
      foreground: '#012'
    }
  },
  {
    scope: 'invalid',
    settings: {
      foreground: '#f44747'
    }
  },
  {
    scope: 'markup.underline',
    settings: {
      fontStyle: 'underline'
    }
  },
  {
    scope: 'markup.bold',
    settings: {
      fontStyle: 'bold',
      foreground: '#002'
    }
  },
  {
    scope: 'markup.heading',
    settings: {
      fontStyle: 'bold',
      foreground: '#002'
    }
  },
  {
    scope: 'markup.italic',
    settings: {
      fontStyle: 'italic'
    }
  },
  {
    scope: 'markup.strikethrough',
    settings: {
      fontStyle: 'strikethrough'
    }
  },
  {
    scope: 'markup.inserted',
    settings: {
      foreground: '#006'
    }
  },
  {
    scope: 'markup.deleted',
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: 'markup.changed',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'punctuation.definition.quote.begin.markdown',
    settings: {
      foreground: '#007'
    }
  },
  {
    scope: 'punctuation.definition.list.begin.markdown',
    settings: {
      foreground: '#6796e6'
    }
  },
  {
    scope: 'markup.inline.raw',
    settings: {
      foreground: '#005'
    }
  },
  {
    name: 'brackets of XML/HTML tags',
    scope: 'punctuation.definition.tag',
    settings: {
      foreground: '#014'
    }
  },
  {
    scope: ['meta.preprocessor', 'entity.name.function.preprocessor'],
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'meta.preprocessor.string',
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: 'meta.preprocessor.numeric',
    settings: {
      foreground: '#006'
    }
  },
  {
    scope: 'meta.structure.dictionary.key.python',
    settings: {
      foreground: '#008'
    }
  },
  {
    scope: 'meta.diff.header',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'storage',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'storage.type',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: ['storage.modifier', 'keyword.operator.noexcept'],
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: ['string', 'meta.embedded.assembly'],
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: 'string.tag',
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: 'string.value',
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: 'string.regexp',
    settings: {
      foreground: '#013'
    }
  },
  {
    name: 'String interpolation',
    scope: [
      'punctuation.definition.template-expression.begin',
      'punctuation.definition.template-expression.end',
      'punctuation.section.embedded'
    ],
    settings: {
      foreground: '#002'
    }
  },
  {
    name: 'Reset JavaScript string interpolation expression',
    scope: ['meta.template.expression'],
    settings: {
      foreground: '#011'
    }
  },
  {
    scope: [
      'support.type.vendored.property-name',
      'support.type.property-name',
      'variable.css',
      'variable.scss',
      'variable.other.less',
      'source.coffee.embedded'
    ],
    settings: {
      foreground: '#008'
    }
  },
  {
    scope: 'keyword',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'keyword.control',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'keyword.operator',
    settings: {
      foreground: '#011'
    }
  },
  {
    scope: [
      'keyword.operator.new',
      'keyword.operator.expression',
      'keyword.operator.cast',
      'keyword.operator.sizeof',
      'keyword.operator.alignof',
      'keyword.operator.typeid',
      'keyword.operator.alignas',
      'keyword.operator.instanceof',
      'keyword.operator.logical.python',
      'keyword.operator.wordlike'
    ],
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'keyword.other.unit',
    settings: {
      foreground: '#006'
    }
  },
  {
    scope: ['punctuation.section.embedded.begin.php', 'punctuation.section.embedded.end.php'],
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'support.function.git-rebase',
    settings: {
      foreground: '#008'
    }
  },
  {
    scope: 'constant.sha.git-rebase',
    settings: {
      foreground: '#006'
    }
  },
  {
    name: 'coloring of the Java import and package identifiers',
    scope: ['storage.modifier.import.java', 'variable.language.wildcard.java', 'storage.modifier.package.java'],
    settings: {
      foreground: '#011'
    }
  },
  {
    name: 'this.self',
    scope: 'variable.language',
    settings: {
      foreground: '#002'
    }
  },
  {
    name: 'Function declarations',
    scope: [
      'entity.name.function',
      'support.function',
      'support.constant.handlebars',
      'source.powershell variable.other.member',
      'entity.name.operator.custom-literal'
    ],
    settings: {
      foreground: '#009'
    }
  },
  {
    name: 'Types declaration and references',
    scope: [
      'support.class',
      'support.type',
      'entity.name.type',
      'entity.name.namespace',
      'entity.other.attribute',
      'entity.name.scope-resolution',
      'entity.name.class',
      'storage.type.numeric.go',
      'storage.type.byte.go',
      'storage.type.boolean.go',
      'storage.type.string.go',
      'storage.type.uintptr.go',
      'storage.type.error.go',
      'storage.type.rune.go',
      'storage.type.cs',
      'storage.type.generic.cs',
      'storage.type.modifier.cs',
      'storage.type.variable.cs',
      'storage.type.annotation.java',
      'storage.type.generic.java',
      'storage.type.java',
      'storage.type.object.array.java',
      'storage.type.primitive.array.java',
      'storage.type.primitive.java',
      'storage.type.token.java',
      'storage.type.groovy',
      'storage.type.annotation.groovy',
      'storage.type.parameters.groovy',
      'storage.type.generic.groovy',
      'storage.type.object.array.groovy',
      'storage.type.primitive.array.groovy',
      'storage.type.primitive.groovy'
    ],
    settings: {
      foreground: '#004'
    }
  },
  {
    name: 'Types declaration and references, TS grammar specific',
    scope: [
      'meta.type.cast.expr',
      'meta.type.new.expr',
      'support.constant.math',
      'support.constant.dom',
      'support.constant.json',
      'entity.other.inherited-class'
    ],
    settings: {
      foreground: '#004'
    }
  },
  {
    name: 'Control flow / Special keywords',
    scope: [
      'keyword.control',
      'source.cpp keyword.operator.new',
      'keyword.operator.delete',
      'keyword.other.using',
      'keyword.other.operator',
      'entity.name.operator'
    ],
    settings: {
      foreground: '#010'
    }
  },
  {
    name: 'Variable and parameter name',
    scope: [
      'variable',
      'meta.definition.variable.name',
      'support.variable',
      'entity.name.variable',
      'constant.other.placeholder'
    ],
    settings: {
      foreground: '#008'
    }
  },
  {
    name: 'Constants and enums',
    scope: ['variable.other.constant', 'variable.other.enummember'],
    settings: {
      foreground: '#003'
    }
  },
  {
    name: 'Object keys, TS grammar specific',
    scope: ['meta.object-literal.key'],
    settings: {
      foreground: '#008'
    }
  },
  {
    name: 'CSS property value',
    scope: [
      'support.constant.property-value',
      'support.constant.font-name',
      'support.constant.media-type',
      'support.constant.media',
      'constant.other.color.rgb-value',
      'constant.other.rgb-value',
      'support.constant.color'
    ],
    settings: {
      foreground: '#005'
    }
  },
  {
    name: 'Regular expression groups',
    scope: [
      'punctuation.definition.group.regexp',
      'punctuation.definition.group.assertion.regexp',
      'punctuation.definition.character-class.regexp',
      'punctuation.character.set.begin.regexp',
      'punctuation.character.set.end.regexp',
      'keyword.operator.negation.regexp',
      'support.other.parenthesis.regexp'
    ],
    settings: {
      foreground: '#005'
    }
  },
  {
    scope: [
      'constant.character.character-class.regexp',
      'constant.other.character-class.set.regexp',
      'constant.other.character-class.regexp',
      'constant.character.set.regexp'
    ],
    settings: {
      foreground: '#013'
    }
  },
  {
    scope: ['keyword.operator.or.regexp', 'keyword.control.anchor.regexp'],
    settings: {
      foreground: '#009'
    }
  },
  {
    scope: 'keyword.operator.quantifier.regexp',
    settings: {
      foreground: '#012'
    }
  },
  {
    scope: 'constant.character',
    settings: {
      foreground: '#002'
    }
  },
  {
    scope: 'constant.character.escape',
    settings: {
      foreground: '#012'
    }
  },
  {
    scope: 'entity.name.label',
    settings: {
      foreground: '#C8C8C8'
    }
  }
]

const colorReplacements: Record<string, string> = {
  '#000': 'color-background',
  '#001': 'color-text',
  '#002': 'token-keyword',
  '#003': 'token-constant',
  '#004': 'token-type',
  '#005': 'token-string-expression',
  '#006': 'token-number',
  '#007': 'token-comment',
  '#008': 'token-variable-parameter',
  '#009': 'token-function',
  '#010': 'token-control-flow',
  '#011': 'token-punctuation',
  '#012': 'token-css-attribute-quantifier-regex',
  '#013': 'token-string-regex',
  '#014': 'token-html-xml-tag'
}

const theme: Shiki.IThemeRegistration = {
  name: 'kenanip',
  type: 'dark',
  bg: `var(--shiki-${colorReplacements['#000']})`,
  fg: `var(--shiki-${colorReplacements['#001']})`,
  settings: tokens
}

export { theme, colorReplacements }
