use leptos::prelude::*;
use std::fs;
use std::path::Path;

fn hello() -> impl IntoView {
    view! { <h1>"Hello from Leptos SSR"</h1> }
}

fn render_page(inner: impl IntoView) -> String {
    // RenderHtml::to_html (tachys) renders a view to a static HTML string.
    let body = inner.to_html();
    format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"></head><body>{body}</body></html>")
}

fn main() {
    let dist = Path::new("dist");
    fs::create_dir_all(dist).expect("create dist");
    let html = render_page(hello());
    fs::write(dist.join("index.html"), &html).expect("write index.html");
    println!("wrote dist/index.html ({} bytes)", html.len());
}
