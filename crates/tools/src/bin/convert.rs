//! Convert incoming Markdown drafts to blog posts. Port of `scripts/convert-md.cts`.
//! Usage:
//!   cargo run -p tools --bin convert              # all files in incoming/
//!   cargo run -p tools --bin convert -- file.md   # single file

use regex::Regex;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::exit;
use time::{Date, Month, OffsetDateTime};

const INCOMING: &str = "src/content/incoming";
const BLOG: &str = "src/content/blog";

#[derive(Deserialize, Default)]
struct ExistingFm {
    title: Option<String>,
    date: Option<String>,
    description: Option<String>,
    image: Option<String>,
    tags: Option<Vec<String>>,
    axes: Option<RawAxes>,
}

#[derive(Deserialize)]
struct RawAxes {
    physician: Option<f64>,
    engineer: Option<f64>,
    life: Option<f64>,
}

fn today() -> String {
    let d = OffsetDateTime::now_utc().date();
    format!("{:04}-{:02}-{:02}", d.year(), d.month() as u8, d.day())
}

fn valid_date(date: &str) -> bool {
    let p: Vec<&str> = date.split('-').collect();
    if p.len() != 3 || p[0].len() != 4 || p[1].len() != 2 || p[2].len() != 2 {
        return false;
    }
    let (Ok(y), Ok(m), Ok(d)) = (p[0].parse::<i32>(), p[1].parse::<u8>(), p[2].parse::<u8>())
    else {
        return false;
    };
    Month::try_from(m)
        .ok()
        .and_then(|m| Date::from_calendar_date(y, m, d).ok())
        .is_some()
}

fn split_frontmatter(raw: &str) -> (String, String) {
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    if let Some(rest) = raw.strip_prefix("---\n").or_else(|| raw.strip_prefix("---\r\n")) {
        let mut offset = 0;
        for line in rest.split_inclusive('\n') {
            if line.trim_end_matches(['\n', '\r']) == "---" {
                return (rest[..offset].to_string(), rest[offset + line.len()..].to_string());
            }
            offset += line.len();
        }
    }
    (String::new(), raw.to_string())
}

fn derive_slug(file_name: &str) -> String {
    let base = file_name.trim_end_matches(".mdx").trim_end_matches(".md");
    // lead convention: YYYY-MM-DD_CHANNEL_english-kebab-slug
    let lead = Regex::new(r"^\d{4}-\d{2}-\d{2}_[A-Za-z0-9-]+_(.+)$").unwrap();
    let source = lead
        .captures(base)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| base.to_string());
    let non_alnum = Regex::new(r"[^a-z0-9]+").unwrap();
    let kebab = non_alnum
        .replace_all(&source.to_lowercase(), "-")
        .trim_matches('-')
        .to_string();
    if kebab.is_empty() {
        "untitled".to_string()
    } else {
        kebab
    }
}

fn derive_date(file_name: &str) -> String {
    if file_name.len() >= 10 && valid_date(&file_name[..10]) {
        file_name[..10].to_string()
    } else {
        today()
    }
}

