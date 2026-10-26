// ---------------------------------------------------------------------------
// Physician — was src/app/physician/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::section_heading;
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::{breadcrumb_schema, person_schema, practice_schema};
use leptos::prelude::*;

pub fn physician() -> RenderedPage {
    let program_cards: Vec<_> = clinical_programs()
        .into_iter()
        .map(|program| {
            let chips: Vec<_> = program
                .items
                .iter()
                .map(|item| view! {
                    <span class="text-xs text-muted border border-border rounded-full px-2.5 py-1">{*item}</span>
                })
                .collect();
            view! {
                <div class="rounded-2xl border border-border bg-card p-5">
                    <h3 class="text-base font-semibold text-foreground">
                        {program.name}
                        {program.korean_name.map(|k| view! {
                            <span class="ml-2 text-sm font-normal text-muted">{k}</span>
                        })}
                    </h3>
                    <p class="mt-2 text-sm text-muted leading-relaxed">{program.summary}</p>
                    <div class="mt-3 flex flex-wrap gap-1.5">{chips}</div>
                </div>
            }
        })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Physician"</h1>
                <p class="mt-4 text-lg text-muted leading-relaxed max-w-xl">
                    "Board-certified plastic surgeon. Scars, slow-aging, and eyelid surgery in Gangnam, Seoul."
                </p>
            </section>
            <section class="pb-16">
                {section_heading("Practice", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"I trained in plastic and reconstructive surgery at Seoul National University Hospital and have practiced in Gangnam since 2020. In 2024 I opened LEAD Plastic Surgery Clinic in Dogok-dong."</p>
                    <p>"The clinic offers both surgical and non-surgical treatment, so the plan follows the diagnosis rather than a fixed menu. Most slow-aging and scar patients are followed over months or years with small adjustments. When no treatment is indicated, I say so."</p>
                </div>
            </section>
            <section class="pb-16">
                {section_heading("Clinical Programs", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-3">{program_cards}</div>
            </section>
            <section class="pb-24">
                {section_heading("Clinic", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="rounded-2xl border border-border bg-card p-6">
                    <h3 class="text-lg font-semibold">{PRACTICE_NAME}</h3>
                    <p class="mt-1 text-sm text-muted">{PRACTICE_LOCATION}</p>
                    <p class="mt-1 text-sm text-muted">{PRACTICE_PHONE}</p>
                    <a href=PRACTICE_URL target="_blank" rel="noopener noreferrer" class="mt-4 inline-block text-sm text-accent hover:underline">
                        "leadps.co.kr \u{2197}"
                    </a>
                </div>
            </section>
        </div>
    };

    let meta = build_page_meta(
        "Physician",
        "Board-certified plastic surgeon in Gangnam, Seoul: non-surgical scar treatment, slow-aging care, and eyelid surgery at LEAD Plastic Surgery Clinic.",
        "/physician",
        OgType::Website,
        false,
        vec![
            breadcrumb_schema(&[("Home", "/"), ("Physician", "/physician")]),
            person_schema(),
            practice_schema(),
        ],
    );
    render("/physician", meta, inner)
}
