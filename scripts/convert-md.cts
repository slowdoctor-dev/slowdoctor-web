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
  if (leadMatch) {
    return leadMatch[1];
  }

  // Already kebab-case
  const kebab = base
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");

  return kebab || "untitled";
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
    if (p.length >= 20) {
      // Truncate to ~160 chars for SEO
      if (p.length > 160) {
        return p.slice(0, 157) + "...";
      }
      return p;
    }
  }

  return "TODO: Write a short description for this post.";
}

/** Strip the first H1 heading from content (since title goes to frontmatter). */
function stripLeadingH1(content: string): string {
  const lines = content.split("\n");
  let foundH1 = false;

  const result: string[] = [];
  for (const line of lines) {
    if (!foundH1 && line.trim().match(/^#\s+/)) {
      foundH1 = true;
      continue;
    }
    result.push(line);
  }

  // Trim leading blank lines
  while (result.length > 0 && !result[0].trim()) {
    result.shift();
  }

  return result.join("\n");
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
  const targetPath = path.join(blogDir, `${slug}.mdx`);

  // Check for collision
  if (fs.existsSync(targetPath)) {
    return {
      source: fileName,
      target: `${slug}.mdx`,
      slug,
      skipped: "target file already exists",
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
    target: `${slug}.mdx`,
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
    if (!fs.existsSync(path.join(incomingDir, resolved))) {
      console.error(`File not found: ${path.join(incomingDir, resolved)}`);
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
