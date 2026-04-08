import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { getAllPosts } from "@/lib/blog";
import { buildPageMetadata } from "@/lib/metadata";
import { SITE } from "@/lib/config";
import { BlogList } from "@/components/blog-list";

export const metadata: Metadata = buildPageMetadata({
  title: "Blog",
  description: "Writing by Joonho Lim on plastic surgery, clinical thinking, engineering, and the slower path. Notes from a surgeon who builds his own tools.",
  path: "/blog",
});

export default async function BlogPage() {
  const posts = await getAllPosts();
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Blog", href: "/blog" },
  ]);

  const collectionSchema = {
    "@context": "https://schema.org",
    "@type": "CollectionPage",
    name: "Blog",
    description: "Writing by Joonho Lim on plastic surgery, clinical thinking, engineering, and the slower path.",
    url: `${SITE.url}/blog`,
    mainEntity: {
      "@type": "ItemList",
      itemListElement: posts.map((post, i) => ({
        "@type": "ListItem",
        position: i + 1,
        url: `${SITE.url}/blog/${post.slug}`,
        name: post.title,
      })),
    },
  };

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      <JsonLd data={collectionSchema} />
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">Blog</h1>
        <p className="mt-4 max-w-xl text-lg leading-relaxed text-muted">
          Notes on plastic surgery, clinical thinking, and building with code.
        </p>
      </section>

      <section className="pb-24">
        <BlogList posts={posts} />
      </section>
    </div>
  );
}