fn derive_title(content: &str, slug: &str) -> String {
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if let Some(h) = t.strip_prefix("# ") {
            return h.trim().to_string();
        }
        break;
    }
    slug.split('-')
        .map(|w| {
            let mut chars = w.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn derive_description(content: &str) -> String {
    let mut paragraphs: Vec<String> = Vec::new();
    let mut current = String::new();
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with('#') || t.starts_with("---") || t.starts_with("- **") {
            if !current.is_empty() {
                paragraphs.push(std::mem::take(&mut current));
            }
            continue;
        }
        if t.is_empty() {
            if !current.is_empty() {
                paragraphs.push(std::mem::take(&mut current));
            }
            continue;
        }
        if t.starts_with('[') && t.ends_with(']') {
            continue;
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(t);
    }
    if !current.is_empty() {
        paragraphs.push(current);
    }

    let img = Regex::new(r"!\[.*?\]\(.*?\)").unwrap();
    let link = Regex::new(r"\[(.*?)\]\(.*?\)").unwrap();
    let emph = Regex::new(r"[*_`~]").unwrap();
    for p in paragraphs {
        let s = img.replace_all(&p, "");
        let s = link.replace_all(&s, "$1");
        let s = emph.replace_all(&s, "");
        let plain = s.trim();
        if plain.chars().count() >= 20 {
            if plain.chars().count() > 160 {
                let truncated: String = plain.chars().take(157).collect();
                return format!("{truncated}...");
            }
            return plain.to_string();
        }
    }
    "TODO: Write a short description for this post.".to_string()
}

fn strip_leading_h1(content: &str) -> String {
    let mut lines: Vec<&str> = content.lines().collect();
    if let Some(idx) = lines.iter().position(|l| !l.trim().is_empty()) {
        if lines[idx].trim().starts_with("# ") {
            lines.remove(idx);
            while lines.first().is_some_and(|l| l.trim().is_empty()) {
                lines.remove(0);
            }
        }
    }
    lines.join("\n")
}

fn parse_axes(raw: &Option<RawAxes>) -> Option<(i64, i64, i64)> {
    let a = raw.as_ref()?;
    let (p, e, l) = (a.physician?, a.engineer?, a.life?);
    for v in [p, e, l] {
        if v.fract() != 0.0 || v < 0.0 || v > 10.0 {
            return None;
        }
    }
    let (p, e, l) = (p as i64, e as i64, l as i64);
    (p + e + l == 10).then_some((p, e, l))
}

fn find_existing_for_slug(slug: &str) -> Option<String> {
    let bare = format!("{slug}.md");
    let prefixed = Regex::new(&format!(r"^\d{{4}}-\d{{2}}-\d{{2}}-{}\.md$", regex::escape(slug))).unwrap();
    std::fs::read_dir(BLOG)
        .ok()?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .find(|n| *n == bare || prefixed.is_match(n))
}

fn yaml_quote(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn build_output(fm: &ExistingFm, title: &str, date: &str, description: &str, body: &str) -> String {
    let mut out = String::from("---\n");
    out.push_str(&format!("title: {}\n", yaml_quote(title)));
    out.push_str(&format!("date: {}\n", yaml_quote(date)));
    out.push_str(&format!("description: {}\n", yaml_quote(description)));
    if let Some(image) = &fm.image {
        out.push_str(&format!("image: {}\n", yaml_quote(image)));
    }
    if let Some(tags) = &fm.tags {
        let tags: Vec<&String> = tags.iter().collect();
        if !tags.is_empty() {
            out.push_str("tags:\n");
            for t in tags {
                out.push_str(&format!("  - {}\n", yaml_quote(t)));
            }
        }
    }
    if let Some((p, e, l)) = parse_axes(&fm.axes) {
        out.push_str(&format!("axes:\n  physician: {p}\n  engineer: {e}\n  life: {l}\n"));
    }
    out.push_str("---\n\n");
    out.push_str(body.trim());
    out.push('\n');
    out
}

struct Outcome {
    source: String,
    target: String,
    skipped: Option<String>,
}

fn convert_file(file_name: &str) -> Result<Outcome, String> {
    let raw = std::fs::read_to_string(Path::new(INCOMING).join(file_name))
        .map_err(|e| format!("read {file_name}: {e}"))?;
    let (yaml, body) = split_frontmatter(&raw);
    let fm: ExistingFm = if yaml.trim().is_empty() {
        ExistingFm::default()
    } else {
        serde_yaml::from_str(&yaml).map_err(|e| format!("frontmatter {file_name}: {e}"))?
    };

    let slug = derive_slug(file_name);
    let date = fm.date.clone().unwrap_or_else(|| derive_date(file_name));
    if !valid_date(&date) {
        return Err(format!("Invalid date in {file_name}: \"{date}\""));
    }

    let target = format!("{date}-{slug}.md");
    if let Some(existing) = find_existing_for_slug(&slug) {
        return Ok(Outcome {
            source: file_name.to_string(),
            target,
            skipped: Some(format!("slug \"{slug}\" already exists as {existing}")),
        });
    }

    let title = fm.title.clone().unwrap_or_else(|| derive_title(&body, &slug));
    let description = fm
        .description
        .clone()
        .unwrap_or_else(|| derive_description(&body));
    let clean_body = strip_leading_h1(&body);
    let output = build_output(&fm, &title, &date, &description, &clean_body);

    std::fs::write(Path::new(BLOG).join(&target), output).map_err(|e| e.to_string())?;
    std::fs::remove_file(Path::new(INCOMING).join(file_name)).map_err(|e| e.to_string())?;

    Ok(Outcome { source: file_name.to_string(), target, skipped: None })
}

fn main() {
    if !Path::new(INCOMING).exists() {
        eprintln!("Incoming directory not found: {INCOMING}");
        exit(1);
    }

    let arg = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let arg = arg.trim();
    let files: Vec<String> = if !arg.is_empty() {
        let resolved = if arg.ends_with(".md") || arg.ends_with(".mdx") {
            arg.to_string()
        } else {
            format!("{arg}.md")
        };
        let full: PathBuf = Path::new(INCOMING).join(&resolved);
        let canon_incoming = Path::new(INCOMING).canonicalize().unwrap_or_default();
        if !full.starts_with(&canon_incoming) && !full.starts_with(INCOMING) {
            eprintln!("File must be inside {INCOMING}: {arg}");
            exit(1);
        }
        if !full.exists() {
            eprintln!("File not found: {}", full.display());
            exit(1);
        }
        vec![resolved]
    } else {
        let mut v: Vec<String> = std::fs::read_dir(INCOMING)
            .expect("read incoming")
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".md") || n.ends_with(".mdx"))
            .collect();
        v.sort();
        v
    };

    if files.is_empty() {
        println!("No files to convert in {INCOMING}/");
        return;
    }

    let (mut converted, mut skipped) = (0u32, 0u32);
    for file in files {
        match convert_file(&file) {
            Ok(o) if o.skipped.is_some() => {
                println!("  SKIP: {} -> {} ({})", o.source, o.target, o.skipped.unwrap());
                skipped += 1;
            }
            Ok(o) => {
                println!("  OK: {} -> {}", o.source, o.target);
                converted += 1;
            }
            Err(e) => {
                eprintln!("  ERROR: {file}: {e}");
                exit(1);
            }
        }
    }
    println!("\nConverted: {converted}, Skipped: {skipped}");
}
