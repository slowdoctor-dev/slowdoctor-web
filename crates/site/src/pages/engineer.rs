// ---------------------------------------------------------------------------
// Engineer — was src/app/engineer/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::{card, section_heading};
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::breadcrumb_schema;
use leptos::prelude::*;

pub fn engineer() -> RenderedPage {
    let interests = [
        ("Medical Informatics", "Applying computing to clinical workflows \u{2014} charting, records, and knowledge systems that actually fit how medicine works."),
        ("Infodemiology", "How health information spreads online \u{2014} using search and social data to understand what patients are actually looking for."),
        ("Health IT", "Building custom tools for a solo clinic \u{2014} scheduling, inventory, patient flow, and digital signage \u{2014} instead of relying on off-the-shelf solutions."),
    ];
    let interest_cards: Vec<_> = interests
        .iter()
        .map(|(name, desc)| {
            card(
                "rounded-2xl border border-border bg-card p-5",
                view! {
                        <h3 class="text-base font-semibold text-foreground">{*name}</h3>
                        <p class="mt-2 text-sm text-muted leading-relaxed">{*desc}</p>
                },
            )
        })
        .collect();

    let projects: [(&str, &str, Option<&str>); 7] = [
        ("lead-signage", "Digital signage system for in-clinic displays.", None),
        ("lead-inventory", "Medical supply inventory management.", None),
        ("lead-viewer", "Content viewer for clinic materials.", None),
        ("workspace-md", "A workspace-topology spec \u{2014} a sibling to agents.md defining how a human and an AI agent share a directory so memory and skills compound with use.", Some("https://github.com/slowdoctor-dev/workspace-md")),
        ("seasoned-hand", "An open-source autonomous agent platform \u{2014} deep task execution with learning that persists across sessions. Self-hosted and model-agnostic.", Some("https://github.com/slowdoctor-dev/seasoned-hand")),
        ("ashy-walnut-desk", "A digital front-desk for regulated-service businesses \u{2014} identity, interaction, and knowledge with AI augmentation and human approval.", Some("https://github.com/slowdoctor-dev/ashy-walnut-desk")),
        ("slowdoctor.dev", "This site.", Some(REPO_URL)),
    ];
    let project_cards: Vec<_> = projects
        .iter()
        .map(|(name, desc, repo)| {
            let name = name.to_string();
            view! {
                <div class="flex items-start gap-4 rounded-2xl border border-border bg-card p-4">
                    <div class="flex-1 min-w-0">
                        <h3 class="text-sm font-semibold font-mono">{name.clone()}</h3>
                        <p class="mt-1 text-sm text-muted">{*desc}</p>
                    </div>
                    {repo.map(|repo| view! {
                        <a
                            href=repo
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label=format!("{name} source on GitHub")
                            class="shrink-0 text-xs text-accent hover:underline"
                        >
                            "Source \u{2197}"
                        </a>
                    })}
                </div>
            }
        })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Engineer"</h1>
                <p class="mt-4 text-lg text-muted leading-relaxed max-w-xl">
                    "Medicine gave me the problems. Engineering gives me the tools."
                </p>
                <p class="mt-4 text-sm">
                    <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">
                        "github.com/slowdoctor-dev \u{2197}"
                    </a>
                </p>
            </section>
            <section class="pb-16">
                {section_heading("Thesis", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"I\u{2019}ve always been curious about computing \u{2014} not just using tools, but understanding how they work underneath. Medicine is where I practice, but engineering is how I think."</p>
                    <p>"I\u{2019}d rather build something myself than rely on a tool that almost works. Most of what I make is for my own clinic, but the mindset applies everywhere \u{2014} understand the problem first, then write the solution."</p>
                </div>
            </section>
            <section class="pb-16">
                {section_heading("Interests", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-3">{interest_cards}</div>
            </section>
            <section class="pb-24">
                {section_heading("Projects", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-3">{project_cards}</div>
            </section>
        </div>
    };

    let meta = build_page_meta(
        "Engineer",
        "Medical informatics, infodemiology, and health IT. Building custom scheduling, inventory, and digital signage tools for a solo plastic surgery clinic.",
        "/engineer",
        OgType::Website,
        false,
        vec![breadcrumb_schema(&[("Home", "/"), ("Engineer", "/engineer")])],
    );
    render("/engineer", meta, inner)
}
