# Codex Handoff: slowdoctor-web Improvements

## Project Context

This is a statically exported Next.js 16 personal website for a plastic surgeon & engineer (slowdoctor.dev). Tech stack: Next.js 16, TypeScript, Tailwind CSS v4, MDX for blog posts. The site uses `output: "export"` — no server runtime.

Key files:
- `next.config.ts` — MDX + static export config
- `src/lib/blog.ts` — blog pipeline (getAllPosts, getPostBySlug, getPostFrontmatter)
- `src/mdx-components.tsx` — MDX component overrides (currently empty)
- `src/content/blog/` — MDX blog posts
- `public/sitemap.xml` — manually maintained sitemap

## Task 1: Auto-generate sitemap.xml at build time

**Problem:** `public/sitemap.xml` is manually maintained. As blog posts grow, this will drift out of sync.

**Requirements:**
- Create a build script (`scripts/generate-sitemap.ts` or similar) that:
  - Reads all page routes from `src/app/` (excluding layout, error, not-found, loading files)
  - Reads all blog post slugs from `src/content/blog/*.mdx`
  - Generates `public/sitemap.xml` with all URLs under `https://slowdoctor.dev/`
  - Static pages get priority 0.8, blog listing 0.7, individual posts 0.5, homepage 1.0
- Add a `"prebuild"` script in `package.json` that runs this before `next build`
- Remove the current manually written `public/sitemap.xml` (it will be auto-generated)
- Add `public/sitemap.xml` to `.gitignore` since it's a build artifact

**Constraints:**
- Do NOT install `next-sitemap` or any external package. Use Node.js fs/path only.
- Keep it simple — a single script file, no abstractions.
- Use the same `blogDirectory` pattern as `src/lib/blog.ts` to find posts.

## Task 2: Blog post OG images

**Problem:** Blog posts have no unique `og:image`. Social sharing shows no preview image.

**Requirements:**
- Add an optional `image` field to blog frontmatter in `src/lib/blog.ts` (type: `string | undefined`)
- In `src/app/blog/[slug]/page.tsx` `generateMetadata`, if a post has an `image` field, include it as `openGraph.images`
- For now, do NOT generate images automatically — just wire up the frontmatter field so images can be manually added per post
- Update the `BlogFrontmatter` interface to include `image?: string`
- Do NOT modify the existing hello-world.mdx post

**Constraints:**
- Keep the `image` field optional — posts without it should work fine (no fallback needed yet)

## Task 3: MDX component overrides with syntax highlighting

**Problem:** `src/mdx-components.tsx` returns an empty object. Blog content relies on `.prose` CSS classes in `globals.css`, which works for basic formatting. But code blocks have no syntax highlighting.

**Requirements:**
- Install `sugar-high` (lightweight syntax highlighter, ~1KB, no heavy deps like shiki/prism)
- In `src/mdx-components.tsx`, override the `code` component:
  - If the code element has a `className` starting with `language-`, apply syntax highlighting via `sugar-high`
  - Inline code (no language class) should remain unstyled (the `.prose code` CSS handles it)
- Override the `pre` component to pass through cleanly (no double-wrapping)
- Override the `a` component to add `target="_blank" rel="noopener noreferrer"` for external links (URLs starting with `http`)

**Constraints:**
- Do NOT install shiki, prism, rehype-highlight, or any heavy highlighting library
- `sugar-high` is the only acceptable dependency for this task
- Keep the file simple — no abstractions, just direct component definitions
- Verify it works by adding a code block to `src/content/blog/hello-world.mdx` (a simple TypeScript snippet)

## Validation

After all three tasks, run:
```bash
npm run build
```
The build must succeed with no errors. All pages should be in the static output.

## Files You'll Touch

- `scripts/generate-sitemap.ts` (new)
- `package.json` (add prebuild script, add sugar-high dep)
- `.gitignore` (add public/sitemap.xml)
- `public/sitemap.xml` (delete — now auto-generated)
- `src/lib/blog.ts` (add optional image field)
- `src/app/blog/[slug]/page.tsx` (wire og:image in generateMetadata)
- `src/mdx-components.tsx` (add component overrides)
- `src/content/blog/hello-world.mdx` (add test code block)
