//! Blog tag-filter island (CSR/WASM).
//!
//! Progressive enhancement: the server renders the full post list as static
//! HTML inside `#blog-list-island` (SEO + no-JS fallback). On load this island
//! reads the serialized posts from `#blog-data`, clears the fallback, and mounts
//! an interactive filtered list in its place. Reuses `axis_bar` from `site`.

use leptos::mount::mount_to;
use leptos::prelude::*;
use site::components::{
    axis_bar, BLOG_CARD_CLASS, TAG_BAR_ACTIVE_CLASS, TAG_BAR_INACTIVE_CLASS, TAG_CHIP_ACTIVE_CLASS,
    TAG_CHIP_INACTIVE_CLASS,
};
use site::types::BlogPostSummary;
use wasm_bindgen::JsCast;

/// A tag toggle button. Clicking selects the tag, or clears it if already active.
fn tag_button(
    tag: String,
    active: ReadSignal<Option<String>>,
    set_active: WriteSignal<Option<String>>,
    active_cls: &'static str,
    inactive_cls: &'static str,
) -> impl IntoView {
    let for_class = tag.clone();
    let for_press = tag.clone();
    let for_click = tag.clone();
    view! {
        <button
            type="button"
            aria-pressed=move || (active.get().as_deref() == Some(for_press.as_str())).to_string()
            on:click=move |_| set_active.update(|a| {
                *a = if a.as_deref() == Some(for_click.as_str()) {
                    None
                } else {
                    Some(for_click.clone())
                };
            })
            class=move || if active.get().as_deref() == Some(for_class.as_str()) {
                active_cls
            } else {
                inactive_cls
            }
        >
            {tag}
        </button>
    }
}

fn card(
    post: &BlogPostSummary,
    active: ReadSignal<Option<String>>,
    set_active: WriteSignal<Option<String>>,
) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    let tags = post.tags.clone();
    let axes = post.axes;
    view! {
        <article class=BLOG_CARD_CLASS>
            <div class="flex flex-col sm:flex-row sm:gap-6">
                <div class="flex-1 min-w-0">
                    <p class="text-sm text-muted">{post.formatted_date.clone()}</p>
                    <h2 class="mt-2 text-xl font-semibold text-foreground">
                        <a href=href class="hover:text-accent">{post.title.clone()}</a>
                    </h2>
                    <p class="mt-2 text-sm leading-relaxed text-muted">{post.description.clone()}</p>
                    {tags.map(|tags| {
                        let chips: Vec<_> = tags
                            .into_iter()
                            .map(|tag| tag_button(tag, active, set_active, TAG_CHIP_ACTIVE_CLASS, TAG_CHIP_INACTIVE_CLASS))
                            .collect();
                        view! { <div class="mt-3 flex flex-wrap gap-1.5">{chips}</div> }
                    })}
                </div>
                {axes.map(|axes| view! {
                    <div class="mt-4 sm:mt-0 sm:w-44 shrink-0">{axis_bar(axes)}</div>
                })}
            </div>
        </article>
    }
}

#[component]
fn BlogList(posts: Vec<BlogPostSummary>) -> impl IntoView {
    let (active, set_active) = signal(None::<String>);

    let mut all_tags: Vec<String> = posts
        .iter()
        .filter_map(|p| p.tags.clone())
        .flatten()
        .collect();
    all_tags.sort();
    all_tags.dedup();

    let posts = StoredValue::new(posts);

    let tag_bar = (!all_tags.is_empty()).then(|| {
        let tag_btns: Vec<_> = all_tags
            .into_iter()
            .map(|tag| tag_button(tag, active, set_active, TAG_BAR_ACTIVE_CLASS, TAG_BAR_INACTIVE_CLASS))
            .collect();
        view! {
            <div class="pb-8 flex flex-wrap gap-1.5">
                <button
                    type="button"
                    aria-pressed=move || active.get().is_none().to_string()
                    on:click=move |_| set_active.set(None)
                    class=move || if active.get().is_none() { TAG_BAR_ACTIVE_CLASS } else { TAG_BAR_INACTIVE_CLASS }
                >
                    "All"
                </button>
                {tag_btns}
            </div>
        }
    });

    let list = move || {
        let a = active.get();
        posts.with_value(|posts| {
            posts
                .iter()
                .filter(|p| match &a {
                    None => true,
                    Some(t) => p.tags.as_ref().is_some_and(|tags| tags.contains(t)),
                })
                .map(|p| card(p, active, set_active))
                .collect::<Vec<_>>()
        })
    };

    view! {
        {tag_bar}
        <div class="space-y-4">{list}</div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    // DOM contract: these IDs are rendered by site::pages::blog.
    let json = document
        .get_element_by_id("blog-data")
        .and_then(|el| el.text_content())
        .unwrap_or_default();
    let posts: Vec<BlogPostSummary> = serde_json::from_str(&json).unwrap_or_default();

    let Some(container) = document.get_element_by_id("blog-list-island") else {
        return;
    };
    // Replace the server-rendered fallback with the interactive list.
    container.set_inner_html("");
    let Ok(container) = container.dyn_into::<web_sys::HtmlElement>() else {
        return;
    };

    let handle = mount_to(container, move || view! { <BlogList posts=posts.clone()/> });
    // Keep the reactive ownership alive for the page lifetime.
    handle.forget();
}
