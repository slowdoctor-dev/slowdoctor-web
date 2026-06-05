//! SSR page views + layout shell. Ported from `src/app/**` route pages and
//! `src/app/layout.tsx`. Each builder returns a `RenderedPage` carrying the
//! head metadata (incl. JSON-LD) and the rendered `<body>` inner HTML.

use crate::components::{axis_bar, post_card, social_icon};
use crate::data::*;
use crate::markdown::FullPost;
use crate::meta::{absolute_url, build_page_meta, default_meta, HeadMeta, OgType};
use crate::schema::{breadcrumb_schema, person_schema, practice_schema};
use crate::types::BlogPostSummary;
use leptos::prelude::*;
use serde_json::json;

pub struct RenderedPage {
    pub meta: HeadMeta,
    pub body: String,
}

fn render(path: &str, meta: HeadMeta, inner: impl IntoView) -> RenderedPage {
    RenderedPage {
        body: shell(path, inner).to_html(),
        meta,
    }
}

// ---------------------------------------------------------------------------
// Layout shell (header / nav / main / footer) — was src/app/layout.tsx
// ---------------------------------------------------------------------------

fn nav_links(current: &str) -> impl IntoView {
    let links = [
        ("/cv", "CV"),
        ("/physician", "Physician"),
        ("/engineer", "Engineer"),
        ("/blog", "Blog"),
        ("/links", "Links"),
    ];
    let items: Vec<_> = links
        .iter()
        .map(|(href, label)| {
            let active = current == *href || current.starts_with(&format!("{href}/"));
            let class = if active {
                "text-sm transition-colors font-medium text-foreground"
            } else {
                "text-sm transition-colors text-muted hover:text-foreground"
            };
            view! {
                <a href=*href aria-current=active.then_some("page") class=class>
                    {*label}
                </a>
            }
        })
        .collect();
    view! { <div class="flex items-center gap-5 sm:gap-6">{items}</div> }
}

fn footer() -> impl IntoView {
    let year = time::OffsetDateTime::now_utc().year();
    let socials: Vec<_> = social_links()
        .into_iter()
        .map(|link| {
            view! {
                <a
                    href=link.url
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label=link.label
                    class="text-muted hover:text-foreground transition-colors"
                >
                    {social_icon(link.label, "w-4 h-4")}
                </a>
            }
        })
        .collect();
    view! {
        <footer aria-label="Site footer" class="border-t border-border">
            <div class="mx-auto max-w-3xl px-6 py-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                <p class="text-sm text-muted">{format!("\u{00a9} {year} {AUTHOR_NAME}")}</p>
                <div class="flex items-center gap-4">
                    {socials}
                    <a
                        href=GITHUB_URL
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="GitHub"
                        class="text-muted hover:text-foreground transition-colors"
                    >
                        {social_icon("GitHub", "w-4 h-4")}
                    </a>
                </div>
            </div>
        </footer>
    }
}

fn shell(current: &str, inner: impl IntoView) -> impl IntoView {
    view! {
        <a href="#main-content" class="skip-link">"Skip to content"</a>
        <header class="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
            <nav
                aria-label="Main"
                class="mx-auto flex max-w-3xl flex-col items-center gap-2 px-6 py-4 sm:flex-row sm:justify-between sm:gap-0"
            >
                <a
                    href="/"
                    class="text-sm font-semibold tracking-tight text-foreground hover:text-accent transition-colors border border-border rounded-md px-2.5 py-1"
                >
                    "slowdoctor.dev"
                </a>
                {nav_links(current)}
            </nav>
        </header>
        <main id="main-content" class="flex-1">
            {inner}
        </main>
        {footer()}
    }
}

// ---------------------------------------------------------------------------
// Home — was src/app/page.tsx
// ---------------------------------------------------------------------------

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
            <section class="pt-24 pb-16 sm:pt-32 sm:pb-20">
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

// ---------------------------------------------------------------------------
// Blog index — was src/app/blog/page.tsx
// ---------------------------------------------------------------------------

const BLOG_DESCRIPTION: &str = "Writing by Joonho Lim on plastic surgery, clinical thinking, engineering, and the slower path. Notes from a surgeon who builds his own tools.";

