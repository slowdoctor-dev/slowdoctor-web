const fs = require("node:fs");
const path = require("node:path");
const matter = require("gray-matter");

const { formatDate } = require("./date-utils.cts");

const outDir = path.join(process.cwd(), "out");
let errors = 0;

function check(condition: boolean, message: string) {
  if (!condition) {
    console.error(`  FAIL: ${message}`);
    errors++;
  } else {
    console.log(`  OK: ${message}`);
  }
}

// Check build output exists
console.log("\n[Build output]");
check(fs.existsSync(outDir), "out/ directory exists");

if (!fs.existsSync(outDir)) {
  console.error("\nRun 'npm run build' first.\n");
  process.exit(1);
}

// Check generated files
console.log("\n[Generated files]");
check(fs.existsSync(path.join(process.cwd(), "public/sitemap.xml")), "sitemap.xml generated");
check(fs.existsSync(path.join(process.cwd(), "public/feed.xml")), "feed.xml generated");

// Check sitemap content
console.log("\n[Sitemap]");
const sitemap = fs.readFileSync(path.join(process.cwd(), "public/sitemap.xml"), "utf8");
check(sitemap.includes("<lastmod>"), "sitemap has <lastmod>");
check(!sitemap.includes("<priority>"), "sitemap has no <priority> (Google ignores it)");
check(!sitemap.includes("Invalid Date"), "sitemap has no invalid date values");

// Check HTML pages
console.log("\n[HTML pages]");
const pages = [
  { file: "index.html", name: "Homepage" },
  { file: "physician.html", name: "Physician" },
  { file: "engineer.html", name: "Engineer" },
  { file: "links.html", name: "Links" },
  { file: "blog.html", name: "Blog" },
];

for (const page of pages) {
  const filePath = path.join(outDir, page.file);
  if (!fs.existsSync(filePath)) {
    console.error(`  FAIL: ${page.name} — ${page.file} not found`);
    errors++;
    continue;
  }

  const html = fs.readFileSync(filePath, "utf8");
  console.log(`\n  ${page.name} (${page.file})`);
  check(html.includes("<title>"), `${page.name} has <title>`);
  check(html.includes('rel="canonical"'), `${page.name} has canonical URL`);
  check(html.includes("og:title"), `${page.name} has og:title`);

  if (page.file !== "index.html") {
    check(html.includes("BreadcrumbList"), `${page.name} has BreadcrumbList JSON-LD`);
  }
}

// Check blog posts
console.log("\n[Blog posts]");
const blogDir = path.join(process.cwd(), "src/content/blog");
const posts = fs.readdirSync(blogDir).filter((f: string) => f.endsWith(".mdx"));

for (const post of posts) {
  const slug = post.replace(/\.mdx$/, "");
  const htmlPath = path.join(outDir, "blog", `${slug}.html`);
  const fileContents = fs.readFileSync(path.join(blogDir, post), "utf8");
  const { data } = matter(fileContents);
  const expectedDateLabel = formatDate(String(data.date), post);

  if (!fs.existsSync(htmlPath)) {
    console.error(`  FAIL: /blog/${slug} — HTML not found`);
    errors++;
    continue;
  }

  const html = fs.readFileSync(htmlPath, "utf8");
  console.log(`\n  /blog/${slug}`);
  check(html.includes("BlogPosting"), `has BlogPosting JSON-LD`);
  check(html.includes("BreadcrumbList"), `has BreadcrumbList JSON-LD`);
  check(html.includes('rel="canonical"'), `has canonical URL`);
  check(html.includes(expectedDateLabel), `shows formatted publication date`);
  check(!html.includes("Invalid Date"), `has no invalid date text`);
}

// Check RSS link
console.log("\n[RSS]");
const indexHtml = fs.readFileSync(path.join(outDir, "index.html"), "utf8");
check(indexHtml.includes("application/rss+xml"), "RSS link tag in layout");
const feed = fs.readFileSync(path.join(process.cwd(), "public/feed.xml"), "utf8");
check(!feed.includes("Invalid Date"), "feed has no invalid date values");

// Summary
console.log(`\n${"=".repeat(40)}`);
if (errors === 0) {
  console.log("All checks passed.\n");
} else {
  console.error(`${errors} check(s) failed.\n`);
  process.exit(1);
}
