const fs = require("node:fs/promises");
const path = require("node:path");
const matter = require("gray-matter");

const { parseDateOnly, stripDatePrefix, siteUrl } = require("./date-utils.cts");
const appDirectory = path.join(process.cwd(), "src/app");
const blogDirectory = path.join(process.cwd(), "src/content/blog");
const sitemapPath = path.join(process.cwd(), "public/sitemap.xml");

function escapeXml(value: string) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

async function getStaticRoutes(directory: string, segments: string[] = []) {
  const entries = await fs.readdir(directory, { withFileTypes: true });
  const routes = new Set<string>();

  for (const entry of entries) {
    if (entry.isDirectory()) {
      if (entry.name.startsWith("[")) {
        continue;
      }

      if (entry.name.startsWith("(")) {
        const groupedRoutes = await getStaticRoutes(
          path.join(directory, entry.name),
          segments,
        );

        for (const route of groupedRoutes) {
          routes.add(route);
        }

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
  const results = [];

  for (const entry of entries) {
    if (!entry.isFile() || !entry.name.endsWith(".mdx")) continue;
    const content = await fs.readFile(path.join(blogDirectory, entry.name), "utf8");
    const { data } = matter(content);
    const lastmod =
      typeof data.date === "string"
        ? parseDateOnly(data.date, entry.name).toISOString().split("T")[0]
        : undefined;
    const slug = stripDatePrefix(entry.name.replace(/\.mdx$/, ""));
    results.push({
      route: `/blog/${slug}`,
      lastmod,
    });
  }

  return results.sort((a, b) => a.route.localeCompare(b.route));
}

function createUrl(route: string, lastmod?: string) {
  const lines = [
    "  <url>",
    `    <loc>${escapeXml(`${siteUrl}${route === "/" ? "/" : route}`)}</loc>`,
  ];
  if (lastmod) {
    lines.push(`    <lastmod>${lastmod}</lastmod>`);
  }
  lines.push("  </url>");
  return lines.join("\n");
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
    createUrl("/"),
    ...otherStaticRoutes.map((route) => createUrl(route)),
    createUrl("/blog"),
    ...blogRoutes.map((entry) => createUrl(entry.route, entry.lastmod)),
    "</urlset>",
    "",
  ].join("\n");

  await fs.writeFile(sitemapPath, sitemap, "utf8");
}

main();
