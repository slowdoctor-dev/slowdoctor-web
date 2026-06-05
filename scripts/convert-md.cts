/**
 * convert-md.cts — Convert incoming Markdown drafts to MDX blog posts.
 *
 * Usage:
 *   npm run convert              # convert all files in src/content/incoming/
 *   npm run convert -- file.md   # convert a single file
 *
 * The script reads Markdown files from src/content/incoming/, generates
 * frontmatter fields required by the blog (title, date, description),
 * and writes .mdx files to src/content/blog/.
 *
 * Input files are expected from lead/creative-content with the naming
 * convention: YYYY-MM-DD_{channel}_{english-kebab-slug}.md
 *
 * After successful conversion the source file is removed from incoming/.
 *
 * Tags and axes are NOT auto-generated — Claude Code handles classification
 * directly after conversion.
 */

const fs = require("node:fs");
const path = require("node:path");
const matter = require("gray-matter");

const { parseDateOnly } = require("./date-utils.cts");
const incomingDir = path.join(process.cwd(), "src/content/incoming");
const blogDir = path.join(process.cwd(), "src/content/blog");

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** Extract a slug from the lead filename convention or fall back to generic. */
function deriveSlug(fileName: string): string {
  const base = fileName.replace(/\.mdx?$/, "");

  // lead convention: YYYY-MM-DD_CHANNEL_english-kebab-slug
  const leadMatch = base.match(/^\d{4}-\d{2}-\d{2}_[A-Za-z0-9-]+_(.+)$/);
  const slugSource = leadMatch ? leadMatch[1] : base;
  const kebab = slugSource
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

  return kebab || "untitled";
}

/**
 * Find an existing blog file that resolves to the same URL slug, if any.
 * Blog filenames carry a `YYYY-MM-DD-` date prefix that is stripped to form
 * the slug (see src/lib/blog.ts), so a collision can hide behind either a
 * date-prefixed name or a legacy bare `slug.mdx`. Returns the colliding
 * filename, or null when the slug is free.
 */
function findExistingPostForSlug(slug: string): string | null {
  const escaped = slug.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const datePrefixed = new RegExp(`^\\d{4}-\\d{2}-\\d{2}-${escaped}\\.mdx$`);
  const entries = fs.readdirSync(blogDir);
  return (
    entries.find(
      (entry: string) => entry === `${slug}.mdx` || datePrefixed.test(entry),
    ) ?? null
  );
}

/** Extract date from lead filename (YYYY-MM-DD prefix) or use today. */
function deriveDate(fileName: string): string {
  const dateMatch = fileName.match(/^(\d{4}-\d{2}-\d{2})/);
  if (dateMatch) {
    return dateMatch[1];
  }

  const now = new Date();
  return [
    now.getFullYear(),
    String(now.getMonth() + 1).padStart(2, "0"),
    String(now.getDate()).padStart(2, "0"),
  ].join("-");
}

/** Try to extract a title from the first markdown heading. */
function deriveTitle(content: string, slug: string): string {
  // Check for # heading on the first non-empty line
  const lines = content.split("\n");
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) continue;
    const headingMatch = trimmed.match(/^#\s+(.+)/);
    if (headingMatch) {
      return headingMatch[1].trim();
    }
    break;
  }

  // Fall back to slug -> title case
  return slug
    .split("-")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

