//! Scaffold a new blog post. Port of `scripts/new-post.cts`.
//! Usage: cargo run -p tools --bin new_post -- "My Post Title"

// This authoring tool intentionally fails fast on invalid input or filesystem errors.

use site::dates::{has_date_prefix, strip_date_prefix};
use std::path::Path;
use std::process::exit;
use time::OffsetDateTime;

const BLOG_DIR: &str = "src/content/blog";

fn slugify(title: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for c in title.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn today() -> String {
    let d = OffsetDateTime::now_utc().date();
    format!("{:04}-{:02}-{:02}", d.year(), d.month() as u8, d.day())
}

/// True if a blog file already resolves to this slug (bare or date-prefixed).
fn slug_collision(slug: &str) -> Option<String> {
    let bare = format!("{slug}.md");
    for entry in std::fs::read_dir(BLOG_DIR)
        .expect("read blog dir")
        .flatten()
    {
        let name = entry.file_name().to_string_lossy().to_string();
        if name == bare {
            return Some(name);
        }
        if let Some(stem) = name.strip_suffix(".md") {
            if has_date_prefix(stem) && strip_date_prefix(stem) == slug {
                return Some(name);
            }
        }
    }
    None
}

fn main() {
    let title: String = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let title = title.trim();
    if title.is_empty() {
        eprintln!("Usage: cargo run -p tools --bin new_post -- \"My Post Title\"");
        exit(1);
    }

    let slug = slugify(title);
    if slug.is_empty() {
        eprintln!("Could not derive an ASCII slug from the title. Use at least one Latin letter or number.");
        exit(1);
    }

    let date = today();
    let file_name = format!("{date}-{slug}.md");
    let path = Path::new(BLOG_DIR).join(&file_name);

    if path.exists() {
        eprintln!("File already exists: {}", path.display());
        exit(1);
    }
    if let Some(existing) = slug_collision(&slug) {
        eprintln!("Post slug \"{slug}\" already exists as {BLOG_DIR}/{existing}");
        exit(1);
    }

    let content = format!(
        "---\ntitle: {title:?}\ndate: \"{date}\"\ndescription: \"TODO: Write a short description for this post.\"\n---\n\n"
    );
    std::fs::write(&path, content).expect("write post");
    println!("Created: {BLOG_DIR}/{file_name}");
}