fn blog_list_static(posts: &[BlogPostSummary]) -> impl IntoView {
    let mut all_tags: Vec<String> = posts
        .iter()
        .filter_map(|p| p.tags.clone())
        .flatten()
        .collect();
    all_tags.sort();
    all_tags.dedup();

    let tag_bar = (!all_tags.is_empty()).then(|| {
        let buttons: Vec<_> = all_tags
            .into_iter()
            .map(|tag| {
                view! {
                    <button
                        type="button"
                        class="text-xs rounded-full px-2.5 py-1 transition-colors text-muted border border-border hover:text-foreground"
                    >
                        {tag}
                    </button>
                }
            })
            .collect();
        view! {
            <div class="pb-8 flex flex-wrap gap-1.5">
                <button
                    type="button"
                    aria-pressed="true"
                    class="text-xs rounded-full px-2.5 py-1 transition-colors bg-accent text-background"
                >
                    "All"
                </button>
                {buttons}
            </div>
        }
    });

    let cards: Vec<_> = posts.iter().map(post_card).collect();

    view! {
        {tag_bar}
        <div class="space-y-4">{cards}</div>
    }
}

pub fn blog(posts: &[BlogPostSummary]) -> RenderedPage {
    let posts_json = serde_json::to_string(posts)
        .unwrap_or_else(|_| "[]".into())
        .replace('<', "\\u003c");

    let collection_schema = json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": "Blog",
        "description": BLOG_DESCRIPTION,
        "url": format!("{SITE_URL}/blog"),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": posts.iter().enumerate().map(|(i, post)| json!({
                "@type": "ListItem",
                "position": i + 1,
                "url": format!("{SITE_URL}/blog/{}", post.slug),
                "name": post.title,
            })).collect::<Vec<_>>(),
        },
    });

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Blog"</h1>
                <p class="mt-4 max-w-xl text-lg leading-relaxed text-muted">
                    "Notes on plastic surgery, clinical thinking, and building with code."
                </p>
            </section>
            <section class="pb-24">
                <div id="blog-list-island">{blog_list_static(posts)}</div>
                <script id="blog-data" type="application/json" inner_html=posts_json></script>
                <script type="module" src="/_assets/blog-init.js"></script>
            </section>
        </div>
    };

    let meta = build_page_meta(
        "Blog",
        BLOG_DESCRIPTION,
        "/blog",
        OgType::Website,
        false,
        vec![
            breadcrumb_schema(&[("Home", "/"), ("Blog", "/blog")]),
            collection_schema,
        ],
    );
    render("/blog", meta, inner)
}

// ---------------------------------------------------------------------------
// Blog post — was src/app/blog/[slug]/page.tsx
// ---------------------------------------------------------------------------

