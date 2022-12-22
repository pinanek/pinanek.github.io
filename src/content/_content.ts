import { getCollection, type CollectionEntry } from "astro:content";

type PostInfo = Pick<CollectionEntry<"posts">, "slug" | "data">;

const posts = await getCollection("posts");

const postsInfo: PostInfo[] = posts.map(({ slug, data }) => ({ slug, data }));

export { posts, postsInfo, type PostInfo };
