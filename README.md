# slowdoctor-web

Personal site for Dr. Joonho Lim at `slowdoctor.dev`.

The site is built with Next.js 16, React 19, TypeScript, Tailwind CSS v4, and MDX. It is exported as a fully static site with `output: "export"`.

## Structure

- `src/app/` — route pages and layout
- `src/content/blog/` — MDX blog posts
- `src/content/incoming/` — drop zone for MD drafts from content pipeline
- `src/data/` — doctor profile (single source of truth)
- `src/lib/` — config, blog utilities, SEO helpers
- `scripts/` — build-time generators, conversion, validation

See [CONTRIBUTING.md](CONTRIBUTING.md#project-structure) for the full file-level breakdown.

## Commands

```bash
npm run dev          # local dev server
npm run build        # production static build (generates sitemap + RSS first)
npm run lint         # eslint
npm run convert      # convert incoming MD drafts to MDX blog posts
npm run new-post     # scaffold a new blog post
npm run validate     # post-build SEO validation
```

## Content Pipeline

### From external drafts

1. Place confirmed Markdown drafts into `src/content/incoming/`
2. Run `npm run convert` to transform them into MDX blog posts
3. Claude Code adds `tags` and `axes` to the frontmatter
4. Branch, commit, PR, merge — auto-deploys to production

The convert script handles:
- Filename convention: `YYYY-MM-DD_CHANNEL_english-kebab-slug.md` -> `YYYY-MM-DD-english-kebab-slug.mdx`
- Date extraction from filename prefix
- Title extraction from first H1 heading
- Auto-generated description from content
- Existing frontmatter preservation (title, description, tags, axes, image)
- Source file removal after successful conversion

### Manual creation

Blog posts live in `src/content/blog/*.mdx` and support this frontmatter:

```yaml
---
title: "Post title"
date: "2026-04-07"
description: "Short summary"
image: "/optional-og-image.jpg"
tags: ["tag1", "tag2"]
axes:
  physician: 3
  engineer: 4
  life: 3
---
```

Required: `title`, `date`, `description`. Optional: `image`, `tags`, `axes` (must sum to 10).

## Deployment (Cloudflare Workers)

The site is a fully static export (`output: "export"` in `next.config.ts`). The build produces an `out/` directory served via Cloudflare Workers + Assets.

Push to `main` triggers automatic build and deployment. PRs get preview URLs.

### Configuration files

- `public/_headers` — cache control and security headers
- `public/_redirects` — redirect rules (currently none needed)

## Notes

- `public/sitemap.xml` and `public/feed.xml` are generated during build and should not be edited manually.
- The project uses `next/font/google`, so production builds need network access unless fonts are vendored locally.
- `AGENTS.md` is authoritative for repo-specific agent instructions. Next.js 16 behavior may differ from older releases.
