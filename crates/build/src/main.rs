use std::path::Path;

// Temporary Phase-2 sanity check; replaced by the full builder in Phase 3.
fn main() {
    let posts = site::markdown::load_posts(Path::new("src/content/blog"))
        .expect("load posts");
    println!("loaded {} posts", posts.len());
    for p in &posts {
        println!(
            "- {} | {} | tags={:?} axes={:?}",
            p.summary.slug, p.summary.formatted_date, p.summary.tags, p.summary.axes
        );
    }
    let first = &posts[0];
    println!("\n--- content_html of '{}' (first 400 chars) ---", first.summary.slug);
    println!("{}", &first.content_html.chars().take(400).collect::<String>());

    // Exercise meta + schema builders.
    let meta = site::meta::build_page_meta(
        "CV",
        "Curriculum vitae",
        "/cv",
        site::meta::OgType::Website,
        false,
        vec![site::schema::person_schema()],
    );
    println!("\n--- head ---\n{}", site::meta::render_head(&meta));
}
