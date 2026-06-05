// ---------------------------------------------------------------------------
// 404 — was src/app/not-found.tsx
// ---------------------------------------------------------------------------

use super::{render, RenderedPage};
use crate::meta::default_meta;
use leptos::prelude::*;

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
