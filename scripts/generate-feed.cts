const fs = require("node:fs/promises");
const path = require("node:path");
const matter = require("gray-matter");

const { parseDateOnly, stripDatePrefix, siteUrl } = require("./date-utils.cts");
const blogDirectory = path.join(process.cwd(), "src/content/blog");
const feedPath = path.join(process.cwd(), "public/feed.xml");

function escapeXml(value: unknown) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&apos;");
}

async function getPosts() {
  const entries = await fs.readdir(blogDirectory, { withFileTypes: true });
  const posts = [];

  for (const entry of entries) {
    if (!entry.isFile() || !entry.name.endsWith(".mdx")) {
      continue;
    }

    const slug = stripDatePrefix(entry.name.replace(/\.mdx$/, ""));
    const fileContents = await fs.readFile(path.join(blogDirectory, entry.name), "utf8");
    const { data } = matter(fileContents);

    if (!data.title || !data.date || !data.description) {
      throw new Error(`Missing required frontmatter in ${entry.name}`);
    }

    posts.push({
      title: data.title,
      date: parseDateOnly(String(data.date), entry.name),
      description: data.description,
      slug,
    });
  }

  return posts.sort((left, right) => right.date.getTime() - left.date.getTime());
}

async function main() {
  const posts = await getPosts();
  const items = posts.map((post) => {
    const link = `${siteUrl}/blog/${post.slug}`;

    return [
      "    <item>",
      `      <title>${escapeXml(post.title)}</title>`,
      `      <link>${link}</link>`,
      `      <description>${escapeXml(post.description)}</description>`,
      `      <pubDate>${post.date.toUTCString()}</pubDate>`,
      `      <guid isPermaLink="true">${link}</guid>`,
      "    </item>",
    ].join("\n");
  });

  const feed = [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<rss version="2.0">',
    "  <channel>",
    "    <title>Joonho Lim - Blog</title>",
    `    <link>${siteUrl}/blog</link>`,
    "    <description>Writing by Joonho Lim on medicine, engineering, and the slower path.</description>",
    "    <language>en-us</language>",
    ...items,
    "  </channel>",
    "</rss>",
    "",
  ].join("\n");

  await fs.writeFile(feedPath, feed, "utf8");
}

main();
