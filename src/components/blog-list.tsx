"use client";

import { useState } from "react";
import Link from "next/link";
import type { Axes } from "@/lib/blog";
import { AxisBar } from "@/components/axis-bar";

interface PostData {
  slug: string;
  title: string;
  description: string;
  formattedDate: string;
  tags?: string[];
  axes?: Axes;
}

export function BlogList({ posts }: { posts: PostData[] }) {
  const [activeTag, setActiveTag] = useState<string | null>(null);

  const allTags = Array.from(
    new Set(posts.flatMap((p) => p.tags ?? [])),
  ).sort();

  const filtered = activeTag
    ? posts.filter((p) => p.tags?.includes(activeTag))
    : posts;

  return (
    <>
      {allTags.length > 0 && (
        <div className="pb-8 flex flex-wrap gap-1.5">
          <button
            type="button"
            onClick={() => setActiveTag(null)}
            aria-pressed={activeTag === null}
            className={`text-xs rounded-full px-2.5 py-1 transition-colors ${
              activeTag === null
                ? "bg-accent text-background"
                : "text-muted border border-border hover:text-foreground"
            }`}
          >
            All
          </button>
          {allTags.map((tag) => (
            <button
              key={tag}
              type="button"
              onClick={() => setActiveTag(activeTag === tag ? null : tag)}
              aria-pressed={activeTag === tag}
              className={`text-xs rounded-full px-2.5 py-1 transition-colors ${
                activeTag === tag
                  ? "bg-accent text-background"
                  : "text-muted border border-border hover:text-foreground"
              }`}
            >
              {tag}
            </button>
          ))}
        </div>
      )}

      <div className="space-y-4">
        {filtered.map((post) => (
          <article
            key={post.slug}
            className="rounded-lg border border-border bg-card p-5 transition-colors hover:border-accent/30"
          >
            <div className="flex flex-col sm:flex-row sm:gap-6">
              <div className="flex-1 min-w-0">
                <p className="text-sm text-muted">{post.formattedDate}</p>
                <h2 className="mt-2 text-xl font-semibold text-foreground">
                  <Link
                    href={`/blog/${post.slug}`}
                    className="hover:text-accent"
                  >
                    {post.title}
                  </Link>
                </h2>
                <p className="mt-2 text-sm leading-relaxed text-muted">
                  {post.description}
                </p>
                {post.tags && post.tags.length > 0 && (
                  <div className="mt-3 flex flex-wrap gap-1.5">
                    {post.tags.map((tag) => (
                      <button
                        key={tag}
                        type="button"
                        onClick={() =>
                          setActiveTag(activeTag === tag ? null : tag)
                        }
                        aria-pressed={activeTag === tag}
                        className={`text-xs rounded-full px-2 py-0.5 transition-colors ${
                          activeTag === tag
                            ? "bg-accent text-background"
                            : "text-muted border border-border hover:text-foreground"
                        }`}
                      >
                        {tag}
                      </button>
                    ))}
                  </div>
                )}
              </div>
              {post.axes && (
                <div className="mt-4 sm:mt-0 sm:w-44 shrink-0">
                  <AxisBar values={post.axes} />
                </div>
              )}
            </div>
          </article>
        ))}
      </div>
    </>
  );
}