pub fn blog_post(post: &FullPost) -> RenderedPage {
    let s = &post.summary;
    let slug = &s.slug;
    let canonical = absolute_url(&format!("/blog/{slug}"));
    let image_url = absolute_url(s.image.as_deref().unwrap_or(OG_IMAGE));
    let d = doctor();

    let mut article = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": s.title,
        "description": s.description,
        "datePublished": s.date,
        "dateModified": s.date,
        "url": canonical,
        "mainEntityOfPage": { "@type": "WebPage", "@id": canonical },
        "inLanguage": "en",
        "author": {
            "@type": "Person",
            "@id": d.id(),
            "name": AUTHOR_NAME,
            "url": format!("{SITE_URL}/cv"),
            "jobTitle": AUTHOR_JOB_TITLE,
            "worksFor": {
                "@type": "MedicalBusiness",
                "@id": d.works_for.id,
                "name": PRACTICE_NAME,
                "url": PRACTICE_URL,
            },
        },
        "publisher": { "@type": "Person", "@id": d.id(), "name": AUTHOR_NAME },
        "image": image_url,
    });
    if let Some(tags) = &s.tags {
        if !tags.is_empty() {
            article["keywords"] = json!(tags.join(", "));
        }
    }
    let breadcrumb = breadcrumb_schema(&[
        ("Home", "/"),
        ("Blog", "/blog"),
        (s.title.as_str(), &format!("/blog/{slug}")),
    ]);

    let axes = s.axes;
    let tags = s.tags.clone();
    let has_meta_row = axes.is_some() || tags.as_ref().is_some_and(|t| !t.is_empty());

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <a href="/blog" class="text-sm text-accent hover:underline">"\u{2190} Back to blog"</a>
                <h1 class="mt-6 text-3xl font-bold tracking-tight sm:text-4xl">{s.title.clone()}</h1>
                <p class="mt-4 text-sm text-muted">
                    "By "
                    <a href="/cv" class="text-foreground hover:text-accent transition-colors">{AUTHOR_NAME}</a>
                </p>
                <p class="mt-2 text-sm text-muted">{s.formatted_date.clone()}</p>
                {has_meta_row.then(|| {
                    let axis_block = axes.map(|axes| view! {
                        <div class="sm:w-48">{axis_bar(axes)}</div>
                    });
                    let tag_block = tags.clone().filter(|t| !t.is_empty()).map(|tags| {
                        let chips: Vec<_> = tags.into_iter().map(|tag| view! {
                            <span class="text-xs text-muted border border-border rounded-full px-2 py-0.5">
                                {tag}
                            </span>
                        }).collect();
                        view! { <div class="flex flex-wrap gap-1.5">{chips}</div> }
                    });
                    view! {
                        <div class="mt-6 flex flex-col sm:flex-row sm:items-start gap-4">
                            {axis_block}
                            {tag_block}
                        </div>
                    }
                })}
            </section>
            <article class="prose pb-24" inner_html=post.content_html.clone()></article>
        </div>
    };

    let title_tag = SITE_TITLE_TEMPLATE.replace("%s", &s.title);
    let meta = HeadMeta {
        title: title_tag,
        description: s.description.clone(),
        canonical: canonical.clone(),
        og_title: s.title.clone(),
        og_type: OgType::Article,
        og_url: canonical,
        og_image: image_url.clone(),
        og_image_dims: false,
        twitter_title: s.title.clone(),
        twitter_image: image_url,
        json_ld: vec![article, breadcrumb],
    };
    render(&format!("/blog/{slug}"), meta, inner)
}

// ---------------------------------------------------------------------------
// CV — was src/app/cv/page.tsx
// ---------------------------------------------------------------------------

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
    view! {
        <h2 class="text-xs font-semibold text-accent uppercase tracking-widest mb-4">{text}</h2>
    }
}