/** Generate a short description from the first paragraph of content. */
function deriveDescription(content: string): string {
  const lines = content.split("\n");
  const paragraphs: string[] = [];
  let current = "";

  for (const line of lines) {
    const trimmed = line.trim();

    // Skip headings, metadata lines, horizontal rules
    if (trimmed.startsWith("#") || trimmed.startsWith("---") || trimmed.startsWith("- **")) {
      if (current) {
        paragraphs.push(current);
        current = "";
      }
      continue;
    }

    if (!trimmed) {
      if (current) {
        paragraphs.push(current);
        current = "";
      }
      continue;
    }

    // Skip lines that look like stage directions [화면 텍스트: ...]
    if (trimmed.startsWith("[") && trimmed.endsWith("]")) continue;

    current += (current ? " " : "") + trimmed;
  }
  if (current) paragraphs.push(current);

  // Pick the first meaningful paragraph (at least 20 chars)
  for (const p of paragraphs) {
    const plainText = p
      .replace(/!\[.*?\]\(.*?\)/g, "")
      .replace(/\[(.*?)\]\(.*?\)/g, "$1")
      .replace(/[*_`~]/g, "")
      .trim();

    if (plainText.length >= 20) {
      // Truncate to ~160 chars for SEO
      if (plainText.length > 160) {
        return plainText.slice(0, 157) + "...";
      }
      return plainText;
    }
  }

  return "TODO: Write a short description for this post.";
}

/** Strip the first H1 heading from content (since title goes to frontmatter). */
function stripLeadingH1(content: string): string {
  const lines = content.split("\n");
  const firstNonEmptyLine = lines.findIndex((line) => line.trim());

  if (
    firstNonEmptyLine === -1 ||
    !lines[firstNonEmptyLine].trim().match(/^#\s+/)
  ) {
    return content;
  }

  lines.splice(firstNonEmptyLine, 1);

  // Trim leading blank lines
  while (lines.length > 0 && !lines[0].trim()) {
    lines.shift();
  }

  return lines.join("\n");
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

interface ConvertResult {
  source: string;
  target: string;
  slug: string;
  skipped?: string;
}

interface AxesFrontmatter {
  physician: number;
  engineer: number;
  life: number;
}

interface ExistingFrontmatter {
  title?: string;
  date?: string;
  description?: string;
  image?: string;
  tags?: unknown;
  axes?: unknown;
}

interface OutputFrontmatter {
  title: string;
  date: string;
  description: string;
  image?: string;
  tags?: string[];
  axes?: AxesFrontmatter;
}

function parseAxesFrontmatter(value: unknown): AxesFrontmatter | undefined {
  if (typeof value !== "object" || value === null) {
    return undefined;
  }

  const axes = value as Partial<AxesFrontmatter>;
  const values = [axes.physician, axes.engineer, axes.life];

  if (
    values.some((entry) => typeof entry !== "number" || !Number.isInteger(entry))
  ) {
    return undefined;
  }

  const [physician, engineer, life] = values as [number, number, number];

  if (
    physician < 0 ||
    physician > 10 ||
    engineer < 0 ||
    engineer > 10 ||
    life < 0 ||
    life > 10 ||
    physician + engineer + life !== 10
  ) {
    return undefined;
  }

  return { physician, engineer, life };
}

function convertFile(fileName: string): ConvertResult {
  const sourcePath = path.join(incomingDir, fileName);
  const raw = fs.readFileSync(sourcePath, "utf8");

  // Parse existing frontmatter if any
  const { data, content: rawContent } = matter(raw);
  const existingFm = data as ExistingFrontmatter;

  const slug = deriveSlug(fileName);
  const date =
    typeof existingFm.date === "string" ? existingFm.date : deriveDate(fileName);

  // The slug is embedded in the output filename, so the date must be a valid
  // YYYY-MM-DD — fail fast with a clear message rather than writing a file the
  // blog loader would later reject.
  parseDateOnly(date, fileName);

  // Output follows the date-prefixed convention (YYYY-MM-DD-slug.mdx), matching
  // new-post.cts and the slug-stripping in src/lib/blog.ts.
  const targetName = `${date}-${slug}.mdx`;
  const targetPath = path.join(blogDir, targetName);

  // Guard against collisions on the resolved slug, not just the exact target
  // path: an existing date-prefixed or legacy bare file can map to the same
  // URL slug and would otherwise produce duplicate /blog/[slug] routes.
  const existing = findExistingPostForSlug(slug);
  if (existing) {
    return {
      source: fileName,
      target: targetName,
      slug,
      skipped: `slug "${slug}" already exists as ${existing}`,
    };
  }

  // Build frontmatter
  const title =
    typeof existingFm.title === "string"
      ? existingFm.title
      : deriveTitle(rawContent, slug);
  const description =
    typeof existingFm.description === "string"
      ? existingFm.description
      : deriveDescription(rawContent);

  const frontmatter: OutputFrontmatter = {
    title,
    date,
    description,
  };

  if (typeof existingFm.image === "string") {
    frontmatter.image = existingFm.image;
  }

  // Preserve existing tags/axes if present in source frontmatter
  if (Array.isArray(existingFm.tags)) {
    frontmatter.tags = existingFm.tags.filter(
      (tag): tag is string => typeof tag === "string",
    );
  }

  const axes = parseAxesFrontmatter(existingFm.axes);
  if (axes) {
    frontmatter.axes = axes;
  }

  // Clean body: strip leading H1 (becomes title), trim whitespace
  let body = stripLeadingH1(rawContent).trim();

  // Ensure trailing newline
  body = body + "\n";

  // Build output
  const output = matter.stringify(body, frontmatter);

  fs.writeFileSync(targetPath, output, "utf8");

  // Remove source
  fs.unlinkSync(sourcePath);

  return {
    source: fileName,
    target: targetName,
    slug,
  };
}

function main() {
  if (!fs.existsSync(incomingDir)) {
    console.error(`Incoming directory not found: ${incomingDir}`);
    process.exit(1);
  }

  const specificFile = process.argv.slice(2).join(" ").trim();
  let files: string[];

  if (specificFile) {
    // Convert a single specified file
    const resolved = specificFile.endsWith(".md") || specificFile.endsWith(".mdx")
      ? specificFile
      : `${specificFile}.md`;
    const resolvedPath = path.resolve(incomingDir, resolved);
    const relativePath = path.relative(incomingDir, resolvedPath);
    if (relativePath.startsWith("..") || path.isAbsolute(relativePath)) {
      console.error(`File must be inside ${incomingDir}: ${specificFile}`);
      process.exit(1);
    }
    if (!fs.existsSync(resolvedPath)) {
      console.error(`File not found: ${resolvedPath}`);
      process.exit(1);
    }
    files = [resolved];
  } else {
    // Convert all .md and .mdx files in incoming/
    files = fs
      .readdirSync(incomingDir)
      .filter((f: string) => f.endsWith(".md") || f.endsWith(".mdx"))
      .sort();
  }

  if (files.length === 0) {
    console.log("No files to convert in src/content/incoming/");
    process.exit(0);
  }

  let converted = 0;
  let skipped = 0;

  for (const file of files) {
    const result = convertFile(file);
    if (result.skipped) {
      console.log(`  SKIP: ${result.source} -> ${result.target} (${result.skipped})`);
      skipped++;
    } else {
      console.log(`  OK: ${result.source} -> ${result.target}`);
      converted++;
    }
  }

  console.log(`\nConverted: ${converted}, Skipped: ${skipped}`);
}

main();
