// ---------------------------------------------------------------------------
// Engineer — was src/app/engineer/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::{card, section_heading};
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::breadcrumb_schema;
use leptos::prelude::*;

struct Project {
    name: &'static str,
    desc: &'static str,
    stack: &'static str,
    repo: Option<&'static str>,
    live: Option<&'static str>,
}

pub fn engineer() -> RenderedPage {
    let interests = [
        ("Medical Informatics", "Computing for clinical workflows: charting, records, and knowledge systems shaped around how a clinic runs."),
        ("Infodemiology", "How health information spreads online, using search and social data to see what patients ask before they visit."),
        ("Health IT", "Custom tools for a solo clinic (scheduling, inventory, patient flow, signage) where off-the-shelf software does not fit."),
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

    let projects = [
        Project {
            name: "slowdoctor.dev",
            desc: "This site. Static HTML rendered at build time from Leptos components, with two small WebAssembly islands (blog filter, canvas game).",
            stack: "Rust, Leptos 0.8, Tailwind v4, Cloudflare Workers",
            repo: Some(REPO_URL),
            live: None,
        },
        Project {
            name: "slowdoctor-type",
            desc: "Typing trainer for studying English: openly licensed passages (news, paper abstracts, federal prose, vocabulary, code) served by a Rust Worker with a daily feeder.",
            stack: "Rust (workers-rs), D1, TypeScript, Vite",
            repo: Some(TYPING_REPO_URL),
            live: Some(TYPING_URL),
        },
        Project {
            name: "workspace-md",
            desc: "A workspace-topology spec, a sibling to agents.md, defining how a human and an AI agent share a directory so memory and skills compound with use.",
            stack: "Specification",
            repo: Some("https://github.com/slowdoctor-dev/workspace-md"),
            live: None,
        },
        Project {
            name: "seasoned-hand",
            desc: "Open-source autonomous agent platform: deep task execution with learning that persists across sessions. Self-hosted and model-agnostic.",
            stack: "Agents",
            repo: Some("https://github.com/slowdoctor-dev/seasoned-hand"),
            live: None,
        },
        Project {
            name: "ashy-walnut-desk",
            desc: "A digital front desk for regulated-service businesses: identity, interaction, and knowledge with AI augmentation and human approval.",
            stack: "Agents",
            repo: Some("https://github.com/slowdoctor-dev/ashy-walnut-desk"),
            live: None,
        },
        Project {
            name: "lead-signage / lead-inventory / lead-viewer",
            desc: "In-clinic tools at LEAD: digital signage for waiting-room displays, medical supply inventory, and a viewer for patient-facing materials. Private.",
            stack: "Internal",
            repo: None,
            live: None,
        },
    ];
    let project_cards: Vec<_> = projects
        .iter()
        .map(|p| {
            let name = p.name;
            view! {
                <div class="rounded-2xl border border-border bg-card p-4">
                    <div class="flex items-start justify-between gap-4">
                        <h3 class="text-sm font-semibold font-mono">{name}</h3>
                        <div class="flex shrink-0 gap-3 text-xs">
                            {p.live.map(|url| view! {
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    aria-label=format!("{name} live site")
                                    class="text-accent hover:underline"
                                >
                                    "Live \u{2197}"
                                </a>
                            })}
                            {p.repo.map(|repo| view! {
                                <a
                                    href=repo
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    aria-label=format!("{name} source on GitHub")
                                    class="text-accent hover:underline"
                                >
                                    "Source \u{2197}"
                                </a>
                            })}
                        </div>
                    </div>
                    <p class="mt-1 text-sm text-muted">{p.desc}</p>
                    <p class="mt-2 text-xs text-muted/80">{p.stack}</p>
                </div>
            }
        })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Engineer"</h1>
                <p class="mt-4 text-lg text-muted leading-relaxed max-w-xl">
                    "Software for a solo clinic, and the computer science behind it."
                </p>
                <p class="mt-4 text-sm">
                    <a href=GITHUB_URL target="_blank" rel="me noopener noreferrer" class="text-accent hover:underline">
                        "github.com/slowdoctor-dev \u{2197}"
                    </a>
                </p>
            </section>
            <section class="pb-16">
                {section_heading("Approach", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"Most of what I build runs in my own clinic: scheduling, inventory, signage, and patient documents. The problems come from daily practice, which keeps the scope concrete."</p>
                    <p>"I prefer to understand a stack well enough to debug it, which is why this site is hand-built in Rust rather than a template. I am completing a computer science degree at Korea National Open University and start the online M.S. in Computer Science at The University of Texas at Austin in 2027."</p>
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
        "Medical informatics, infodemiology, and health IT. Clinic software for a solo plastic surgery practice, a Rust static site, and a typing trainer on Cloudflare Workers.",
        "/engineer",
        OgType::Website,
        false,
        vec![breadcrumb_schema(&[("Home", "/"), ("Engineer", "/engineer")])],
    );
    render("/engineer", meta, inner)
}
