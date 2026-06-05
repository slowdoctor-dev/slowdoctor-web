// ---------------------------------------------------------------------------
// Home — was src/app/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::person_schema;
use crate::types::BlogPostSummary;
use leptos::prelude::*;

pub fn home(posts: &[BlogPostSummary]) -> RenderedPage {
    let recent: Vec<_> = posts
        .iter()
        .take(3)
        .map(|post| {
            let href = format!("/blog/{}", post.slug);
            view! {
                <a
                    href=href
                    class="flex items-baseline justify-between gap-4 rounded-2xl border border-border bg-card px-5 py-4 transition-colors hover:border-accent/30"
                >
                    <span class="text-foreground font-medium truncate">{post.title.clone()}</span>
                    <span class="text-sm text-muted whitespace-nowrap shrink-0">
                        {post.formatted_date.clone()}
                    </span>
                </a>
            }
        })
        .collect();
    let has_recent = !recent.is_empty();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-8 pb-10">
                <div class="game-wrap">
                    // DOM contract: crates/game mounts into this canvas.
                    <canvas
                        id="game-canvas"
                        width="900"
                        height="260"
                        role="img"
                        aria-label="Mini-game"
                    ></canvas>
                </div>
            </section>

            <section class="pb-16 sm:pb-20">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">{AUTHOR_NAME}</h1>
                <p class="mt-4 text-lg text-accent font-medium">
                    "Choosing the right way over the fast way."
                </p>
                <p class="mt-4 text-lg text-muted leading-relaxed">{DESCRIPTION_BRIEF}</p>
                <p class="mt-6 text-foreground/90 leading-relaxed">
                    "I practice both surgery and non-surgical medicine \u{2014} so the recommendation is always what you actually need. Outside the clinic, I explore CS and AI, looking for where they can solve real problems."
                </p>
            </section>

            <section class="pb-16 grid grid-cols-1 sm:grid-cols-2 gap-4">
                <a
                    href="/physician"
                    class="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
                >
                    <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">
                        "As a Physician"
                    </h2>
                    <p class="mt-2 text-sm text-muted">"Slow-aging, scars, and natural eyes."</p>
                    <span class="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
                        "Learn more \u{2192}"
                    </span>
                </a>
                <a
                    href="/engineer"
                    class="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
                >
                    <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">
                        "As an Engineer"
                    </h2>
                    <p class="mt-2 text-sm text-muted">
                        "AI, automation, and real-world problem solving."
                    </p>
                    <span class="mt-4 inline-block text-sm text-accent opacity-0 group-hover:opacity-100 transition-opacity">
                        "Learn more \u{2192}"
                    </span>
                </a>
            </section>

            {has_recent.then(|| view! {
                <section class="pb-24">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-sm font-medium text-accent uppercase tracking-wider">"Latest"</h2>
                        <a href="/blog" class="text-sm text-muted hover:text-foreground transition-colors">
                            "All posts \u{2192}"
                        </a>
                    </div>
                    <div class="space-y-3">{recent}</div>
                </section>
            })}
            <script type="module" src="/_assets/game-init.js"></script>
        </div>
    };

    let meta = build_page_meta(
        SITE_TITLE,
        DESCRIPTION_FULL,
        "/",
        OgType::Website,
        true,
        vec![person_schema()],
    );
    render("/", meta, inner)
}
