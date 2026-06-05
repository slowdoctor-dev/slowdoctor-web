# Deployment

The site is a fully static build (`dist/`) served by **Cloudflare Workers static
assets** (assets-only — no Worker script). The Rust/Leptos build runs in **GitHub
Actions**, which then deploys to Cloudflare with `wrangler deploy`.

Why GitHub Actions instead of Cloudflare Workers Builds: the Workers Builds image
has no Rust toolchain, a hard 20-minute build cap, and no caching for a custom Rust
build — so every build would be a cold compile near the cap. GitHub Actions has no
such cap and caches `~/.cargo` + `target/`, so incremental builds are ~1–3 minutes.

## One-time cutover steps (manual — only you can do these)

1. **Create a Cloudflare API token**
   - Cloudflare dashboard → My Profile → API Tokens → Create Token
   - Use the **"Edit Cloudflare Workers"** template (or a custom token with
     `Account › Workers Scripts › Edit`).
   - Note your **Account ID** (Workers & Pages → right sidebar).

2. **Add GitHub repository secrets** (repo → Settings → Secrets and variables →
   Actions → New repository secret):
   - `CLOUDFLARE_API_TOKEN` — the token from step 1
   - `CLOUDFLARE_ACCOUNT_ID` — your account ID

3. **Disable the old Cloudflare Workers Builds auto-deploy**
   - Dashboard → the `slowdoctor-web` Worker → Settings → Build →
     disconnect the Git integration (or disable automatic builds).
   - This prevents Cloudflare from also trying to build on push (it would fail —
     no Rust) and double-deploying.

4. **Merge `rewrite/rust-leptos` → `main`.** The `Deploy` workflow
   (`.github/workflows/deploy.yml`) builds and deploys on every push to `main`.

After this, the workflow: builds the WASM bundles (blog filter + game), compiles CSS, renders the static
pages, runs `make validate`, then `wrangler deploy` uploads `dist/` to Cloudflare.

## Local build & dev

Install the toolchain (one-time):

```bash
# Rust + wasm target (rust-toolchain.toml pins versions)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
rustup target add wasm32-unknown-unknown

# wasm-bindgen (MUST match the wasm-bindgen crate version in Cargo.lock)
cargo install wasm-bindgen-cli --version 0.2.122   # or download the release binary

# Tailwind CSS v4 standalone CLI -> on PATH as `tailwindcss`
```

Then:

```bash
make build      # full static build into dist/
make serve      # build + serve dist/ on http://localhost:8080
make validate   # post-build SEO checks
```

## Version pins

- Rust: `rust-toolchain.toml`
- `wasm-bindgen` + Tailwind: `.github/workflows/deploy.yml` (`env:` block).
  When bumping the `wasm-bindgen` crate, bump `WASM_BINDGEN_VERSION` to match.

## Notes

- `wrangler.toml` points `[assets] directory` at `./dist`.
- `dist/_headers` sets caching + a CSP that includes `'wasm-unsafe-eval'` (required
  for the Leptos WASM island).
- A future Cloudflare Worker is additive: add `main = "..."` alongside `[assets]`
  in `wrangler.toml`; static-asset requests still bypass the Worker.
