# Agent instructions

This is a **Rust + Leptos** static site (not Next.js). It renders to fully static
HTML at build time via a custom builder using Leptos SSR (`RenderHtml::to_html`), and
serves from Cloudflare Workers static assets. There is **no server runtime**.

Before writing code, know the layout (see `README.md` "Structure"):

- `crates/site/` — shared lib. Data lives in `src/data.rs` (profile, clinic programs,
  talks, publications; `CV_UPDATED` is bumped by hand); page views in `src/pages/`
  (one file per route) with the layout in `src/pages/layout.rs`; shared components (used
  by SSR pages *and* the WASM island) in `src/components.rs`; metadata/JSON-LD in
  `src/meta.rs` + `src/schema.rs`; Markdown loading in `src/markdown.rs`; shared date +
  frontmatter helpers in `src/dates.rs` + `src/frontmatter.rs`. Feature-gated: `ssr`
  (build) vs `csr` (island).
- `crates/build-site/` — the SSG. Owns the HTML document shell (`<html>/<head>`) and the
  asset pipeline (CSS hashing, `public/` copy, sitemap + feed).
- `crates/island-blog-filter/` — the blog tag-filter island (Leptos CSR → WASM).
- `crates/game/` — the home-page canvas mini-game (Leptos → WASM).
- `crates/tools/` — `new_post`, `convert`, `validate`.

Conventions:

- Leptos 0.8. Render to string with the `RenderHtml::to_html` trait method (there is
  no `leptos::ssr::render_to_string` in 0.8). Mount the island with `leptos::mount::mount_to`.
- A *dynamic* `inner_html=expr` injects raw inner content; a *literal*
  `inner_html="..."` becomes a plain attribute — use an expression (or external file).
- Build + verify with `make build && make validate` (CI runs the same on PRs plus
  `cargo fmt --check` and unit tests). Match the existing Tailwind
  class usage in `view!` macros (Tailwind v4 scans `crates/**/src` via `@source`).
- Talks: `talks()` in `data.rs` holds title/event/date/summary; the fuller write-up
  (photos in `public/images/talks/`, context) is a normal blog post tagged `talks`,
  linked from the entry's `href`. Drafts start in `src/content/incoming/`.
- Contact address is `CONTACT_EMAIL` in `data.rs` (rendered on /, /cv, /links and in
  the Person JSON-LD).
- Keep SEO parity: every route ships full `<head>` metadata + JSON-LD. Blog pages
  render the full post list statically (no-JS/SEO fallback) before the island enhances it.
- Pin notes: `wasm-bindgen` CLI must match the crate version in `Cargo.lock`; see
  `.github/workflows/deploy.yml` and `DEPLOY.md`.
