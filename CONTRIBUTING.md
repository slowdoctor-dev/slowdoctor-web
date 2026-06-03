# Content Editing Guide

This is a static Next.js 16 site. All content lives in source files — there is no database or CMS. Edit the files, rebuild, and deploy.

## Quick Reference

| What to change | File | Notes |
|---|---|---|
| Homepage (hero) | `src/app/page.tsx` | Edit text directly |
| Physician page | `src/app/physician/page.tsx` | Clinical focus, philosophy |
| Engineer page | `src/app/engineer/page.tsx` | `interests`, `projects` arrays |
| CV page | `src/app/cv/page.tsx` | Education, career, publications |
| Publications | `src/lib/cv.ts` | `publications` array |
| Links page | `src/app/links/page.tsx` | Auto-populated from shared data |
| Navigation & footer | `src/app/layout.tsx` | `navLinks` array |
| Site constants | `src/lib/config.ts` | `SITE`, `AUTHOR`, `PRACTICE` |
| Social / profile URLs | `src/lib/links.ts` | Single source of truth for all URLs |
| Site metadata & SEO | `src/app/layout.tsx` | `metadata` and `personSchema` objects |
| Theme & colors | `src/app/globals.css` | CSS variables in `:root` |
| Blog posts | `src/content/blog/*.mdx` | See "Adding a Blog Post" below |

## Adding a Blog Post

### From external drafts (recommended pipeline)

1. Copy the confirmed Markdown file to `src/content/incoming/`
2. Run the conversion:

```bash
npm run convert                              # convert all files in incoming/
npm run convert -- "2026-04-11_PT_my-post.md"  # convert a single file
```

3. The script creates `src/content/blog/2026-04-11-my-post.mdx` (date-prefixed) with correct frontmatter
4. Claude Code reads the post and adds `tags` and `axes` to the frontmatter
5. Review and edit as needed

Files follow the naming convention `YYYY-MM-DD_CHANNEL_slug.md`. The date and slug are extracted from the filename, and the output keeps the `YYYY-MM-DD-slug.mdx` convention (the date prefix is stripped to form the URL slug).

### From scratch

Run the helper script:

```bash
npm run new-post -- "My Post Title"
```

This creates `src/content/blog/2026-04-11-my-post-title.mdx` (with today's date) with frontmatter pre-filled. Open it and write your content in Markdown/MDX.

### Manual creation

Create a file in `src/content/blog/` with the `YYYY-MM-DD-slug.mdx` naming convention:

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
| `tags` | No | Array of lowercase tags, e.g. `["meta", "introduction"]` |
| `axes` | No | Physician/Engineer/Life weights (integers 0-10, must sum to 10) |

**File naming:** Blog filenames use a `YYYY-MM-DD-slug.mdx` convention. The date prefix is stripped to produce the URL slug. `2026-04-08-my-post.mdx` → `/blog/my-post`

## Adding a Publication

Edit `src/lib/cv.ts`. Add an entry to the `publications` array:

```typescript
{
  title: "Full paper title",
  authors: "Author A, Author B, Lim J",
  journal: "Journal Name",
  year: 2026,
  volume: "1",                 // optional
  issue: "2",                  // optional
  pages: "10-15",              // optional
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
    cv/page.tsx         # Curriculum vitae
    physician/page.tsx  # Clinical philosophy and focus
    engineer/page.tsx   # Engineering thesis and projects
    links/page.tsx      # External profile links
  components/
    axis-bar.tsx        # Physician/Engineer/Life axis visualization
    blog-list.tsx       # Blog listing with tag filtering
    json-ld.tsx         # JSON-LD structured data component
  content/
    blog/*.mdx          # Blog posts (Markdown + JSX)
    incoming/           # Drop zone for MD drafts
  data/
    doctor.ts           # Doctor profile (single source of truth)
  lib/
    blog.ts             # Blog utilities (read posts, parse MDX)
    breadcrumbs.ts      # BreadcrumbList JSON-LD helper
    config.ts           # Site, author, practice constants
    cv.ts               # Publication data
    links.ts            # Centralized social/profile URLs
    metadata.ts         # Page metadata builder
  mdx-components.tsx    # MDX component overrides (syntax highlighting, links)
scripts/
  date-utils.cts        # Shared date parsing + stripDatePrefix for build scripts
  generate-feed.cts     # Build-time RSS feed generator
  generate-sitemap.cts  # Build-time sitemap generator
  convert-md.cts        # Convert incoming MD drafts to MDX blog posts
  new-post.cts          # Scaffold new blog post
  validate.cts          # Post-build SEO validation
public/
  robots.txt
  og-default.png        # Default social sharing image
  feed.xml              # Generated RSS feed
  sitemap.xml           # Generated sitemap
```
