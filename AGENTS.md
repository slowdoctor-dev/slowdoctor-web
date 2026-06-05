# Agent instructions

This is a **Rust + Leptos** static site (not Next.js). It renders to fully static
HTML at build time via a custom builder using Leptos SSR (`RenderHtml::to_html`), and
serves from Cloudflare Workers static assets. There is **no server runtime**.

Before writing code, know the layout (see `README.md` "Structure"):

- `crates/site/` — shared lib. Data lives in `src/data.rs`; page + layout views in
  `src/pages.rs`; shared components (used by SSR pages *and* the WASM island) in
  `src/components.rs`; metadata/JSON-LD in `src/meta.rs` + `src/schema.rs`; Markdown
  loading in `src/markdown.rs`. Feature-gated: `ssr` (build) vs `csr` (island).
- `crates/build-site/` — the SSG. Owns the HTML document shell (`<html>/<head>`) and the
  asset pipeline (CSS hashing, `public/` copy, sitemap + feed).
- `crates/island-blog-filter/` — the only client-side code (Leptos CSR → WASM).
- `crates/tools/` — `new_post`, `convert`, `validate`.

Conventions:

- Leptos 0.8. Render to string with the `RenderHtml::to_html` trait method (there is
  no `leptos::ssr::render_to_string` in 0.8). Mount the island with `leptos::mount::mount_to`.
- A *dynamic* `inner_html=expr` injects raw inner content; a *literal*
  `inner_html="..."` becomes a plain attribute — use an expression (or external file).
- Build + verify with `make build && make validate`. Match the existing Tailwind
  class usage in `view!` macros (Tailwind v4 scans `crates/**/src` via `@source`).
- Keep SEO parity: every route ships full `<head>` metadata + JSON-LD. Blog pages
  render the full post list statically (no-JS/SEO fallback) before the island enhances it.
- Pin notes: `wasm-bindgen` CLI must match the crate version in `Cargo.lock`; see
  `.github/workflows/deploy.yml` and `DEPLOY.md`.
