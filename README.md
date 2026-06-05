# slowdoctor-web

Personal site for Dr. Joonho Lim at `slowdoctor.dev`.

Built with **Rust + [Leptos](https://leptos.dev)**, rendered to a fully static site
at build time and served by Cloudflare Workers static assets. Styling is Tailwind CSS
v4; blog posts are Markdown (rendered with `comrak` + `syntect`). The only client-side
interactivity — the blog tag filter — ships as a small Leptos→WASM island.

## Structure

- `crates/site/` — shared library: data, shared types, page + layout components,
  metadata/JSON-LD builders, Markdown loader
- `crates/build-site/` — the static site generator (renders every route to `dist/`,
  hashes CSS, copies `public/`, generates sitemap + feed)
- `crates/island-blog-filter/` — Leptos CSR/WASM island for the blog tag filter
- `crates/tools/` — `new_post`, `convert`, `validate` build/authoring tools
- `src/content/blog/` — Markdown blog posts
- `src/content/incoming/` — drop zone for Markdown drafts (see content pipeline)
- `public/` — static assets (og image, profile image, fonts, robots, `_headers`,
  `_redirects`) copied verbatim into `dist/`
- `globals.css` — Tailwind entry + theme + `.prose`/font `@font-face` rules

## Commands

```bash
make build      # full static build into dist/
make serve      # build + serve dist/ on http://localhost:8080
make validate   # post-build SEO validation
make new-post TITLE="My Post Title"   # scaffold a new blog post
make convert                          # convert incoming drafts -> blog posts
make convert FILE=draft.md            # convert a single draft
```

See [DEPLOY.md](DEPLOY.md) for the toolchain setup (Rust, wasm-bindgen, Tailwind CLI)
and the Cloudflare/GitHub Actions deployment.

## Content Pipeline

### From external drafts

1. Place confirmed Markdown drafts into `src/content/incoming/`
2. Run `make convert` to transform them into blog posts
3. Add `tags` and `axes` to the frontmatter
4. Branch, commit, PR, merge — auto-deploys to production

The convert tool handles:
- Filename convention `YYYY-MM-DD_CHANNEL_english-kebab-slug.md` → `YYYY-MM-DD-english-kebab-slug.md`
- Date extraction from filename prefix
- Title extraction from the first H1 heading
- Auto-generated description from content
- Existing frontmatter preservation (title, description, tags, axes, image)
- Source file removal after successful conversion

### Manual creation

Blog posts live in `src/content/blog/*.md` and support this frontmatter:

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

The build produces a `dist/` directory served via Cloudflare Workers + static assets.
Builds + deploys run in **GitHub Actions** (`.github/workflows/deploy.yml`) on push to
`main`. See [DEPLOY.md](DEPLOY.md).

### Configuration files

- `public/_headers` — cache control and security headers (CSP includes
  `'wasm-unsafe-eval'` for the WASM island)
- `public/_redirects` — redirect rules (currently none needed)
- `wrangler.toml` — `[assets] directory = "./dist"`

## Notes

- `sitemap.xml` and `feed.xml` are generated into `dist/` during the build.
- Fonts (Inter, Plus Jakarta Sans) are vendored in `public/fonts/` — no network
  needed at build time.
- `AGENTS.md` is authoritative for repo-specific agent instructions.
