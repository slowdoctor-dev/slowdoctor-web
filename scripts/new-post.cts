const fs = require("node:fs");
const path = require("node:path");

const title = process.argv.slice(2).join(" ").trim();

if (!title) {
  console.error("Usage: npm run new-post -- \"My Post Title\"");
  process.exit(1);
}

const slug = title
  .toLowerCase()
  .replace(/[^a-z0-9]+/g, "-")
  .replace(/^-|-$/g, "");

if (!slug) {
  console.error(
    "Could not derive an ASCII slug from the title. Use at least one Latin letter or number.",
  );
  process.exit(1);
}

const now = new Date();
const date = [
  now.getFullYear(),
  String(now.getMonth() + 1).padStart(2, "0"),
  String(now.getDate()).padStart(2, "0"),
].join("-");
const fileName = `${date}-${slug}.mdx`;
const filePath = path.join(process.cwd(), "src/content/blog", fileName);

if (fs.existsSync(filePath)) {
  console.error(`File already exists: ${filePath}`);
  process.exit(1);
}

const escapedSlug = slug.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
const datePrefixedSlug = new RegExp(
  `^\\d{4}-\\d{2}-\\d{2}-${escapedSlug}\\.mdx$`,
);
const collidingFile = fs
  .readdirSync(path.join(process.cwd(), "src/content/blog"))
  .find(
    (entry: string) =>
      entry === `${slug}.mdx` || datePrefixedSlug.test(entry),
  );

if (collidingFile) {
  console.error(
    `Post slug "${slug}" already exists as src/content/blog/${collidingFile}`,
  );
  process.exit(1);
}

const content = `---
title: "${title}"
date: "${date}"
description: "TODO: Write a short description for this post."
---

`;

fs.writeFileSync(filePath, content, "utf8");
console.log(`Created: src/content/blog/${fileName}`);
