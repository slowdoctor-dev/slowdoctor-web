---
title: "From Next.js to Rust for No Good Reason"
date: "2026-06-05"
description: "I rewrote a site that already worked — from Next.js into Rust and Leptos — for no good reason except that I wanted to learn how it would feel underneath."
tags: ["meta", "engineering", "rust", "ai", "retrospective"]
axes:
  physician: 1
  engineer: 6
  life: 3
---

A few weeks ago I wrote about building this site in a week with Next.js, Claude Code, and Codex. It worked. It deployed. It did everything I needed: five pages and a blog, static HTML, structured data, fast.

So naturally I rewrote the whole thing from scratch in Rust.

If you visit slowdoctor.dev right now, you will not notice. Same pages. Same dark background with the one gold accent. Same content, same metadata, same SEO. Nothing visible changed. The entire rebuild happened under the floorboards — a new engine bolted under a car that was already driving fine.

Let me be honest about why before I get to how: it didn't need rewriting. I did it to learn Rust and WebAssembly, and because the engineer in me wanted to. That's the whole reason. This post is the retrospective.

## Why rewrite something that works

There's a version of this story where I claim the Next.js site was slow, or bloated, or hard to maintain, and Rust fixed it. That version would be a lie. The old site was fine. A static export is a static export — the browser gets HTML either way, and no visitor can tell whether the bytes were assembled by React or by something I wrote in Rust.

The real reason is the one in the tagline I keep coming back to: choosing the right way over the fast way. The fast way was already done in April. The slow way was a chance to take a perfectly working system apart, understand every layer, and rebuild it with my hands on each piece. Over-engineering a personal site on purpose, as a playground. There is no clinic deadline here, no patient waiting, no business case. That freedom is exactly why a personal site is the right place to do something impractical and learn from it.

So I treated it like a study. And the thing I wanted to study was Rust on the web: Leptos, WebAssembly, and what it actually takes to ship a static site out of a language that wasn't built for this.

## The shape of the before and after

**Before:** Next.js 16, React 19, TypeScript, MDX for posts, Tailwind v4, static export, served by Cloudflare.

**After:** Rust 1.96 with Leptos 0.8, Tailwind v4 (the standalone CLI now, no Node), comrak and syntect doing the Markdown and syntax highlighting, one small WebAssembly island, built in GitHub Actions and deployed to Cloudflare with wrangler.

To the visitor: identical. To me: a completely different machine.

## The decision that shaped everything: no off-the-shelf generator

The obvious path would have been to use Leptos' built-in static-site mode. I read the docs and the issue tracker first — diagnose before you treat — and found that Leptos 0.8's static rendering is still rough, with a couple of open issues that touched exactly what I needed. Worse, the islands model assumes a live server sitting behind the page. I don't want a server. I want flat files on a CDN.

So instead of fighting a tool to do something it wasn't ready for, I wrote my own build program. It walks every page, renders each one to a static HTML string through Leptos' server-rendering, and writes the file out. That meant full control over the parts that matter for a site like this: the `<head>`, the per-page metadata, the JSON-LD structured data that ties the physician and the engineer together for search engines and AI systems.

This is the slow way in miniature. The fast way was a config flag and a prayer. The right way was a few hundred lines of Rust that do exactly what I asked and nothing I didn't.

## Mostly static, one island

Almost the entire site is pre-rendered HTML. No JavaScript, no hydration, nothing for the browser to do but display the page. The one exception is the tag filter on the blog index.

That filter ships as a tiny Leptos component compiled to WebAssembly — an "island" of interactivity in an otherwise static page. But it enhances a fallback rather than replacing it. With JavaScript off, or for a search engine crawler, the blog index still lists every post, fully linked. The island just adds live filtering on top when the browser can run it. Progressive enhancement, the old idea: the page works first, then gets better if it can.

It's one filter on a five-page blog. It did not need to be WebAssembly. But that was the point — I wanted to see the whole pipeline, from a Rust component to a `.wasm` file the browser executes. The island weighs 283 KB raw, about 100 KB gzipped, for code that hides and shows some blog cards. On a real product that ratio would be a red flag. On a playground it's the lesson.

## Everything in Rust, including the boring parts

The first version had a handful of Node scripts doing the unglamorous work: generating the sitemap, building the RSS feed, scaffolding a new post, converting drafts, validating the build before deploy. I ported all of them to Rust too. There's no Node left in the repo.

This was the least exciting and most satisfying part. Nobody will ever see a sitemap generator. But once the build pipeline is one language, the whole thing holds together in your head as a single program instead of a Rust core with a ring of scripts around it.

The payoff was a quiet kind of proof. The Rust build generated `sitemap.xml` and the RSS `feed.xml`, and both came out byte-for-byte identical to what the old Next.js pipeline produced. Not "close enough." Identical. That's the cleanest signal I could have asked for that the rewrite preserved behavior rather than quietly changing it. A faithful port, confirmed by a diff that found nothing.

