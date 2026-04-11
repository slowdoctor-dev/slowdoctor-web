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
 * When axes or tags are missing from frontmatter, the script calls the
 * Anthropic API (claude-haiku-4-5-20251001) to classify the post automatically.
 * Set ANTHROPIC_API_KEY to enable; without it the script warns and leaves
 * axes/tags empty.
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
// AI Classification (axes + tags)
// ---------------------------------------------------------------------------

const CLASSIFY_MODEL = "claude-haiku-4-5-20251001";

const CLASSIFY_PROMPT_TEMPLATE = `You are a blog post classifier for slowdoctor.dev, a personal site by a board-certified plastic surgeon who is also an engineer.

Analyze this blog post and return a JSON object with:
1. "tags": an array of 3-5 lowercase English tags (no spaces, use hyphens). Tags should be specific and descriptive.
2. "axes": an object with three keys — "physician", "engineer", "life" — each an integer 0-10. The three values MUST sum to exactly 10.

Axis definitions:
- physician: medical knowledge, clinical practice, surgery, dermatology, slow-aging, evidence-based medicine
- engineer: software engineering, AI, automation, programming, data science, system design
- life: personal reflection, career, identity, daily life, philosophy, non-professional topics

Title: {{TITLE}}
Description: {{DESCRIPTION}}

Content:
{{CONTENT}}

Respond with ONLY the JSON object, no explanation.`;

function isValidAxisValue(value: unknown): value is number {
  return typeof value === "number" && Number.isInteger(value) && value >= 0 && value <= 10;
}

function isValidTag(tag: unknown): tag is string {
  return typeof tag === "string" && /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(tag);
}

interface ClassifyResult {
  tags: string[];
  axes: { physician: number; engineer: number; life: number };
}

async function classifyPost(
  title: string,
  description: string,
  content: string,
): Promise<ClassifyResult | null> {
  const apiKey = process.env.ANTHROPIC_API_KEY;

  if (!apiKey) {
    console.warn("  WARN: ANTHROPIC_API_KEY not set — skipping AI classification");
    return null;
  }

  const prompt = CLASSIFY_PROMPT_TEMPLATE
    .replace("{{TITLE}}", title)
    .replace("{{DESCRIPTION}}", description || "")
    .replace("{{CONTENT}}", content.slice(0, 3000));

  let response: Response;
  try {
    response = await fetch("https://api.anthropic.com/v1/messages", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-api-key": apiKey,
        "anthropic-version": "2023-06-01",
      },
      body: JSON.stringify({
        model: CLASSIFY_MODEL,
        max_tokens: 256,
        messages: [{ role: "user", content: prompt }],
      }),
    });
  } catch (err) {
    console.warn(`  WARN: API request failed — ${err}`);
    return null;
  }

  if (!response.ok) {
    const errorText = await response.text();
    console.warn(`  WARN: API error (${response.status}): ${errorText}`);
    return null;
  }

  const result = await response.json();
  const text = (result as { content: { text: string }[] }).content[0].text.trim();

  let parsed: Record<string, unknown>;
  try {
    parsed = JSON.parse(text);
  } catch {
    console.warn("  WARN: Failed to parse AI response:", text);
    return null;
  }

  const { tags, axes } = parsed as {
    tags: unknown;
    axes: unknown;
  };

  // Validate axes
  if (
    typeof axes !== "object" ||
    axes === null ||
    !isValidAxisValue((axes as Record<string, unknown>).physician) ||
    !isValidAxisValue((axes as Record<string, unknown>).engineer) ||
    !isValidAxisValue((axes as Record<string, unknown>).life)
  ) {
    console.warn("  WARN: Invalid axes from AI:", axes);
    return null;
  }

  const axesObj = axes as { physician: number; engineer: number; life: number };
  if (axesObj.physician + axesObj.engineer + axesObj.life !== 10) {
    console.warn(
      `  WARN: Axes sum is ${axesObj.physician + axesObj.engineer + axesObj.life}, expected 10`,
    );
    return null;
  }

  // Validate tags
  if (!Array.isArray(tags) || tags.length < 3 || tags.length > 5 || !tags.every(isValidTag)) {
    console.warn("  WARN: Invalid tags from AI:", tags);
    return null;
  }

  return {
    tags: tags as string[],
    axes: {
      physician: axesObj.physician,
      engineer: axesObj.engineer,
      life: axesObj.life,
    },
  };
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

async function convertFile(fileName: string): Promise<ConvertResult> {
  const sourcePath = path.join(incomingDir, fileName);
  const raw = fs.readFileSync(sourcePath, "utf8");

  // Parse existing frontmatter if any
  const { data: existingFm, content: rawContent } = matter(raw);

  const slug = deriveSlug(fileName);
  const date = (existingFm.date as string) || deriveDate(fileName);
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
  const title = (existingFm.title as string) || deriveTitle(rawContent, slug);
  const description =
    (existingFm.description as string) || deriveDescription(rawContent);

  const frontmatter: Record<string, unknown> = {
    title,
    date,
    description,
  };

  if (existingFm.image) {
    frontmatter.image = existingFm.image;
  }

  // Preserve existing tags/axes if present
  const hasExistingTags = existingFm.tags && Array.isArray(existingFm.tags);
  const hasExistingAxes =
    existingFm.axes &&
    typeof existingFm.axes === "object" &&
    typeof existingFm.axes.physician === "number";

  if (hasExistingTags) {
    frontmatter.tags = existingFm.tags;
  }

  if (hasExistingAxes) {
    frontmatter.axes = existingFm.axes;
  }

  // Auto-classify with AI when tags or axes are missing
  if (!hasExistingTags || !hasExistingAxes) {
    const classified = await classifyPost(title, description, rawContent);

    if (classified) {
      if (!hasExistingTags) {
        frontmatter.tags = classified.tags;
        console.log(`    AI tags: ${classified.tags.join(", ")}`);
      }
      if (!hasExistingAxes) {
        frontmatter.axes = classified.axes;
        console.log(
          `    AI axes: physician=${classified.axes.physician} engineer=${classified.axes.engineer} life=${classified.axes.life}`,
        );
      }
    }
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

async function main() {
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
    const result = await convertFile(file);
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
