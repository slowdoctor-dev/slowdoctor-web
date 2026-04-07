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
  image?: string;
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

function parseFrontmatter(data: unknown, fileName: string): BlogFrontmatter {
  if (typeof data !== "object" || data === null) {
    throw new Error(`Invalid frontmatter in ${fileName}`);
  }
  const obj = data as Record<string, unknown>;
  const missing: string[] = [];
  if (typeof obj.title !== "string" || !obj.title) missing.push("title");
  if (typeof obj.date !== "string" || !obj.date) missing.push("date");
  if (typeof obj.description !== "string" || !obj.description)
    missing.push("description");
  if (missing.length > 0) {
    throw new Error(
      `Missing required frontmatter in ${fileName}: ${missing.join(", ")}`,
    );
  }
  return {
    title: obj.title as string,
    date: obj.date as string,
    description: obj.description as string,
    image: typeof obj.image === "string" ? obj.image : undefined,
  };
}

async function readBlogFrontmatter(fileName: string) {
  const fullPath = path.join(blogDirectory, fileName);
  const fileContents = await fs.readFile(fullPath, "utf8");
  const { data } = matter(fileContents);
  const frontmatter = parseFrontmatter(data, fileName);

  return {
    slug: fileName.replace(/\.mdx$/, ""),
    ...frontmatter,
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
  const frontmatter = parseFrontmatter(data, `${slug}.mdx`);

  const module = (await evaluate(content, {
    ...runtime,
    useMDXComponents,
  })) as {
    default: ComponentType;
  };

  return {
    slug,
    ...frontmatter,
    formattedDate: formatDate(frontmatter.date),
    Content: module.default,
  };
}
