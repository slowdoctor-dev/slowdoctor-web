mod assets;
mod generators;

use site::markdown::{load_posts, FullPost};
use site::meta::render_head;
use site::pages::{self, RenderedPage};
use std::fs;
use std::path::Path;

const BLOG_DIR: &str = "src/content/blog";
const PUBLIC: &str = "public";
const DIST: &str = "dist";

/// Global `<head>` fragment shared by every page (charset, viewport, icons,
/// fonts, RSS alternate, stylesheet).
fn global_head(css_href: &str) -> String {
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
        css = css_href,
    )
}

fn document(page: &RenderedPage, css_href: &str) -> String {
    format!(
        "<!DOCTYPE html><html lang=\"en\" class=\"h-full antialiased\"><head>{global}{head}</head><body class=\"min-h-full flex flex-col\">{body}</body></html>",
        global = global_head(css_href),
        head = render_head(&page.meta),
        body = page.body,
    )
}

fn write_page(rel_path: &str, page: &RenderedPage, css_href: &str) {
    let path = Path::new(DIST).join(rel_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(&path, document(page, css_href)).expect("write page");
}

fn main() {
    let dist = Path::new(DIST);
    fs::create_dir_all(dist).expect("create dist");

    // Copy static assets (og image, favicon, robots, images, fonts, _headers,
    // _redirects). Does not touch dist/_assets (island + CSS land there).
    assets::copy_dir(Path::new(PUBLIC), dist);

    // Content-hash the Tailwind CSS (built by the pipeline before this runs).
    let css_href = assets::hash_and_rename_css(dist);

    let posts: Vec<FullPost> = load_posts(Path::new(BLOG_DIR)).expect("load posts");
    let summaries: Vec<_> = posts.iter().map(|p| p.summary.clone()).collect();

    write_page("index.html", &pages::home(&summaries), &css_href);
    write_page("cv.html", &pages::cv(), &css_href);
    write_page("physician.html", &pages::physician(), &css_href);
    write_page("engineer.html", &pages::engineer(), &css_href);
    write_page("links.html", &pages::links(), &css_href);
    write_page("blog.html", &pages::blog(&summaries), &css_href);
    write_page("404.html", &pages::not_found(), &css_href);
    for post in &posts {
        write_page(
            &format!("blog/{}.html", post.summary.slug),
            &pages::blog_post(post),
            &css_href,
        );
    }

    // Island entry module (loaded by /blog).
    let asset_dir = dist.join("_assets");
    fs::create_dir_all(&asset_dir).expect("create _assets");
    fs::write(
        asset_dir.join("blog-init.js"),
        "import init from \"/_assets/blog-filter.js\";\ninit();\n",
    )
    .expect("write blog-init.js");
    fs::write(
        asset_dir.join("game-init.js"),
        "import init from \"/_assets/game.js\";\ninit();\n",
    )
    .expect("write game-init.js");

    generators::write_sitemap(dist, &summaries);
    generators::write_feed(dist, &summaries);

    println!(
        "built {} static pages -> {DIST}/ (css {css_href}, {} blog posts, sitemap + feed)",
        7 + posts.len(),
        posts.len()
    );
}
