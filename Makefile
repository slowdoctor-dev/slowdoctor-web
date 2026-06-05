SHELL := /bin/bash
DIST := dist
ISLAND_WASM := target/wasm32-unknown-unknown/release/island-blog-filter.wasm
GAME_WASM := target/wasm32-unknown-unknown/release/game.wasm

# Requires on PATH: cargo, wasm-bindgen (matching wasm-bindgen crate version),
# tailwindcss (standalone v4). See DEPLOY.md for toolchain setup.

.PHONY: build clean island game css pages validate new-post convert serve dev

# Full static build into dist/ (mirrors the old `next build` + export).
build: clean island game css pages
	@echo "Build complete -> $(DIST)/"

clean:
	rm -rf $(DIST)
	mkdir -p $(DIST)/_assets

# Compile + bindgen the blog-filter WASM island.
island:
	cargo build -p island-blog-filter --target wasm32-unknown-unknown --release
	wasm-bindgen --target web --no-typescript \
		--out-dir $(DIST)/_assets --out-name blog-filter $(ISLAND_WASM)

# Compile + bindgen the mini-game WASM.
game:
	cargo build -p game --target wasm32-unknown-unknown --release
	wasm-bindgen --target web --no-typescript \
		--out-dir $(DIST)/_assets --out-name game $(GAME_WASM)

# Tailwind v4 -> dist/_assets/app.css (build-site content-hashes it).
css:
	tailwindcss -i globals.css -o $(DIST)/_assets/app.css --minify

# Render pages, copy public/, hash CSS, generate sitemap + feed.
pages:
	cargo run -p build-site

# Post-build SEO validation.
validate:
	cargo run -p tools --bin validate

# Scaffold a post:  make new-post TITLE="My Post Title"
new-post:
	cargo run -p tools --bin new_post -- "$(TITLE)"

# Convert incoming drafts:  make convert   (or)  make convert FILE=draft.md
convert:
	cargo run -p tools --bin convert -- $(FILE)

# Build then serve dist/ locally on :8080.
serve: build
	cd $(DIST) && python3 -m http.server 8080

dev: serve
