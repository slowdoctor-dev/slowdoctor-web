# slowdoctor-web

Personal site for Dr. Joonho Lim at `slowdoctor.dev`.

The site is built with Next.js 16, React 19, TypeScript, Tailwind CSS v4, and MDX. It is exported as a fully static site with `output: "export"`.

## Structure

- `src/app/` route pages and layout
- `src/content/blog/` MDX blog posts
- `src/lib/blog.ts` blog loading and frontmatter parsing
- `src/mdx-components.tsx` MDX rendering overrides
- `scripts/generate-sitemap.cts` build-time sitemap generation

## Commands

```bash
npm run dev
npm run build
npm run lint
```

`npm run build` runs the sitemap generator first, then performs the production static build.

## Content Authoring

Blog posts live in `src/content/blog/*.mdx` and support this frontmatter:

```yaml
---
title: "Post title"
date: "2026-04-07"
description: "Short summary"
image: "/optional-og-image.jpg"
---
```

`image` is optional and is used for per-post Open Graph and Twitter metadata.

## Notes

- `public/sitemap.xml` is generated during build and should not be edited manually.
- The project uses `next/font/google`, so production builds need network access unless fonts are vendored locally.
- `AGENTS.md` is authoritative for repo-specific agent instructions. Next.js 16 behavior may differ from older releases.