pub fn cv() -> RenderedPage {
    let d = doctor();

    let publication_schemas: Vec<_> = publications()
        .iter()
        .map(|p| {
            let authors: Vec<_> = p
                .authors
                .split(", ")
                .map(|name| {
                    if name == "Lim J" {
                        json!({ "@type": "Person", "name": name, "@id": d.id() })
                    } else {
                        json!({ "@type": "Person", "name": name })
                    }
                })
                .collect();
            let mut schema = json!({
                "@context": "https://schema.org",
                "@type": "ScholarlyArticle",
                "headline": p.title,
                "author": authors,
                "datePublished": p.published_date.map(|s| s.to_string()).unwrap_or_else(|| p.year.to_string()),
                "isPartOf": { "@type": "Periodical", "name": p.journal },
            });
            if let Some(doi) = p.doi {
                schema["url"] = json!(format!("https://doi.org/{doi}"));
                schema["identifier"] = json!({
                    "@type": "PropertyValue",
                    "propertyID": "DOI",
                    "value": doi,
                });
            }
            schema
        })
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

// ---------------------------------------------------------------------------
// Physician — was src/app/physician/page.tsx
// ---------------------------------------------------------------------------

pub fn physician() -> RenderedPage {
    let d = doctor();
    let clinical_focus = [
        ("Slow-aging", "Knowing when a device is enough, when injectables are right, and when surgery is the honest answer. Calibrated for long-term results over quick fixes. I plan treatment as an ongoing relationship, not a one-off session."),
        ("Scars", "Every scar has its own timeline and its own answer. Lasers, devices, injectables, or surgery \u{2014} I match the tool to the stage. Covering the full lifecycle from fresh wounds to mature scars."),
        ("Natural Eyes", "Specializing in nonincisional blepharoplasty at a level few surgeons attempt. Minimal, precise, and designed to look like nothing was done at all. Results over speed, every time."),
    ];
    let focus_cards: Vec<_> = clinical_focus
        .iter()
        .map(|(name, desc)| view! {
            <div class="rounded-2xl border border-border bg-card p-5">
                <h3 class="text-base font-semibold text-foreground">{*name}</h3>
                <p class="mt-2 text-sm text-muted leading-relaxed">{*desc}</p>
            </div>
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
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Clinical Philosophy"</h2>
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"I practice both surgery and non-surgical medicine in depth. That means the recommendation is always based on what a patient actually needs \u{2014} not limited by what I happen to offer."</p>
                    <p>"Better outcomes take more time. I\u{2019}d rather see someone regularly over years, adjusting as they change, than chase a single dramatic result. If a treatment is not needed, I say so."</p>
                </div>
            </section>
            <section class="pb-16">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Clinical Focus"</h2>
                <div class="space-y-3">{focus_cards}</div>
            </section>
            <section class="pb-16">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Treatment Areas"</h2>
                <div class="flex flex-wrap gap-2">{treatment_areas}</div>
            </section>
            <section class="pb-24">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Practice"</h2>
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

// ---------------------------------------------------------------------------
// Engineer — was src/app/engineer/page.tsx
// ---------------------------------------------------------------------------

pub fn engineer() -> RenderedPage {
    let interests = [
        ("Medical Informatics", "Applying computing to clinical workflows \u{2014} charting, records, and knowledge systems that actually fit how medicine works."),
        ("Infodemiology", "How health information spreads online \u{2014} using search and social data to understand what patients are actually looking for."),
        ("Health IT", "Building custom tools for a solo clinic \u{2014} scheduling, inventory, patient flow, and digital signage \u{2014} instead of relying on off-the-shelf solutions."),
    ];
    let interest_cards: Vec<_> = interests
        .iter()
        .map(|(name, desc)| view! {
            <div class="rounded-2xl border border-border bg-card p-5">
                <h3 class="text-base font-semibold text-foreground">{*name}</h3>
                <p class="mt-2 text-sm text-muted leading-relaxed">{*desc}</p>
            </div>
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
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Thesis"</h2>
                <div class="space-y-5 text-foreground/90 leading-relaxed">
                    <p>"I\u{2019}ve always been curious about computing \u{2014} not just using tools, but understanding how they work underneath. Medicine is where I practice, but engineering is how I think."</p>
                    <p>"I\u{2019}d rather build something myself than rely on a tool that almost works. Most of what I make is for my own clinic, but the mindset applies everywhere \u{2014} understand the problem first, then write the solution."</p>
                </div>
            </section>
            <section class="pb-16">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Interests"</h2>
                <div class="space-y-3">{interest_cards}</div>
            </section>
            <section class="pb-24">
                <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-6">"Projects"</h2>
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

// ---------------------------------------------------------------------------
// Links — was src/app/links/page.tsx
// ---------------------------------------------------------------------------

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
                    <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-4">"Practice"</h2>
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
                    <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-4">"Medical"</h2>
                    <div class="rounded-2xl border border-border bg-card divide-y divide-border">{medical}</div>
                </section>
                <section>
                    <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-4">"Code"</h2>
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
                    <h2 class="text-sm font-medium text-accent uppercase tracking-wider mb-4">"Social (Korean)"</h2>
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

// ---------------------------------------------------------------------------
// 404 — was src/app/not-found.tsx
// ---------------------------------------------------------------------------

pub fn not_found() -> RenderedPage {
    let inner = view! {
        <div class="mx-auto max-w-3xl px-6 pt-24 pb-24 sm:pt-32">
            <h1 class="text-4xl font-bold tracking-tight sm:text-5xl">"404"</h1>
            <p class="mt-4 text-lg text-muted">"Page not found."</p>
            <a href="/" class="mt-8 inline-block text-sm text-accent hover:underline">"\u{2190} Back to home"</a>
        </div>
    };
    render("/404", default_meta("/404"), inner)
}
