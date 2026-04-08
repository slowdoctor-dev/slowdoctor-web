import type { Metadata } from "next";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { getAllPosts } from "@/lib/blog";
import { buildPageMetadata } from "@/lib/metadata";
import { BlogList } from "@/components/blog-list";

export const metadata: Metadata = buildPageMetadata({
  title: "Blog",
  description: "Writing by Joonho Lim on medicine, engineering, and the slower path.",
  path: "/blog",
});

export default async function BlogPage() {
  const posts = await getAllPosts();
  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Blog", href: "/blog" },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={breadcrumbSchema} />
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">Blog</h1>
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
