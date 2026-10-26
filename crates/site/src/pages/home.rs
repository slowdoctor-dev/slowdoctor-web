// ---------------------------------------------------------------------------
// Home — was src/app/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::person_schema;
use crate::types::BlogPostSummary;
use leptos::prelude::*;

/// Short "what I'm doing now" list. Keep it to facts that are true today;
/// update when they change (and bump `CV_UPDATED` if the CV changed too).
const NOW: [&str; 4] = [
    "Founder and director, LEAD Plastic Surgery Clinic, Gangnam (since 2024)",
    "B.S. Computer Science and B.S. Statistics & Data Science, Korea National Open University (in progress)",
    "Admitted to the online M.S. in Computer Science at The University of Texas at Austin; starting 2027",
    "Building: this site (Rust/Leptos) and a typing trainer at type.slowdoctor.dev",
];

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

    let now_items: Vec<_> = NOW
        .iter()
        .map(|item| view! { <li class="text-sm text-foreground/90 leading-relaxed">{*item}</li> })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">{AUTHOR_NAME}</h1>
                <p class="mt-3 text-base text-muted">{AUTHOR_KOREAN}" \u{00b7} "{AUTHOR_JOB_TITLE}</p>
                <p class="mt-5 text-lg text-foreground/90 leading-relaxed">{DESCRIPTION_BRIEF}</p>
                <p class="mt-3 text-sm text-muted leading-relaxed">
                    "Trained at Seoul National University Hospital. Practicing in Gangnam, Seoul at "
                    <a href=PRACTICE_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">{PRACTICE_NAME}</a>
                    "."
                </p>
                <p class="mt-3 text-sm text-muted">
                    "Contact: "
                    <a href=format!("mailto:{CONTACT_EMAIL}") class="text-accent hover:underline">{CONTACT_EMAIL}</a>
                </p>
            </section>

            <section class="pb-12 grid grid-cols-1 sm:grid-cols-2 gap-4">
                <a
                    href="/physician"
                    class="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
                >
                    <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">
                        "Physician"
                    </h2>
                    <p class="mt-2 text-sm text-muted">"Scars, slow-aging, and eyelid surgery."</p>
                </a>
                <a
                    href="/engineer"
                    class="group rounded-2xl border border-border bg-card p-6 transition-all hover:border-accent/30 hover:bg-accent-muted"
                >
                    <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">
                        "Engineer"
                    </h2>
                    <p class="mt-2 text-sm text-muted">
                        "Clinic software, medical informatics, and CS coursework."
                    </p>
                </a>
            </section>

            <section class="pb-12">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-4">"Now"</h2>
                <ul class="list-disc list-outside ml-4 space-y-1.5">{now_items}</ul>
            </section>

            {has_recent.then(|| view! {
                <section class="pb-12">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-sm font-medium text-accent uppercase tracking-wider">"Latest"</h2>
                        <a href="/blog" class="text-sm text-muted hover:text-foreground transition-colors">
                            "All posts \u{2192}"
                        </a>
                    </div>
                    <div class="space-y-3">{recent}</div>
                </section>
            })}

            <section class="pb-24">
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
                <p class="mt-2 text-xs text-muted">
                    "A small canvas game written in Rust and compiled to WebAssembly. "
                    <a href=REPO_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">"Source \u{2197}"</a>
                </p>
            </section>
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
