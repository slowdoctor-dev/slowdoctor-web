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

const date = new Date().toISOString().split("T")[0];
const filePath = path.join(process.cwd(), "src/content/blog", `${slug}.mdx`);

if (fs.existsSync(filePath)) {
  console.error(`File already exists: ${filePath}`);
  process.exit(1);
}

const content = `---
title: "${title}"
date: "${date}"
description: "TODO: Write a short description for this post."
---

`;

fs.writeFileSync(filePath, content, "utf8");
console.log(`Created: src/content/blog/${slug}.mdx`);
