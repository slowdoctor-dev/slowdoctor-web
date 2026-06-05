// ---------------------------------------------------------------------------
// Blog index — was src/app/blog/page.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::components::{axis_bar, post_card, TAG_BAR_ACTIVE_CLASS, TAG_BAR_INACTIVE_CLASS};
use crate::data::*;
use crate::markdown::FullPost;
use crate::meta::{absolute_url, article_meta, build_page_meta, OgType};
use crate::schema::{blog_collection_schema, blog_posting_schema, breadcrumb_schema};
use crate::types::BlogPostSummary;
use leptos::prelude::*;

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
                        class=TAG_BAR_INACTIVE_CLASS
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
                    class=TAG_BAR_ACTIVE_CLASS
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

    let collection_schema = blog_collection_schema("Blog", BLOG_DESCRIPTION, posts);

    let inner = view! {
        <div class="mx-auto max-w-3xl px-6">
            <section class="pt-24 pb-12 sm:pt-32 sm:pb-16">
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">"Blog"</h1>
                <p class="mt-4 max-w-xl text-lg leading-relaxed text-muted">
                    "Notes on plastic surgery, clinical thinking, and building with code."
                </p>
            </section>
            <section class="pb-24">
                // DOM contract: crates/island-blog-filter reads #blog-data and mounts here.
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
    let article = blog_posting_schema(s, &canonical, &image_url);
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

    let meta = article_meta(
        &s.title,
        &s.description,
        &format!("/blog/{slug}"),
        s.image.as_deref(),
        vec![article, breadcrumb],
    );
    render(&format!("/blog/{slug}"), meta, inner)
}
