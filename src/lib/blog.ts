import { promises as fs } from "node:fs";
import path from "node:path";
import { evaluate } from "@mdx-js/mdx";
import type { ComponentType } from "react";
import { cache } from "react";
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
const DATE_PREFIX_PATTERN = /^\d{4}-\d{2}-\d{2}-/;

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

function parseTags(raw: unknown): string[] | undefined {
  if (!Array.isArray(raw)) return undefined;

  const tags = Array.from(
    new Set(
      raw
        .filter((tag): tag is string => typeof tag === "string")
        .map((tag) => tag.trim())
        .filter(Boolean),
    ),
  );

  return tags.length > 0 ? tags : undefined;
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
    tags: parseTags(obj.tags),
    axes: parseAxes(obj.axes),
  };
}

const readBlogFrontmatter = cache(async (fileName: string) => {
  const fullPath = path.join(blogDirectory, fileName);
  const fileContents = await fs.readFile(fullPath, "utf8");
  const { data } = matter(fileContents);
  const frontmatter = parseFrontmatter(data, fileName);

  // Strip YYYY-MM-DD- date prefix from filename to produce a clean slug.
  // File: 2026-04-07-hello-world.mdx → slug: hello-world → URL: /blog/hello-world
  const stem = fileName.replace(/\.mdx$/, "");
  const slug = stem.replace(DATE_PREFIX_PATTERN, "");

  return {
    slug,
    ...frontmatter,
    formattedDate: formatDate(frontmatter.date),
  } satisfies BlogPostSummary;
});

export const getAllPosts = cache(async (): Promise<BlogPostSummary[]> => {
  const entries = await fs.readdir(blogDirectory);
  const posts = await Promise.all(
    entries.filter((entry) => entry.endsWith(".mdx")).map(readBlogFrontmatter),
  );

  return posts.sort((left, right) => right.date.localeCompare(left.date));
});

/**
 * Resolve a clean slug to its .mdx filename on disk.
 * Handles both date-prefixed (2026-04-07-hello-world.mdx) and plain (hello-world.mdx) filenames.
 */
async function resolveSlugToFile(slug: string): Promise<string | null> {
  // Try exact match first (supports legacy non-prefixed files)
  const exactName = `${slug}.mdx`;
  try {
    await fs.access(path.join(blogDirectory, exactName));
    return exactName;
  } catch {
    // Not found — try date-prefixed pattern
  }

  // Scan directory for YYYY-MM-DD-{slug}.mdx
  const entries = await fs.readdir(blogDirectory);
  const pattern = new RegExp(`^\\d{4}-\\d{2}-\\d{2}-${slug.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\.mdx$`);
  const match = entries.find((entry) => pattern.test(entry));
  return match ?? null;
}

export const getPostFrontmatter = cache(async (
  slug: string,
): Promise<BlogPostSummary | null> => {
  const fileName = await resolveSlugToFile(slug);
  if (!fileName) return null;
  try {
    return await readBlogFrontmatter(fileName);
  } catch {
    return null;
  }
});

export const getPostBySlug = cache(async (
  slug: string,
): Promise<BlogPost | null> => {
  const fileName = await resolveSlugToFile(slug);
  if (!fileName) return null;

  const fullPath = path.join(blogDirectory, fileName);

  let fileContents: string;
  try {
    fileContents = await fs.readFile(fullPath, "utf8");
  } catch {
    return null;
  }

  const { content, data } = matter(fileContents);
  const frontmatter = parseFrontmatter(data, fileName);

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
});
