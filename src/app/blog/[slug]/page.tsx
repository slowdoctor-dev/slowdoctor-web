import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { getAllPosts, getPostBySlug, getPostFrontmatter } from "@/lib/blog";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";

export const dynamicParams = false;

export async function generateStaticParams() {
  const posts = await getAllPosts();

  return posts.map((post) => ({
    slug: post.slug,
  }));
}

export async function generateMetadata(
  props: PageProps<"/blog/[slug]">,
): Promise<Metadata> {
  const { slug } = await props.params;
  const post = await getPostFrontmatter(slug);

  if (!post) {
    return {};
  }

  const image = post.image ?? "/og-default.png";

  return {
    title: post.title,
    description: post.description,
    alternates: {
      canonical: `/blog/${slug}`,
    },
    openGraph: {
      title: post.title,
      description: post.description,
      type: "article",
      url: `/blog/${slug}`,
      images: [image],
    },
    twitter: {
      card: "summary_large_image",
      title: post.title,
      description: post.description,
      images: [image],
    },
  };
}

export default async function BlogPostPage(props: PageProps<"/blog/[slug]">) {
  const { slug } = await props.params;
  const post = await getPostBySlug(slug);

  if (!post) {
    notFound();
  }

  const { Content } = post;

  const articleSchema = {
    "@context": "https://schema.org",
    "@type": "BlogPosting",
    headline: post.title,
    description: post.description,
    datePublished: post.date,
    url: `https://slowdoctor.dev/blog/${slug}`,
    author: {
      "@type": "Person",
      name: "Joonho Lim",
      url: "https://slowdoctor.dev",
    },
    image: post.image ?? "/og-default.png",
  };

  const breadcrumbSchema = buildBreadcrumbSchema([
    { name: "Home", href: "/" },
    { name: "Blog", href: "/blog" },
    { name: post.title, href: `/blog/${slug}` },
  ]);

  return (
    <div className="mx-auto max-w-3xl px-6">
      <JsonLd data={articleSchema} />
      <JsonLd data={breadcrumbSchema} />
      <section className="pt-24 pb-12 sm:pt-32 sm:pb-16">
        <Link href="/blog" className="text-sm text-accent hover:underline">
          &larr; Back to blog
        </Link>
        <h1 className="mt-6 text-4xl font-bold tracking-tight sm:text-5xl">
          {post.title}
        </h1>
        <p className="mt-4 text-sm text-muted">{post.formattedDate}</p>
      </section>

      <article className="prose pb-24">
        <Content />
      </article>
    </div>
  );
}