## The Cloudflare saga

This is the part where the slow way earned its keep, because it's where I got bitten.

The old site let Cloudflare build it. The new one can't. Cloudflare's build image has no Rust toolchain, there's a hard 20-minute cap on a build, and custom builds don't get dependency caching — which for Rust means every build is a cold compile from scratch, right up against the wall. A Rust dependency tree for a five-page blog is, for the record, 275 crates. Compiling that cold, every time, was never going to fit.

The fix was to move the build out. GitHub Actions compiles everything — cached, no time cap — and hands the finished static files to Cloudflare, which just serves them. Cloudflare stopped being a build system and went back to being what I actually wanted: a place to put files.

Then came the bug that I'm still a little proud of catching, because it's the exact failure mode I warn about in medicine.

The first production deploy looked perfect. Green checkmark, site live, pages loading. Done. Except I checked the live HTTP headers — and the security headers were gone. No Content-Security-Policy. The caching rules I'd carefully written, missing. The page worked, but everything I'd specified about how it should be served had silently vanished.

The cause: the deploy tool defaulted to an old version of wrangler (3.90), and that version uploads the `_headers` file as a plain static asset instead of reading it and applying the rules. The file was sitting there in the deploy, doing nothing, looking for all the world like it was working. I pinned wrangler to 4.x, redeployed, and the headers came back.

There's a line I use about surgery: the procedure going smoothly is not the same as the outcome being right. You don't trust that it went well — you confirm it. "It deployed green" is not "it's actually correct." A green pipeline told me the site was fine. The site was not fine. The only reason I know is that I went and looked at production instead of trusting the checkmark.

One clarification, because the phrase gets muddy: "Rust on Cloudflare" can mean two different things. It can mean running Rust as the server — Cloudflare Workers executing your code on each request. Or it can mean shipping a Rust-compiled WebAssembly file that the visitor's browser runs. This site is the second kind. There's no Rust running on Cloudflare's side at all; Cloudflare just serves a static `.wasm` file like any other asset, and the browser does the work.

## How it was built

The first post was about building this site with Claude Code and Codex. This is a continuation of that thread, and the workflow was the same in spirit: I directed, the tool executed, and I owned every decision.

But the structure was more deliberate this time, because the stakes of getting the architecture wrong were higher. I ran it as seven phases, committed and verified one at a time:

1. **Research** — read the entire existing codebase, then studied how mature Leptos' static rendering actually was and how Cloudflare wanted to receive a static deploy. No code until I understood the terrain.
2. **Scaffold** — the Rust workspace, four crates, and a check that server-rendering produced sane HTML at all.
3. **Shared layer** — data, types, metadata, structured-data schema, and the Markdown pipeline.
4. **Pages** — the page components and the layout shell.
5. **Island** — the blog tag filter compiled to WebAssembly.
6. **Asset pipeline** — the generators (sitemap, RSS) and the build tooling.
7. **Deploy** — CI, the Cloudflare wiring, and tearing out the old Node and Next.js.

Each phase was its own commit, verified before the next one started — about ten commits in all, plus the wrangler fix that came after the first real deploy taught me a lesson. Research first, then build outward from a solid center. That's just diagnosis before treatment, applied to software.

## The numbers, for the curious

Roughly 2,860 lines of Rust across four crates: the site itself is 1,878 lines, the build program 284, the WebAssembly island 159, and the tooling 536. The dependency tree, as mentioned, is 275 crates for a blog you could count the pages of on one hand. The island is 283 KB raw and about 100 KB gzipped; the CSS is 21 KB. A full production build and deploy runs about two minutes forty seconds cold, one to three minutes warm.

None of these numbers are good in the sense a product engineer would mean. 275 crates and a 100 KB WebAssembly file to filter blog posts is, objectively, absurd. I know. That's the joke, and it's also the syllabus.

## What it actually bought me

Not speed. The site isn't faster — static was already as fast as static gets. Not maintainability in any honest sense; a five-page blog in Next.js was already trivial to maintain, and Rust didn't make it more so.

What it bought was control and understanding. I now know what's happening at every layer, because I built every layer. When the header bug appeared, I knew exactly where to look. When Leptos' static mode wasn't ready, I knew enough to route around it instead of waiting. That kind of knowledge doesn't come from shipping fast. It comes from doing the slow, unnecessary, hands-on version of something you could have finished in an afternoon.

The clinic side of my life runs on schedules and efficiency, and it should — people are waiting. This site is the place where I get to do the opposite: take the long road on purpose, because the long road is where you learn the terrain. Nobody asked for this rewrite. That's precisely what made it worth doing.

The site is live. The headers are correct — I checked. And I understand it now in a way I didn't before.

We'll see what I take apart next.
