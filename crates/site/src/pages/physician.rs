// ---------------------------------------------------------------------------
// Physician — was src/app/physician/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::{card, section_heading};
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::{breadcrumb_schema, person_schema, practice_schema};
use leptos::prelude::*;

pub fn physician() -> RenderedPage {
    let d = doctor();
    let clinical_focus = [
        ("Slow-aging", "Knowing when a device is enough, when injectables are right, and when surgery is the honest answer. Calibrated for long-term results over quick fixes. I plan treatment as an ongoing relationship, not a one-off session."),
        ("Scars", "Every scar has its own timeline and its own answer. Lasers, devices, injectables, or surgery \u{2014} I match the tool to the stage. Covering the full lifecycle from fresh wounds to mature scars."),
        ("Natural Eyes", "Specializing in nonincisional blepharoplasty at a level few surgeons attempt. Minimal, precise, and designed to look like nothing was done at all. Results over speed, every time."),
    ];
    let focus_cards: Vec<_> = clinical_focus
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
    let treatment_areas: Vec<_> = d
        .knows_about
        .iter()
        .map(|area| view! {
            <span class="text-sm text-muted border border-border rounded-full px-3 py-1">{*area}</span>
        })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Physician"</h1>
                <p class="mt-4 text-lg text-muted leading-relaxed max-w-xl">
                    "The right treatment at the right time."
                </p>
            </section>
            <section class="pb-16">
                {section_heading("Clinical Philosophy", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"I practice both surgery and non-surgical medicine in depth. That means the recommendation is always based on what a patient actually needs \u{2014} not limited by what I happen to offer."</p>
                    <p>"Better outcomes take more time. I\u{2019}d rather see someone regularly over years, adjusting as they change, than chase a single dramatic result. If a treatment is not needed, I say so."</p>
                </div>
            </section>
            <section class="pb-16">
                {section_heading("Clinical Focus", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="space-y-3">{focus_cards}</div>
            </section>
            <section class="pb-16">
                {section_heading("Treatment Areas", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="flex flex-wrap gap-2">{treatment_areas}</div>
            </section>
            <section class="pb-24">
                {section_heading("Practice", "text-sm font-medium text-accent uppercase tracking-wider mb-6")}
                <div class="rounded-2xl border border-border bg-card p-6">
                    <h3 class="text-lg font-semibold">{PRACTICE_NAME}</h3>
                    <p class="mt-1 text-sm text-muted">{PRACTICE_LOCATION}</p>
                    <p class="mt-1 text-sm text-muted">{PRACTICE_PHONE}</p>
                    <a href=PRACTICE_URL target="_blank" rel="noopener noreferrer" class="mt-4 inline-block text-sm text-accent hover:underline">
                        "Visit LEAD Plastic Surgery Clinic \u{2192}"
                    </a>
                </div>
            </section>
        </div>
    };

    let meta = build_page_meta(
        "Physician",
        "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and natural blepharoplasty. Calibrated for long-term results, not quick fixes.",
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
