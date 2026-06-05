// ---------------------------------------------------------------------------
// Links — was src/app/links/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::section_heading;
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::breadcrumb_schema;
use leptos::prelude::*;

pub fn links() -> RenderedPage {
    let medical: Vec<_> = medical_links()
        .into_iter()
        .map(|link| view! {
            <a
                href=link.url
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center justify-between px-5 py-3.5 hover:bg-accent-muted transition-colors first:rounded-t-2xl last:rounded-b-2xl"
            >
                <span class="text-foreground font-medium">{link.label}</span>
                <span class="text-sm text-muted flex items-center gap-2">
                    {link.detail}
                    <span>"\u{2197}"</span>
                </span>
            </a>
        })
        .collect();

    let social: Vec<_> = social_links()
        .into_iter()
        .map(|link| view! {
            <a
                href=link.url
                target="_blank"
                rel="noopener noreferrer"
                class="flex items-center justify-between px-5 py-3.5 hover:bg-accent-muted transition-colors first:rounded-t-2xl last:rounded-b-2xl"
            >
                <span class="text-foreground font-medium">{link.label}</span>
                <span class="text-sm text-muted flex items-center gap-2">
                    {link.handle}
                    <span>"\u{2197}"</span>
                </span>
            </a>
        })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Links"</h1>
                <p class="mt-4 max-w-xl text-lg leading-relaxed text-muted">
                    "Official profiles, social channels, and practice links for Joonho Lim."
                </p>
            </section>
            <div class="pb-24 space-y-10">
                <section>
                    {section_heading("Practice", "text-sm font-medium text-accent uppercase tracking-wider mb-4")}
                    <div class="rounded-2xl border border-border bg-card">
                        <a
                            href=PRACTICE_URL
                            target="_blank"
                            rel="noopener noreferrer"
                            class="flex items-center justify-between px-5 py-4 hover:bg-accent-muted transition-colors rounded-2xl"
                        >
                            <div>
                                <p class="text-foreground font-medium">{PRACTICE_NAME}</p>
                                <p class="text-sm text-muted">{PRACTICE_LOCATION}</p>
                            </div>
                            <span class="text-muted shrink-0 ml-4">"\u{2197}"</span>
                        </a>
                    </div>
                </section>
                <section>
                    {section_heading("Medical", "text-sm font-medium text-accent uppercase tracking-wider mb-4")}
                    <div class="rounded-2xl border border-border bg-card divide-y divide-border">{medical}</div>
                </section>
                <section>
                    {section_heading("Code", "text-sm font-medium text-accent uppercase tracking-wider mb-4")}
                    <div class="rounded-2xl border border-border bg-card">
                        <a
                            href=GITHUB_URL
                            target="_blank"
                            rel="noopener noreferrer"
                            class="flex items-center justify-between px-5 py-3.5 hover:bg-accent-muted transition-colors rounded-2xl"
                        >
                            <span class="text-foreground font-medium">"GitHub"</span>
                            <span class="text-sm text-muted flex items-center gap-2">
                                "@slowdoctor-dev"
                                <span>"\u{2197}"</span>
                            </span>
                        </a>
                    </div>
                </section>
                <section>
                    {section_heading("Social (Korean)", "text-sm font-medium text-accent uppercase tracking-wider mb-4")}
                    <div class="rounded-2xl border border-border bg-card divide-y divide-border">{social}</div>
                </section>
            </div>
        </div>
    };

    let meta = build_page_meta(
        "Links",
        "Official profiles and social links for Joonho Lim \u{2014} ORCID, Google Scholar, ResearchGate, YouTube, Instagram, and LEAD Plastic Surgery Clinic.",
        "/links",
        OgType::Website,
        false,
        vec![breadcrumb_schema(&[("Home", "/"), ("Links", "/links")])],
    );
    render("/links", meta, inner)
}
