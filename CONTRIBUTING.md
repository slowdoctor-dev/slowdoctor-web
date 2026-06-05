# Content Editing Guide

This is a static **Rust + Leptos** site. All content lives in source files — there is
no database or CMS. Edit the files, rebuild (`make build`), and deploy. See
[DEPLOY.md](DEPLOY.md) for toolchain setup.

## Quick Reference

| What to change | File | Notes |
|---|---|---|
| Homepage (hero) | `crates/site/src/pages/home.rs` | Edit text directly |
| Physician page | `crates/site/src/pages/physician.rs` | Clinical focus, philosophy |
| Engineer page | `crates/site/src/pages/engineer.rs` | `interests`, `projects` arrays |
| CV page | `crates/site/src/pages/cv.rs` | Education, career, publications |
| Publications | `crates/site/src/data.rs` (`publications`) | Publication list |
| Links page | `crates/site/src/pages/links.rs` | Auto-populated from shared data |
| Navigation & footer | `crates/site/src/pages/layout.rs` (`nav_links`, `footer`) | |
| Mini-game | `crates/game/src/main.rs` | Canvas endless-runner (WASM) |
| Site constants | `crates/site/src/data.rs` | `SITE_*`, `AUTHOR_*`, `PRACTICE_*` |
| Social / profile URLs | `crates/site/src/data.rs` (`social_links`, etc.) | Single source of truth |
| JSON-LD / SEO | `crates/site/src/{schema,meta}.rs` | Person/Practice/Breadcrumb schemas |
| Theme & colors | `globals.css` | CSS variables in `:root` |
| Blog posts | `src/content/blog/*.md` | See "Adding a Blog Post" below |

## Adding a Blog Post

### From external drafts (recommended pipeline)

1. Copy the confirmed Markdown file to `src/content/incoming/`
2. Run the conversion:

```bash
make convert                       # convert all files in incoming/
make convert FILE=2026-04-11_PT_my-post.md   # convert a single file
```

3. The tool creates `src/content/blog/2026-04-11-my-post.md` (date-prefixed) with
   correct frontmatter
4. Add `tags` and `axes` to the frontmatter
5. Review and edit as needed

Files follow the naming convention `YYYY-MM-DD_CHANNEL_slug.md`. The date and slug are
extracted from the filename, and the output keeps the `YYYY-MM-DD-slug.md` convention
(the date prefix is stripped to form the URL slug).

### From scratch

```bash
make new-post TITLE="My Post Title"
```

This creates `src/content/blog/2026-04-11-my-post-title.md` (with today's date) with
frontmatter pre-filled. Open it and write your content in Markdown.

### Manual creation

Create a file in `src/content/blog/` with the `YYYY-MM-DD-slug.md` naming convention:

```md
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

**File naming:** Blog filenames use a `YYYY-MM-DD-slug.md` convention. The date prefix
is stripped to produce the URL slug. `2026-04-08-my-post.md` → `/blog/my-post`

## Adding a Publication

Edit `crates/site/src/data.rs`. Add an entry to the `publications()` vector:

```rust
Publication {
    title: "Full paper title",
    authors: "Author A, Author B, Lim J",
    journal: "Journal Name",
    year: 2026,
    published_date: Some("2026-01-15"), // optional
    volume: Some("1"),                  // optional
    issue: Some("2"),                   // optional
    pages: Some("10-15"),               // optional
    doi: Some("10.xxxx/xxxxx"),         // optional
    pubmed: Some("12345678"),           // optional
},
```

## Changing Social Links

Edit `crates/site/src/data.rs` (`social_links` / `medical_links`). All consumers
(footer, links page, JSON-LD `sameAs`) update automatically.

## Adding a New Page

1. Add a `pub fn your_page() -> RenderedPage` in `crates/site/src/pages/your_page.rs`
   and declare the module in `crates/site/src/pages/mod.rs` (build metadata with
   `build_page_meta`, include a `breadcrumb_schema`).
2. Add a `write_page("your-page.html", &pages::your_page(), &css_href)` call in
   `crates/build-site/src/main.rs`.
3. Add the route to `nav_links` in `crates/site/src/pages/layout.rs`, and to
   `other_static` in `crates/build-site/src/generators.rs` (sitemap), if it should be listed.

## Build & Preview

```bash
make build      # build into dist/ (CSS, WASM island, pages, sitemap, feed)
make serve      # build + serve dist/ on http://localhost:8080
```

## Validation

```bash
make validate   # verifies metadata, canonical URLs, and structured data
```

## Project Structure

```
crates/
  site/src/
    lib.rs            # module wiring + feature gates (ssr / csr)
    data.rs           # doctor profile, site/author/practice constants, links, CV
    types.rs          # shared serde types (BlogPostSummary, Axes)
    dates.rs          # shared date parse/format helpers
    frontmatter.rs    # shared YAML frontmatter split/parse
    components.rs     # shared views: social icons, axis bar, post card
    pages/            # one file per route (home, blog, cv, ...) + layout.rs
    meta.rs           # <head> metadata builder + JSON-LD escaping
    schema.rs         # Person/Practice/Breadcrumb + per-page JSON-LD
    markdown.rs       # blog loading: frontmatter, validation, comrak+syntect
  build-site/src/
    main.rs           # SSG: HTML shell, writes dist/, island loaders
    assets.rs         # public/ copy, CSS content-hashing
    generators.rs     # sitemap.xml + feed.xml
  island-blog-filter/src/
    main.rs           # Leptos CSR/WASM blog tag-filter island
  game/src/
    main.rs           # Leptos/WASM canvas mini-game (home page)
  tools/src/bin/
    new_post.rs       # scaffold a new post
    convert.rs        # incoming drafts -> blog posts
    validate.rs       # post-build SEO validation
src/content/
  blog/*.md           # blog posts
  incoming/           # drop zone for MD drafts
public/
  fonts/              # vendored Inter + Plus Jakarta Sans woff2
  images/profile.jpg
  og-default.png      # default social sharing image
  favicon.ico
  robots.txt
  _headers            # cache + security headers (copied to dist/)
  _redirects          # redirect rules (copied to dist/)
globals.css           # Tailwind entry + theme + prose/font rules
Makefile              # build / serve / validate / new-post / convert
wrangler.toml         # Cloudflare assets config (directory = ./dist)
```
