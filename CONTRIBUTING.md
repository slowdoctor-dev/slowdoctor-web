# Content Editing Guide

This is a static Next.js 16 site. All content lives in source files — there is no database or CMS. Edit the files, rebuild, and deploy.

## Quick Reference

| What to change | File | Notes |
|---|---|---|
| Homepage (hero, CV) | `src/app/page.tsx` | Edit text directly |
| Physician page | `src/app/physician/page.tsx` | `publications` array for papers |
| Engineer page | `src/app/engineer/page.tsx` | `projects`, `techStack` arrays |
| Links page | `src/app/links/page.tsx` | Auto-populated from shared data |
| Navigation & footer | `src/app/layout.tsx` | `navLinks` array |
| Social / profile URLs | `src/lib/links.ts` | Single source of truth for all URLs |
| Site metadata & SEO | `src/app/layout.tsx` | `metadata` and `personSchema` objects |
| Theme & colors | `src/app/globals.css` | CSS variables in `:root` |
| Blog posts | `src/content/blog/*.mdx` | See "Adding a Blog Post" below |

## Adding a Blog Post

Run the helper script:

```bash
npm run new-post -- "My Post Title"
```

This creates `src/content/blog/my-post-title.mdx` with frontmatter pre-filled. Open it and write your content in Markdown/MDX.

### Manual creation

Create a file in `src/content/blog/` with the `.mdx` extension:

```mdx
---
title: "Post Title"
date: "2026-04-08"
description: "One-line summary for SEO and listing page."
image: "/images/post-cover.png"
---

Your content here. Standard Markdown works.

## Subheadings

- Lists, **bold**, *italic*, [links](https://example.com)

Code blocks get syntax highlighting automatically:

\`\`\`ts
const x = 42;
\`\`\`
```

**Frontmatter fields:**

| Field | Required | Description |
|---|---|---|
| `title` | Yes | Post title (shown on page and in metadata) |
| `date` | Yes | ISO date string, e.g. `"2026-04-08"` |
| `description` | Yes | Short summary for SEO and blog listing |
| `image` | No | Path to OG image (falls back to `/og-default.png`) |

**File naming:** The filename becomes the URL slug. `my-post.mdx` → `/blog/my-post`

## Adding a Publication

Edit `src/app/physician/page.tsx`. Add an entry to the `publications` array:

```typescript
{
  title: "Full paper title",
  authors: "Author A, Author B, Lim J",
  journal: "Journal Name",
  year: 2026,
  doi: "10.xxxx/xxxxx",       // optional
  pubmed: "12345678",          // optional
},
```

## Changing Social Links

Edit `src/lib/links.ts`. All consumers (layout footer, homepage channels, links page, JSON-LD sameAs) update automatically.

```typescript
export const socialLinks = [
  { label: "YouTube", url: "https://...", handle: "@slowdoctor" },
  // add or remove entries here
];
```

## Adding a New Page

1. Create `src/app/your-page/page.tsx`
2. Add metadata with `alternates: { canonical: "/your-page" }`
3. Add a BreadcrumbList JSON-LD (see existing pages for pattern)
4. Add the route to `navLinks` in `src/app/layout.tsx` if it should appear in navigation

## Build & Preview

```bash
npm run build                                    # Build (generates sitemap + RSS)
python3 -m http.server 3001 --directory out      # Preview locally
```

## Validation

After building, run the SEO check:

```bash
npm run validate
```

This verifies all pages have proper metadata, canonical URLs, and structured data.

## Project Structure

```
src/
  app/
    layout.tsx          # Root layout (nav, footer, fonts, metadata)
    page.tsx            # Homepage
    not-found.tsx       # Custom 404
    globals.css         # Theme, colors, prose styles
    blog/
      page.tsx          # Blog listing
      [slug]/page.tsx   # Individual blog post
    physician/page.tsx
    engineer/page.tsx
    links/page.tsx
  components/
    json-ld.tsx         # JSON-LD structured data component
  content/
    blog/*.mdx          # Blog posts (Markdown + JSX)
  lib/
    blog.ts             # Blog utilities (read posts, parse MDX)
    links.ts            # Centralized social/profile URLs
    breadcrumbs.ts      # BreadcrumbList JSON-LD helper
  mdx-components.tsx    # MDX component overrides (syntax highlighting, links)
scripts/
  generate-sitemap.cts  # Build-time sitemap generator
  generate-feed.cts     # Build-time RSS feed generator
public/
  robots.txt
  og-default.png        # Default social sharing image
  feed.xml              # Generated RSS feed (gitignored)
  sitemap.xml           # Generated sitemap (gitignored)
```
