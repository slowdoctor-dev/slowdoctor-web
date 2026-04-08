import { promises as fs } from "node:fs";
import path from "node:path";
import { evaluate } from "@mdx-js/mdx";
import type { ComponentType } from "react";
import * as runtime from "react/jsx-runtime";
import matter from "gray-matter";
import { useMDXComponents } from "@/mdx-components";

const blogDirectory = path.join(process.cwd(), "src/content/blog");

export interface Axes {
  physician: number;
  engineer: number;
  life: number;
}

interface BlogFrontmatter {
  title: string;
  date: string;
  description: string;
  image?: string;
  tags?: string[];
  axes?: Axes;
}

export interface BlogPostSummary extends BlogFrontmatter {
  slug: string;
  formattedDate: string;
}

export interface BlogPost extends BlogPostSummary {
  Content: ComponentType;
}

const DATE_ONLY_PATTERN = /^\d{4}-\d{2}-\d{2}$/;

function parseDateOnly(date: string, fileName: string) {
  if (!DATE_ONLY_PATTERN.test(date)) {
    throw new Error(
      `Invalid date in ${fileName}: expected YYYY-MM-DD, received "${date}"`,
    );
  }

  const [year, month, day] = date.split("-").map(Number);
  const parsed = new Date(Date.UTC(year, month - 1, day));

  if (
    parsed.getUTCFullYear() !== year ||
    parsed.getUTCMonth() !== month - 1 ||
    parsed.getUTCDate() !== day
  ) {
    throw new Error(`Invalid calendar date in ${fileName}: "${date}"`);
  }

  return parsed;
}

function formatDate(date: string) {
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    timeZone: "UTC",
  }).format(parseDateOnly(date, "frontmatter"));
}

function parseAxes(raw: unknown): Axes | undefined {
  if (typeof raw !== "object" || raw === null) return undefined;
  const obj = raw as Record<string, unknown>;
  const p = Number(obj.physician);
  const e = Number(obj.engineer);
  const l = Number(obj.life);
  const values = [p, e, l];
  const hasInvalidValue = values.some(
    (value) => !Number.isFinite(value) || !Number.isInteger(value),
  );
  if (hasInvalidValue) return undefined;
  const isOutOfRange = values.some((value) => value < 0 || value > 10);
  if (isOutOfRange) return undefined;
  if (p + e + l !== 10) return undefined;
  return { physician: p, engineer: e, life: l };
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
  const date = obj.date as string;
  parseDateOnly(date, fileName);
  return {
    title: obj.title as string,
    date,
    description: obj.description as string,
    image: typeof obj.image === "string" ? obj.image : undefined,
    tags: Array.isArray(obj.tags)
      ? obj.tags.filter((t): t is string => typeof t === "string")
      : undefined,
    axes: parseAxes(obj.axes),
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

  return posts.sort((left, right) => right.date.localeCompare(left.date));
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

  const evaluatedContent = (await evaluate(content, {
    ...runtime,
    useMDXComponents,
  })) as {
    default: ComponentType;
  };

  return {
    slug,
    ...frontmatter,
    formattedDate: formatDate(frontmatter.date),
    Content: evaluatedContent.default,
  };
}
