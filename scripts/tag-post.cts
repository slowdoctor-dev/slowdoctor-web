const fs = require("node:fs");
const path = require("node:path");
const matter = require("gray-matter");

const slug = process.argv.slice(2).join(" ").trim();

if (!slug) {
  console.error('Usage: npm run tag-post -- "slug"');
  process.exit(1);
}

const filePath = path.join(process.cwd(), "src/content/blog", `${slug}.mdx`);

if (!fs.existsSync(filePath)) {
  console.error(`File not found: ${filePath}`);
  process.exit(1);
}

const fileContents = fs.readFileSync(filePath, "utf8");
const { data, content } = matter(fileContents);

if (data.tags && data.axes) {
  console.log(`Already tagged: ${slug}`);
  console.log(`  Tags: ${data.tags.join(", ")}`);
  console.log(
    `  Axes: physician=${data.axes.physician} engineer=${data.axes.engineer} life=${data.axes.life}`,
  );
  const force = process.argv.includes("--force");
  if (!force) {
    console.log("Use --force to re-tag.");
    process.exit(0);
  }
  console.log("Re-tagging (--force)...\n");
}

const apiKey = process.env.ANTHROPIC_API_KEY;
if (!apiKey) {
  console.error(
    "ANTHROPIC_API_KEY not set. Export it or add to your environment.",
  );
  process.exit(1);
}

const prompt = `You are a blog post classifier for slowdoctor.dev, a personal site by a board-certified plastic surgeon who is also an engineer.

Analyze this blog post and return a JSON object with:
1. "tags": an array of 2-5 lowercase English tags (no spaces, use hyphens). Tags should be specific and descriptive.
2. "axes": an object with three keys — "physician", "engineer", "life" — each an integer 0-10. The three values MUST sum to exactly 10.

Axis definitions:
- physician: medical knowledge, clinical practice, surgery, dermatology, slow-aging, evidence-based medicine
- engineer: software engineering, AI, automation, programming, data science, system design
- life: personal reflection, career, identity, daily life, philosophy, non-professional topics

Title: ${data.title}
Description: ${data.description || ""}

Content:
${content.slice(0, 3000)}

Respond with ONLY the JSON object, no explanation.`;

function isValidAxisValue(value) {
  return Number.isInteger(value) && value >= 0 && value <= 10;
}

function isValidTag(tag) {
  return typeof tag === "string" && /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(tag);
}

async function main() {
  const response = await fetch("https://api.anthropic.com/v1/messages", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-api-key": apiKey,
      "anthropic-version": "2023-06-01",
    },
    body: JSON.stringify({
      model: "claude-haiku-4-5-20251001",
      max_tokens: 256,
      messages: [{ role: "user", content: prompt }],
    }),
  });

  if (!response.ok) {
    const errorText = await response.text();
    console.error(`API error (${response.status}): ${errorText}`);
    process.exit(1);
  }

  const result = await response.json();
  const text = result.content[0].text.trim();

  let parsed;
  try {
    parsed = JSON.parse(text);
  } catch {
    console.error("Failed to parse AI response:", text);
    process.exit(1);
  }

  const { tags, axes } = parsed;

  if (
    !Array.isArray(tags) ||
    typeof axes !== "object" ||
    typeof axes.physician !== "number" ||
    typeof axes.engineer !== "number" ||
    typeof axes.life !== "number"
  ) {
    console.error("Invalid AI response structure:", parsed);
    process.exit(1);
  }

  if (tags.length < 2 || tags.length > 5 || !tags.every(isValidTag)) {
    console.error("Invalid tags returned by AI:", tags);
    process.exit(1);
  }

  if (
    !isValidAxisValue(axes.physician) ||
    !isValidAxisValue(axes.engineer) ||
    !isValidAxisValue(axes.life)
  ) {
    console.error("Invalid axis values returned by AI:", axes);
    process.exit(1);
  }

  const sum = axes.physician + axes.engineer + axes.life;
  if (sum !== 10) {
    console.error(`Axes sum is ${sum}, expected 10. Response:`, axes);
    process.exit(1);
  }

  data.tags = tags;
  data.axes = {
    physician: axes.physician,
    engineer: axes.engineer,
    life: axes.life,
  };

  const updated = matter.stringify(content, data);
  fs.writeFileSync(filePath, updated, "utf8");

  console.log(`Tagged: ${slug}`);
  console.log(`  Tags: ${tags.join(", ")}`);
  console.log(
    `  Axes: physician=${axes.physician} engineer=${axes.engineer} life=${axes.life}`,
  );
}

main();
