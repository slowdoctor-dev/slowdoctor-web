import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { getAllPosts, getPostBySlug, getPostFrontmatter } from "@/lib/blog";
import { JsonLd } from "@/components/json-ld";
import { buildBreadcrumbSchema } from "@/lib/breadcrumbs";
import { SITE, AUTHOR, PRACTICE } from "@/lib/config";
import { practiceUrl } from "@/lib/links";
import { doctor } from "@/data/doctor";
import { AxisBar } from "@/components/axis-bar";

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
  const canonicalUrl = new URL(`/blog/${slug}`, SITE.url).toString();

  return {
    title: post.title,
    description: post.description,
    authors: [{ name: AUTHOR.name, url: SITE.url }],
    creator: AUTHOR.name,
    publisher: SITE.name,
    alternates: {
      canonical: canonicalUrl,
    },
    openGraph: {
      title: post.title,
      description: post.description,
      type: "article",
      url: canonicalUrl,
      siteName: SITE.name,
      locale: "en_US",
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
    url: `${SITE.url}/blog/${slug}`,
    inLanguage: "en",
    author: {
      "@type": "Person",
      "@id": doctor.id,
      name: AUTHOR.name,
      url: `${SITE.url}/cv`,
      jobTitle: AUTHOR.jobTitle,
      worksFor: {
        "@type": "MedicalBusiness",
        "@id": doctor.worksFor.id,
        name: PRACTICE.fullName,
        url: practiceUrl,
      },
    },
    publisher: {
      "@type": "Person",
      "@id": doctor.id,
      name: AUTHOR.name,
    },
    image: `${SITE.url}${post.image ?? SITE.ogImage}`,
    ...(post.tags && post.tags.length > 0 && { keywords: post.tags.join(", ") }),
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
        <h1 className="mt-6 text-3xl font-bold tracking-tight sm:text-4xl">
          {post.title}
        </h1>
        <p className="mt-4 text-sm text-muted">
          By <Link href="/cv" className="text-foreground hover:text-accent transition-colors">{AUTHOR.name}</Link>
        </p>
        <p className="mt-2 text-sm text-muted">{post.formattedDate}</p>
        {(post.axes || (post.tags && post.tags.length > 0)) && (
          <div className="mt-6 flex flex-col sm:flex-row sm:items-start gap-4">
            {post.axes && (
              <div className="sm:w-48">
                <AxisBar values={post.axes} />
              </div>
            )}
            {post.tags && post.tags.length > 0 && (
              <div className="flex flex-wrap gap-1.5">
                {post.tags.map((tag) => (
                  <span
                    key={tag}
                    className="text-xs text-muted border border-border rounded-full px-2 py-0.5"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            )}
          </div>
        )}
      </section>

      <article className="prose pb-24">
        <Content />
      </article>
    </div>
  );
}
