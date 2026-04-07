import type { Metadata } from "next";
import Link from "next/link";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { getAllPosts } from "@/lib/blog";

export const metadata: Metadata = {
  title: "Blog",
  description: "Writing by Joonho Lim on medicine, engineering, and the slower path.",
  alternates: { canonical: "/blog" },
  openGraph: {
    title: "Blog",
    description: "Writing by Joonho Lim on medicine, engineering, and the slower path.",
    url: "/blog",
  },
};

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
        <div className="space-y-4">
          {posts.map((post) => (
            <article
              key={post.slug}
              className="rounded-lg border border-border bg-card p-5 transition-colors hover:border-accent/30"
            >
              <p className="text-sm text-muted">{post.formattedDate}</p>
              <h2 className="mt-2 text-xl font-semibold text-foreground">
                <Link href={`/blog/${post.slug}`} className="hover:text-accent">
                  {post.title}
                </Link>
              </h2>
              <p className="mt-2 text-sm leading-relaxed text-muted">
                {post.description}
              </p>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}
