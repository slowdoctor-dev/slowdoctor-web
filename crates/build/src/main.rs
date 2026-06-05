use site::markdown::{load_posts, FullPost};
use site::meta::render_head;
use site::pages::{self, RenderedPage};
use std::fs;
use std::path::Path;

const BLOG_DIR: &str = "src/content/blog";
const DIST: &str = "dist";
/// Stylesheet path injected into every page (Phase 5 will hash this).
const CSS_HREF: &str = "/_assets/app.css";

/// Global `<head>` fragment shared by every page (charset, viewport, icons,
/// fonts, RSS alternate, stylesheet).
fn global_head() -> String {
    format!(
        concat!(
            "<meta charset=\"utf-8\"/>",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"/>",
            "<link rel=\"icon\" href=\"/favicon.ico\"/>",
            "<link rel=\"preload\" href=\"/fonts/inter-latin.woff2\" as=\"font\" type=\"font/woff2\" crossorigin=\"anonymous\"/>",
            "<link rel=\"preload\" href=\"/fonts/plus-jakarta-sans-latin.woff2\" as=\"font\" type=\"font/woff2\" crossorigin=\"anonymous\"/>",
            "<link rel=\"alternate\" type=\"application/rss+xml\" title=\"Blog\" href=\"/feed.xml\"/>",
            "<link rel=\"stylesheet\" href=\"{css}\"/>",
        ),
        css = CSS_HREF,
    )
}

/// Wrap a rendered page body + head metadata in the full HTML document.
fn document(page: &RenderedPage) -> String {
    format!(
        "<!DOCTYPE html><html lang=\"en\" class=\"h-full antialiased\"><head>{global}{head}</head><body class=\"min-h-full flex flex-col\">{body}</body></html>",
        global = global_head(),
        head = render_head(&page.meta),
        body = page.body,
    )
}

fn write_page(rel_path: &str, page: &RenderedPage) {
    let path = Path::new(DIST).join(rel_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&path, document(page)).expect("write page");
}

fn main() {
    let posts: Vec<FullPost> = load_posts(Path::new(BLOG_DIR)).expect("load posts");
    let summaries: Vec<_> = posts.iter().map(|p| p.summary.clone()).collect();

    fs::create_dir_all(DIST).expect("create dist");

    write_page("index.html", &pages::home(&summaries));
    write_page("cv.html", &pages::cv());
    write_page("physician.html", &pages::physician());
    write_page("engineer.html", &pages::engineer());
    write_page("links.html", &pages::links());
    write_page("blog.html", &pages::blog(&summaries));
    write_page("404.html", &pages::not_found());

    for post in &posts {
        write_page(&format!("blog/{}.html", post.summary.slug), &pages::blog_post(post));
    }

    // Entry module for the blog-filter island (loaded by /blog).
    let assets = Path::new(DIST).join("_assets");
    fs::create_dir_all(&assets).expect("create _assets");
    fs::write(
        assets.join("blog-init.js"),
        "import init from \"/_assets/blog-filter.js\";\ninit();\n",
    )
    .expect("write blog-init.js");

    println!(
        "built {} static pages into {DIST}/ ({} blog posts)",
        7 + posts.len(),
        posts.len()
    );
}
