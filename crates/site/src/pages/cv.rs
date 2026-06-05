// ---------------------------------------------------------------------------
// CV — was src/app/cv/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::section_heading;
use crate::data::*;
use crate::meta::{build_page_meta, OgType};
use crate::schema::{breadcrumb_schema, scholarly_article_schema};
use leptos::prelude::*;

fn cv_entry(title: &str, subtitle: Option<&str>, date: &str) -> impl IntoView {
    let title = title.to_string();
    let subtitle = subtitle.map(|s| s.to_string());
    let date = date.to_string();
    view! {
        <div class="flex justify-between gap-4">
            <div>
                <p class="text-foreground font-medium">{title}</p>
                {subtitle.map(|s| view! { <p class="text-muted">{s}</p> })}
            </div>
            <p class="text-muted whitespace-nowrap shrink-0">{date}</p>
        </div>
    }
}

fn cv_section_heading(text: &'static str) -> impl IntoView {
    section_heading(
        text,
        "text-xs font-semibold text-accent uppercase tracking-widest mb-4",
    )
}

pub fn cv() -> RenderedPage {
    let d = doctor();
    let publication_schemas: Vec<_> = publications()
        .iter()
        .map(scholarly_article_schema)
        .collect();

    let pub_items: Vec<_> = publications()
        .into_iter()
        .map(|p| {
            let citation = format!("{}. {}. ", p.authors, p.title);
            view! {
                <li class="text-foreground/90 leading-relaxed pl-1">
                    {citation}
                    <span class="italic">{p.journal}</span>
                    ". "
                    {p.year.to_string()}
                    {p.volume.map(|v| format!(";{v}"))}
                    {p.issue.map(|i| format!("({i})"))}
                    {p.pages.map(|pg| format!(":{pg}"))}
                    "."
                    {(p.doi.is_some() || p.pubmed.is_some()).then(|| {
                        let doi_link = p.doi.map(|doi| view! {
                            <a href=format!("https://doi.org/{doi}") target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">"DOI"</a>
                        });
                        let sep = (p.doi.is_some() && p.pubmed.is_some()).then(|| view! {
                            <span class="text-muted mx-1">"\u{00b7}"</span>
                        });
                        let pubmed_link = p.pubmed.map(|pm| view! {
                            <a href=format!("https://pubmed.ncbi.nlm.nih.gov/{pm}") target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">"PubMed"</a>
                        });
                        view! { <span class="ml-1">{doi_link}{sep}{pubmed_link}</span> }
                    })}
                </li>
            }
        })
        .collect();

    let memberships: Vec<_> = d
        .member_of
        .iter()
        .map(|society| view! { <li>{*society}</li> })
        .collect();

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-8 sm:pt-32 sm:pb-10 text-center">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">{AUTHOR_CREDENTIALED_NAME}</h1>
                <p class="mt-1 text-muted text-sm">{AUTHOR_KOREAN}</p>
                <p class="mt-3 text-sm text-muted">
                    {format!("{PRACTICE_NAME} \u{00b7} {PRACTICE_LOCATION}")}
                </p>
                <p class="mt-1 text-sm text-muted">
                    <a href=PRACTICE_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">
                        "LEAD Plastic Surgery Clinic website"
                    </a>
                </p>
            </section>

            <hr class="border-border mb-10"/>

            <section class="pb-10">
                {cv_section_heading("Education")}
                <div class="space-y-3 text-sm">
                    {cv_entry("Korea National Open University", Some("B.S. in Statistics and Data Science / B.S. in Computer Science (Double Major, in progress)"), "2025 \u{2013} Present")}
                    {cv_entry("Seoul National University College of Medicine", Some("Doctor of Medicine (M.D.)"), "2006 \u{2013} 2012")}
                    {cv_entry("Seoul Science High School", Some("Early graduation, Valedictorian"), "2004 \u{2013} 2006")}
                </div>
            </section>

            <section class="pb-10">
                {cv_section_heading("Postgraduate Training")}
                <div class="space-y-3 text-sm">
                    {cv_entry("Residency, Department of Plastic and Reconstructive Surgery", Some("Seoul National University Hospital"), "2016 \u{2013} 2020")}
                    {cv_entry("Internship", Some("Seoul National University Hospital"), "2012 \u{2013} 2013")}
                </div>
            </section>

            <section class="pb-10">
                {cv_section_heading("Military Service")}
                <div class="text-sm">
                    {cv_entry("Military Medical Officer", Some("Republic of Korea Army, Daejeon"), "2013 \u{2013} 2016")}
                </div>
            </section>

            <section class="pb-10">
                {cv_section_heading("Licensure & Board Certification")}
                <div class="space-y-3 text-sm">
                    {cv_entry("Board Certification in Plastic Surgery", Some("Ministry of Health and Welfare, Republic of Korea"), "2020")}
                    {cv_entry("Physician License", Some("Ministry of Health and Welfare, Republic of Korea"), "2012")}
                </div>
            </section>

            <section class="pb-10">
                {cv_section_heading("Professional Experience")}
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between gap-4">
                        <div>
                            <p class="text-foreground font-medium">"Founder & Director"</p>
                            <p class="text-muted">
                                <a href=PRACTICE_URL target="_blank" rel="noopener noreferrer" class="text-accent hover:underline">
                                    "LEAD Plastic Surgery Clinic website"
                                </a>
                                ", Gangnam, Seoul"
                            </p>
                        </div>
                        <p class="text-muted whitespace-nowrap shrink-0">"2024 \u{2013} Present"</p>
                    </div>
                    {cv_entry("Plastic Surgeon", Some("Wonderful Plastic Surgery Clinic, Gangnam, Seoul"), "2022 \u{2013} 2024")}
                    {cv_entry("Plastic Surgeon", Some("POP Plastic Surgery Clinic, Gangnam, Seoul"), "2021 \u{2013} 2022")}
                    {cv_entry("Plastic Surgeon", Some("THE Plastic Surgery Clinic, Gangnam, Seoul"), "2020 \u{2013} 2021")}
                </div>
            </section>

            <section class="pb-10">
                {cv_section_heading("Professional Memberships")}
                <ul class="space-y-1 text-sm text-foreground">{memberships}</ul>
            </section>

            <section class="pb-24">
                {cv_section_heading("Peer-Reviewed Publications")}
                <ol class="list-decimal list-outside ml-4 space-y-4 text-sm">{pub_items}</ol>
            </section>
        </div>
    };

    let mut json_ld = vec![breadcrumb_schema(&[("Home", "/"), ("CV", "/cv")])];
    json_ld.extend(publication_schemas);

    let meta = build_page_meta(
        "CV",
        "Curriculum vitae of Joonho Lim, M.D. \u{2014} education at Seoul National University, plastic surgery residency, board certification, and eight peer-reviewed publications.",
        "/cv",
        OgType::Website,
        false,
        json_ld,
    );
    render("/cv", meta, inner)
}
