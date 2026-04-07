import { promises as fs } from "node:fs";
import path from "node:path";
import { evaluate } from "@mdx-js/mdx";
import type { ComponentType } from "react";
import * as runtime from "react/jsx-runtime";
import matter from "gray-matter";
import { useMDXComponents } from "@/mdx-components";

const blogDirectory = path.join(process.cwd(), "src/content/blog");

interface BlogFrontmatter {
  title: string;
  date: string;
  description: string;
}

export interface BlogPostSummary extends BlogFrontmatter {
  slug: string;
  formattedDate: string;
}

export interface BlogPost extends BlogPostSummary {
  Content: ComponentType;
}

function formatDate(date: string) {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  }).format(new Date(date));
}

async function readBlogFrontmatter(fileName: string) {
  const fullPath = path.join(blogDirectory, fileName);
  const fileContents = await fs.readFile(fullPath, "utf8");
  const { data } = matter(fileContents);
  const frontmatter = data as Partial<BlogFrontmatter>;

  if (!frontmatter.title || !frontmatter.date || !frontmatter.description) {
    throw new Error(`Missing required frontmatter in ${fileName}`);
  }

  return {
    slug: fileName.replace(/\.mdx$/, ""),
    title: frontmatter.title,
    date: frontmatter.date,
    description: frontmatter.description,
    formattedDate: formatDate(frontmatter.date),
  } satisfies BlogPostSummary;
}

export async function getAllPosts(): Promise<BlogPostSummary[]> {
  const entries = await fs.readdir(blogDirectory);
  const posts = await Promise.all(
    entries.filter((entry) => entry.endsWith(".mdx")).map(readBlogFrontmatter),
  );

  return posts.sort(
    (left, right) =>
      new Date(right.date).getTime() - new Date(left.date).getTime(),
  );
}

export async function getPostFrontmatter(
  slug: string,
): Promise<BlogPostSummary | null> {
  try {
    return await readBlogFrontmatter(`${slug}.mdx`);
  } catch {
    return null;
  }
}

export async function getPostBySlug(slug: string): Promise<BlogPost | null> {
  const fullPath = path.join(blogDirectory, `${slug}.mdx`);

  let fileContents: string;
  try {
    fileContents = await fs.readFile(fullPath, "utf8");
  } catch {
    return null;
  }

  const { content, data } = matter(fileContents);
  const frontmatter = data as Partial<BlogFrontmatter>;

  if (!frontmatter.title || !frontmatter.date || !frontmatter.description) {
    throw new Error(`Missing required frontmatter in ${slug}.mdx`);
  }

  const module = (await evaluate(content, {
    ...runtime,
    useMDXComponents,
  })) as {
    default: ComponentType;
  };

  return {
    slug,
    title: frontmatter.title,
    date: frontmatter.date,
    description: frontmatter.description,
    formattedDate: formatDate(frontmatter.date),
    Content: module.default,
  };
}
