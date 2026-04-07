# Codex Handoff: SEO & Feed Improvements

## Project Context

Static Next.js 16 personal website at `slowdoctor.dev`. Uses `output: "export"`, MDX blog, sugar-high syntax highlighting, and build-time sitemap generation.

Already completed (do NOT redo):
- BlogPosting JSON-LD on blog posts (`src/app/blog/[slug]/page.tsx`)
- `<lastmod>` in sitemap, `<priority>` removed (`scripts/generate-sitemap.cts`)
- Canonical URLs on all pages
- OG/Twitter metadata with optional image support
- Person/Physician JSON-LD in root layout

## Task 1: RSS feed generator

**Problem:** No RSS feed exists. Google recommends RSS as a supplementary discovery signal, and it enables readers to subscribe.

**Requirements:**
- Create `scripts/generate-feed.cts` that:
  - Reads all `.mdx` files from `src/content/blog/`
  - Parses frontmatter with `gray-matter` (already a dependency)
  - Generates `public/feed.xml` in RSS 2.0 format
  - Channel: title "Joonho Lim - Blog", link "https://slowdoctor.dev/blog", description from blog page metadata
  - Each item: title, link (`https://slowdoctor.dev/blog/{slug}`), description, pubDate (RFC 822 format from frontmatter `date`), guid (same as link, with `isPermaLink="true"`)
  - Sort items by date descending
- Update the `prebuild` script in `package.json` to also run this script (chain with `&&`)
- Add `public/feed.xml` to `.gitignore`
- Add an `<link rel="alternate" type="application/rss+xml">` tag in `src/app/layout.tsx` inside `<head>`:
  ```tsx
  <link rel="alternate" type="application/rss+xml" title="Blog" href="/feed.xml" />
  ```
- Also add the feed URL to `public/robots.txt` as a second line after the Sitemap line (no blank line needed — just keep the format clean)

**Constraints:**
- Do NOT install any RSS library. Use string concatenation like the sitemap generator.
- Use `require("gray-matter")` the same way `generate-sitemap.cts` does.
- No abstractions — single simple script file.

## Task 2: BreadcrumbList JSON-LD on sub-pages

**Problem:** Google generates breadcrumbs from URL structure alone. Explicit BreadcrumbList structured data gives control over how breadcrumbs appear in search results.

**Requirements:**
- Create a shared helper in `src/lib/breadcrumbs.ts`:
  ```typescript
  export function buildBreadcrumbSchema(items: { name: string; href: string }[]) {
    return {
      "@context": "https://schema.org",
      "@type": "BreadcrumbList",
      itemListElement: items.map((item, index) => ({
        "@type": "ListItem",
        position: index + 1,
        name: item.name,
        item: `https://slowdoctor.dev${item.href}`,
      })),
    };
  }
  ```
- Add `<JsonLd data={breadcrumbSchema} />` to these pages:
  - `/physician` — breadcrumbs: Home `/` → Physician `/physician`
  - `/engineer` — breadcrumbs: Home `/` → Engineer `/engineer`
  - `/links` — breadcrumbs: Home `/` → Links `/links`
  - `/blog` — breadcrumbs: Home `/` → Blog `/blog`
  - `/blog/[slug]` — breadcrumbs: Home `/` → Blog `/blog` → {post.title} `/blog/{slug}`
- Import `JsonLd` from `@/components/json-ld` (already exists)

**Constraints:**
- The homepage does NOT need breadcrumbs.
- Keep each page's breadcrumb inline — don't over-abstract.
- The blog post page already has a `<JsonLd>` for BlogPosting. Add the BreadcrumbList as a second `<JsonLd>` (multiple JSON-LD blocks per page is valid and recommended by Google).

## Task 3: Default OG image

**Problem:** Social sharing and Google Discover show no preview image when a blog post has no `image` frontmatter. There is no fallback.

**Requirements:**
- Create a minimal OG image at `public/og-default.png` (1200x630px):
  - Use a simple CLI tool like `node-canvas` or just create a solid dark background (#0a0a0a) with centered text "slowdoctor.dev" in white
  - If generating programmatically is too complex, create a 1200x630 solid #0a0a0a PNG as a placeholder (the user can replace it later)
- In `src/app/layout.tsx`, add to the root metadata's `openGraph`:
  ```typescript
  images: [{ url: "/og-default.png", width: 1200, height: 630, alt: "slowdoctor.dev" }],
  ```
- Also add to `twitter`:
  ```typescript
  images: ["/og-default.png"],
  ```

**Constraints:**
- Do NOT install `@vercel/og`, `satori`, or any heavy image generation library.
- A simple solid-color PNG placeholder is perfectly acceptable. Keep it minimal.
- If you cannot generate a PNG programmatically, create it via a small Node.js script using only built-in modules, or just create a 1x1 placeholder and note that it should be replaced manually.

## Validation

After all three tasks, run:
```bash
npm run build
```
The build must succeed. Verify:
- `public/feed.xml` exists and is valid RSS 2.0
- `public/sitemap.xml` exists with `<lastmod>` (no `<priority>`)
- Blog post HTML contains both `BlogPosting` and `BreadcrumbList` JSON-LD
- Sub-page HTML contains `BreadcrumbList` JSON-LD
- Root layout HTML contains `<link rel="alternate" type="application/rss+xml">`

## Files You'll Touch

- `scripts/generate-feed.cts` (new)
- `src/lib/breadcrumbs.ts` (new)
- `public/og-default.png` (new — placeholder)
- `package.json` (update prebuild)
- `.gitignore` (add public/feed.xml)
- `public/robots.txt` (add feed.xml reference)
- `src/app/layout.tsx` (add RSS link tag, OG image)
- `src/app/physician/page.tsx` (add BreadcrumbList)
- `src/app/engineer/page.tsx` (add BreadcrumbList)
- `src/app/links/page.tsx` (add BreadcrumbList)
- `src/app/blog/page.tsx` (add BreadcrumbList)
- `src/app/blog/[slug]/page.tsx` (add BreadcrumbList)
