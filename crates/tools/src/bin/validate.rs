//! Post-build SEO validation. Port of `scripts/validate.cts` (retargeted to dist/).
//! Usage: cargo run -p tools --bin validate   (run after a build)

use site::markdown::load_posts;
use std::path::Path;
use std::process::exit;

const DIST: &str = "dist";
const BLOG_DIR: &str = "src/content/blog";

struct Checker {
    errors: u32,
}

impl Checker {
    fn check(&mut self, cond: bool, msg: &str) {
        if cond {
            println!("  OK: {msg}");
        } else {
            eprintln!("  FAIL: {msg}");
            self.errors += 1;
        }
    }
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn main() {
    let mut c = Checker { errors: 0 };
    let dist = Path::new(DIST);

    println!("\n[Build output]");
    let dist_exists = dist.is_dir();
    c.check(dist_exists, "dist/ directory exists");
    if !dist_exists {
        eprintln!("\nRun the build first.\n");
        exit(1);
    }

    println!("\n[Generated files]");
    let sitemap_path = dist.join("sitemap.xml");
    let feed_path = dist.join("feed.xml");
    c.check(sitemap_path.exists(), "sitemap.xml generated");
    c.check(feed_path.exists(), "feed.xml generated");

    println!("\n[Sitemap]");
    let sitemap = read(&sitemap_path).unwrap_or_default();
    c.check(sitemap.contains("<lastmod>"), "sitemap has <lastmod>");
    c.check(!sitemap.contains("<priority>"), "sitemap has no <priority> (Google ignores it)");
    c.check(!sitemap.contains("Invalid Date"), "sitemap has no invalid date values");

    println!("\n[HTML pages]");
    let pages = [
        ("index.html", "Homepage"),
        ("physician.html", "Physician"),
        ("engineer.html", "Engineer"),
        ("cv.html", "CV"),
        ("links.html", "Links"),
        ("blog.html", "Blog"),
    ];
    for (file, name) in pages {
        let path = dist.join(file);
        match read(&path) {
            None => {
                eprintln!("  FAIL: {name} — {file} not found");
                c.errors += 1;
            }
            Some(html) => {
                println!("\n  {name} ({file})");
                c.check(html.contains("<title>"), &format!("{name} has <title>"));
                c.check(html.contains("rel=\"canonical\""), &format!("{name} has canonical URL"));
                c.check(html.contains("og:title"), &format!("{name} has og:title"));
                if file != "index.html" {
                    c.check(html.contains("BreadcrumbList"), &format!("{name} has BreadcrumbList JSON-LD"));
                }
            }
        }
    }

    println!("\n[Blog posts]");
    let posts = load_posts(Path::new(BLOG_DIR)).expect("load posts");
    for post in &posts {
        let slug = &post.summary.slug;
        let path = dist.join("blog").join(format!("{slug}.html"));
        match read(&path) {
            None => {
                eprintln!("  FAIL: /blog/{slug} — HTML not found");
                c.errors += 1;
            }
            Some(html) => {
                println!("\n  /blog/{slug}");
                c.check(html.contains("BlogPosting"), "has BlogPosting JSON-LD");
                c.check(html.contains("BreadcrumbList"), "has BreadcrumbList JSON-LD");
                c.check(html.contains("rel=\"canonical\""), "has canonical URL");
                c.check(html.contains(&post.summary.formatted_date), "shows formatted publication date");
                c.check(!html.contains("Invalid Date"), "has no invalid date text");
            }
        }
    }

    println!("\n[RSS]");
    let index = read(&dist.join("index.html")).unwrap_or_default();
    c.check(index.contains("application/rss+xml"), "RSS link tag in layout");
    let feed = read(&feed_path).unwrap_or_default();
    c.check(!feed.contains("Invalid Date"), "feed has no invalid date values");

    println!("\n{}", "=".repeat(40));
    if c.errors == 0 {
        println!("All checks passed.\n");
    } else {
        eprintln!("{} check(s) failed.\n", c.errors);
        exit(1);
    }
}
