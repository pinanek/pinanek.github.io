// eslint-disable-next-line import/no-unresolved
import { useMDXComponent } from 'next-contentlayer/hooks'
import { ArticleLayout } from '@/layouts'
import { Metadata } from '@/components'
import {
  ProseCodeBlock,
  ProseHeading2,
  ProseHeading3,
  ProseHeading4,
  ProseInlineCode,
  ProseLink,
  ProseOrderedList,
  ProseParagraph,
  ProseUnorderedList
} from '@/components/proses'
import { allPosts, type Post } from 'contentlayer/generated'
import type { GetStaticPaths, GetStaticProps } from 'next'
import type { Page } from '@/types/next'

interface PostPageProps {
  post: Post
}

const proseComponents = {
  pre: ProseCodeBlock,
  inlineCode: ProseInlineCode,
  a: ProseLink,
  p: ProseParagraph,
  h2: ProseHeading2,
  h3: ProseHeading3,
  h4: ProseHeading4,
  ol: ProseOrderedList,
  ul: ProseUnorderedList
}

const PostPage: Page<PostPageProps> = ({ post: { title, seoTitle, description, image, body, layout } }) => {
  const MDXContent = useMDXComponent(body.code || '')

  return (
    <>
      <Metadata title={title} seoTitle={seoTitle} description={description} image={image.url} imageAlt={image.alt} />
      {layout === 'article' && (
        <ArticleLayout>
          <MDXContent components={{ ...proseComponents }} />
        </ArticleLayout>
      )}
    </>
  )
}

const getStaticPaths: GetStaticPaths = async () => {
  const paths: string[] = allPosts.map(({ slug }) => `/posts/${slug}`)

  return {
    paths,
    fallback: false
  }
}

const getStaticProps: GetStaticProps = async ({ params }) => {
  const { slug } = params as { slug: string }

  const post = allPosts.find((post) => post.slug === slug)

  return {
    props: { post }
  }
}

export default PostPage
export { getStaticPaths, getStaticProps }