---
title: "From Next.js to Rust"
date: "2026-06-05"
description: "I rewrote a site that already worked — from Next.js to Rust and Leptos — to learn the stack underneath."
tags: ["meta", "engineering", "rust", "ai", "retrospective"]
axes:
  physician: 1
  engineer: 6
  life: 3
---

A few weeks ago I wrote about building this site in a week with Next.js, Claude Code, and Codex. It worked, it deployed, and it did everything I needed: five pages and a blog, static HTML, structured data.

Then I rewrote the whole thing from scratch in Rust.

If you visit slowdoctor.dev now, you won't notice. Same pages, same design, same content, same metadata and SEO. The rewrite is entirely under the hood. To be clear about why: it didn't need rewriting. I did it to learn Rust and WebAssembly. This post is the retrospective.

## Why rewrite something that works

The old Next.js site was fine. A static export is a static export — the browser gets HTML either way, and no visitor can tell whether it was assembled by React or by Rust.

So this was a study, and the subject was Rust on the web: Leptos, WebAssembly, and what it takes to ship a static site from a language that wasn't built for it.

## Before and after

**Before:** Next.js 16, React 19, TypeScript, MDX, Tailwind v4, static export, served by Cloudflare.

**After:** Rust 1.96 with Leptos 0.8, Tailwind v4 (the standalone CLI, no Node), comrak and syntect for Markdown and syntax highlighting, one small WebAssembly island, built in GitHub Actions and deployed to Cloudflare with wrangler.

To the visitor: identical. Underneath: a different machine.

## No off-the-shelf generator

The obvious path was Leptos' built-in static-site mode. I read the docs and issue tracker first and found that Leptos 0.8's static rendering is still rough, with open issues touching exactly what I needed. The islands model also assumes a live server, and I wanted flat files on a CDN.

So I wrote my own build program. It walks every page, renders each one to a static HTML string through Leptos' server-rendering, and writes the file out. That gave full control over what matters for this site: the `<head>`, the per-page metadata, and the JSON-LD structured data that ties the physician and the engineer together for search engines and AI systems.

## Mostly static, one island

Almost the entire site is pre-rendered HTML — no JavaScript, no hydration. The one exception is the tag filter on the blog index.

It ships as a small Leptos component compiled to WebAssembly: an island of interactivity in a static page. It enhances a fallback rather than replacing it — with JavaScript off, or for a crawler, the blog index still lists every post, fully linked, and the island adds live filtering when the browser can run it.

It's one filter on a five-page blog; it didn't need to be WebAssembly. But I wanted to see the whole pipeline, from a Rust component to a `.wasm` file the browser runs.

## Everything in Rust

The first version had Node scripts for the unglamorous work: the sitemap, the RSS feed, scaffolding a post, converting drafts, validating the build. I ported all of them to Rust. There's no Node left in the repo.

Once the build pipeline is one language, the whole thing reads as a single program rather than a Rust core with a ring of scripts around it.

The Rust build generated `sitemap.xml` and `feed.xml` byte-for-byte identical to the old Next.js output — a clean signal that the rewrite preserved behavior rather than changing it.

## Cloudflare

The old site let Cloudflare build it. The new one can't: Cloudflare's build image has no Rust toolchain, there's a hard 20-minute build cap, and custom builds don't get dependency caching, so every build is a cold compile from scratch. The dependency tree is 275 crates for a five-page blog; compiling that cold every time was never going to fit.

So the build moved to GitHub Actions — cached, no time cap — which hands the finished static files to Cloudflare to serve.

Then a bug. The first production deploy looked fine: green check, site live, pages loading. But the live HTTP headers were missing the Content-Security-Policy and the cache rules I had set. The cause: the deploy tool defaulted to an old wrangler (3.90) that uploads the `_headers` file as a static asset instead of applying it. I pinned wrangler to 4.x, redeployed, and the headers came back. A green pipeline is not the same as a correct deploy; the only reason I caught it was checking production directly.

One clarification: "Rust on Cloudflare" can mean two things — running Rust as the server (Cloudflare Workers), or shipping a Rust-compiled WebAssembly file the browser runs. This site is the second. There is no Rust running on Cloudflare's side; it serves a static `.wasm` file like any other asset.

## How it was built

Built with Claude Code; I directed and reviewed every decision. The structure was seven phases, committed and verified one at a time:

1. Research — read the existing codebase, then check how mature Leptos' static rendering was and how Cloudflare expects a static deploy.
2. Scaffold — the Rust workspace, four crates, and a check that server-rendering produced sane HTML.
3. Shared layer — data, types, metadata, structured-data schema, and the Markdown pipeline.
4. Pages — the page components and the layout shell.
5. Island — the blog tag filter compiled to WebAssembly.
6. Asset pipeline — the generators (sitemap, RSS) and the build tooling.
7. Deploy — CI, the Cloudflare wiring, and removing the old Node and Next.js.

About ten commits, plus the wrangler fix after the first deploy.

## The numbers

Roughly 2,860 lines of Rust across four crates: the site is 1,878 lines, the build program 284, the WebAssembly island 159, and the tooling 536. The dependency tree is 275 crates. The island is 283 KB raw, about 100 KB gzipped; the CSS is 21 KB. A full production build and deploy runs about two minutes forty seconds cold, one to three minutes warm.

275 crates and a 100 KB WebAssembly file to filter blog posts is not efficient by any product measure. For a learning project, that was not the point.

## What it bought

Not speed — static was already as fast as static gets. Not maintainability in any real sense; a five-page blog in Next.js was already trivial to maintain.

What it bought was understanding. I now know what happens at every layer, because I built every layer. When the header bug appeared, I knew where to look. When Leptos' static mode was not ready, I knew enough to route around it.

The site is live, the headers are correct, and I understand it in a way I did not before.
