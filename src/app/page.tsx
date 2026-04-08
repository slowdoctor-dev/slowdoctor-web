import type { Metadata } from "next";
import Link from "next/link";
import { SITE, AUTHOR, DESCRIPTIONS } from "@/lib/config";
import { getAllPosts } from "@/lib/blog";
import { buildPageMetadata } from "@/lib/metadata";

export const metadata: Metadata = buildPageMetadata({
  title: SITE.title,
  description: DESCRIPTIONS.full,
  path: "/",
  absoluteTitle: true,
});

export default async function Home() {
  const posts = await getAllPosts();
  const recent = posts.slice(0, 3);

  return (
    <div className="mx-auto max-w-3xl px-6">
      {/* Hero */}
      <section className="pt-24 pb-16 sm:pt-32 sm:pb-20">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          {AUTHOR.name}
        </h1>
        <p className="mt-4 text-lg text-accent font-medium">
          Choosing the right way over the fast way.
        </p>
        <p className="mt-4 text-lg text-muted leading-relaxed">
          {DESCRIPTIONS.brief}
        </p>
        <p className="mt-6 text-foreground/90 leading-relaxed">
          I practice both surgery and non-surgical medicine — so the
          recommendation is always what you actually need. Outside the
          clinic, I explore CS and AI, looking for where they can solve
          real problems.
        </p>
      </section>

      {/* Two Axes */}
      <section className="pb-16 grid grid-cols-1 sm:grid-cols-2 gap-4">
        <Link
          href="/physician"
          className="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
        >
          <h2 className="text-lg font-semibold group-hover:text-accent transition-colors">
            As a Physician
          </h2>
          <p className="mt-2 text-sm text-muted">
            Slow-aging, scars, and natural eyes.
          </p>
          <span className="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
            Learn more &rarr;
          </span>
        </Link>
        <Link
          href="/engineer"
          className="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
        >
          <h2 className="text-lg font-semibold group-hover:text-accent transition-colors">
            As an Engineer
          </h2>
          <p className="mt-2 text-sm text-muted">
            AI, automation, and real-world problem solving.
          </p>
          <span className="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
            Learn more &rarr;
          </span>
        </Link>
      </section>

      {/* Latest */}
      {recent.length > 0 && (
        <section className="pb-24">
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-sm font-medium text-accent uppercase tracking-wider">
              Latest
            </h2>
            <Link
              href="/blog"
              className="text-sm text-muted hover:text-foreground transition-colors"
            >
              All posts &rarr;
            </Link>
          </div>
          <div className="space-y-3">
            {recent.map((post) => (
              <Link
                key={post.slug}
                href={`/blog/${post.slug}`}
                className="flex items-baseline justify-between gap-4 rounded-2xl border border-border bg-card px-5 py-4 transition-colors hover:border-accent/30"
              >
                <span className="text-foreground font-medium truncate">
                  {post.title}
                </span>
                <span className="text-sm text-muted whitespace-nowrap shrink-0">
                  {post.formattedDate}
                </span>
              </Link>
            ))}
          </div>
        </section>
      )}
    </div>
  );
}
