//! Shared SSR page layout shell.

use crate::components::social_icon;
use crate::data::*;
use crate::meta::HeadMeta;
use leptos::prelude::*;

pub struct RenderedPage {
    pub meta: HeadMeta,
    pub body: String,
}

pub(super) fn render(path: &str, meta: HeadMeta, inner: impl IntoView) -> RenderedPage {
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
