const fs = require("node:fs/promises");
const path = require("node:path");

const siteUrl = "https://slowdoctor.dev";
const appDirectory = path.join(process.cwd(), "src/app");
const blogDirectory = path.join(process.cwd(), "src/content/blog");
const sitemapPath = path.join(process.cwd(), "public/sitemap.xml");

async function getStaticRoutes(directory, segments = []) {
  const entries = await fs.readdir(directory, { withFileTypes: true });
  const routes = new Set();

  for (const entry of entries) {
    if (entry.isDirectory()) {
      if (entry.name.startsWith("[") || entry.name.startsWith("(")) {
        continue;
      }

      const nestedRoutes = await getStaticRoutes(path.join(directory, entry.name), [
        ...segments,
        entry.name,
      ]);

      for (const route of nestedRoutes) {
        routes.add(route);
      }

      continue;
    }

    if (!entry.isFile()) {
      continue;
    }

    const extension = path.extname(entry.name);
    const baseName = path.basename(entry.name, extension);

    if (![".js", ".jsx", ".ts", ".tsx", ".md", ".mdx"].includes(extension)) {
      continue;
    }

    if (baseName !== "page") {
      continue;
    }

    routes.add(segments.length === 0 ? "/" : `/${segments.join("/")}`);
  }

  return routes;
}

async function getBlogRoutes() {
  const entries = await fs.readdir(blogDirectory, { withFileTypes: true });

  return entries
    .filter((entry) => entry.isFile() && entry.name.endsWith(".mdx"))
    .map((entry) => `/blog/${entry.name.replace(/\.mdx$/, "")}`)
    .sort();
}

function createUrl(route, priority) {
  return [
    "  <url>",
    `    <loc>${siteUrl}${route === "/" ? "/" : route}</loc>`,
    `    <priority>${priority}</priority>`,
    "  </url>",
  ].join("\n");
}

async function main() {
  const staticRoutes = Array.from(await getStaticRoutes(appDirectory)).sort();
  const blogRoutes = await getBlogRoutes();
  const otherStaticRoutes = staticRoutes.filter(
    (route) => route !== "/" && route !== "/blog",
  );

  const sitemap = [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
    createUrl("/", "1.0"),
    ...otherStaticRoutes.map((route) => createUrl(route, "0.8")),
    createUrl("/blog", "0.7"),
    ...blogRoutes.map((route) => createUrl(route, "0.5")),
    "</urlset>",
    "",
  ].join("\n");

  await fs.writeFile(sitemapPath, sitemap, "utf8");
}

main();
