//! Build-time blog loading + Markdown rendering (SSR only).
//! Ports `src/lib/blog.ts`: frontmatter parse, validation, slug derivation,
//! UTC date formatting, and content rendering (comrak + syntect).

use crate::types::{Axes, BlogPostSummary};
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use serde::Deserialize;
use std::path::Path;
use time::{Date, Month};

/// A blog post with rendered content HTML, for the `/blog/{slug}` pages.
pub struct FullPost {
    pub summary: BlogPostSummary,
    pub content_html: String,
}

#[derive(Deserialize)]
struct RawFrontmatter {
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

const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

/// Parse `YYYY-MM-DD`, rejecting invalid calendar dates. (parseDateOnly)
fn parse_date_only(date: &str, file: &str) -> Result<Date, String> {
    let parts: Vec<&str> = date.split('-').collect();
    let ok_shape = parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()));
    if !ok_shape {
        return Err(format!(
            "Invalid date in {file}: expected YYYY-MM-DD, received \"{date}\""
        ));
    }
    let year: i32 = parts[0].parse().map_err(|_| format!("Invalid date in {file}"))?;
    let month_n: u8 = parts[1].parse().map_err(|_| format!("Invalid date in {file}"))?;
    let day: u8 = parts[2].parse().map_err(|_| format!("Invalid date in {file}"))?;
    let month = Month::try_from(month_n)
        .map_err(|_| format!("Invalid calendar date in {file}: \"{date}\""))?;
    Date::from_calendar_date(year, month, day)
        .map_err(|_| format!("Invalid calendar date in {file}: \"{date}\""))
}

/// "Month D, YYYY" in en-US, UTC (matches Intl.DateTimeFormat). (formatDate)
fn format_date(d: Date) -> String {
    format!("{} {}, {}", MONTHS[(d.month() as u8 - 1) as usize], d.day(), d.year())
}

/// Validate axes: integers in 0..=10 summing to 10, else None. (parseAxes)
fn parse_axes(raw: &Option<RawAxes>) -> Option<Axes> {
    let raw = raw.as_ref()?;
    let p = raw.physician?;
    let e = raw.engineer?;
    let l = raw.life?;
    for v in [p, e, l] {
        if !v.is_finite() || v.fract() != 0.0 || v < 0.0 || v > 10.0 {
            return None;
        }
    }
    let (p, e, l) = (p as i64, e as i64, l as i64);
    if p + e + l != 10 {
        return None;
    }
    Some(Axes { physician: p, engineer: e, life: l })
}

/// Trim, drop empties, dedup (preserving order). (parseTags)
fn parse_tags(raw: &Option<Vec<String>>) -> Option<Vec<String>> {
    let raw = raw.as_ref()?;
    let mut seen = std::collections::HashSet::new();
    let tags: Vec<String> = raw
        .iter()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .filter(|t| seen.insert(t.clone()))
        .collect();
    if tags.is_empty() {
        None
    } else {
        Some(tags)
    }
}

/// Split gray-matter style frontmatter: returns (yaml, body).
fn split_frontmatter(raw: &str) -> (String, String) {
    let trimmed_start = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    if let Some(rest) = trimmed_start.strip_prefix("---") {
        // Require the opening fence to be its own line.
        if let Some(rest) = rest.strip_prefix('\n').or_else(|| rest.strip_prefix("\r\n")) {
            // Find the closing `---` line.
            if let Some(end) = find_closing_fence(rest) {
                let yaml = rest[..end.0].to_string();
                let body = rest[end.1..].to_string();
                return (yaml, body);
            }
        }
    }
    (String::new(), raw.to_string())
}

/// Locate the closing `---` fence; returns (yaml_end, body_start) byte offsets.
fn find_closing_fence(rest: &str) -> Option<(usize, usize)> {
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let stripped = line.trim_end_matches(['\n', '\r']);
        if stripped == "---" {
            return Some((offset, offset + line.len()));
        }
        offset += line.len();
    }
    None
}

/// Render Markdown to HTML with syntax highlighting + external-link hardening.
fn render_markdown(body: &str) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.table = true;

    let adapter = SyntectAdapter::new(Some("base16-ocean.dark"));
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let html = markdown_to_html_with_plugins(body, &options, &plugins);

    // Let the .prose card background show through highlighted code blocks
    // (strip syntect's inline background-color on <pre>).
    let html = html.replace(" style=\"background-color:#2b303b;\"", "");

    // External links open in a new tab, like mdx-components.tsx.
    let external = regex::Regex::new(r#"<a href="(https?://[^"]*)""#).unwrap();
    external
        .replace_all(&html, r#"<a target="_blank" rel="noopener noreferrer" href="$1""#)
        .into_owned()
}

fn strip_date_prefix(stem: &str) -> String {
    // Strip a leading YYYY-MM-DD- date prefix.
    let bytes = stem.as_bytes();
    if bytes.len() > 11
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'-'
        && bytes[..4].iter().all(|b| b.is_ascii_digit())
        && bytes[5..7].iter().all(|b| b.is_ascii_digit())
        && bytes[8..10].iter().all(|b| b.is_ascii_digit())
    {
        stem[11..].to_string()
    } else {
        stem.to_string()
    }
}

fn parse_one(path: &Path) -> Result<FullPost, String> {
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read {file_name}: {e}"))?;
    let (yaml, body) = split_frontmatter(&raw);

    let fm: RawFrontmatter = serde_yaml::from_str(&yaml)
        .map_err(|e| format!("Invalid frontmatter in {file_name}: {e}"))?;

    let mut missing = Vec::new();
    if fm.title.as_deref().unwrap_or("").is_empty() {
        missing.push("title");
    }
    if fm.date.as_deref().unwrap_or("").is_empty() {
        missing.push("date");
    }
    if fm.description.as_deref().unwrap_or("").is_empty() {
        missing.push("description");
    }
    if !missing.is_empty() {
        return Err(format!(
            "Missing required frontmatter in {file_name}: {}",
            missing.join(", ")
        ));
    }

    let date_str = fm.date.unwrap();
    let date = parse_date_only(&date_str, file_name)?;

    let stem = file_name.strip_suffix(".md").unwrap_or(file_name);
    let slug = strip_date_prefix(stem);

    let summary = BlogPostSummary {
        slug,
        title: fm.title.unwrap(),
        date: date_str,
        description: fm.description.unwrap(),
        image: fm.image,
        tags: parse_tags(&fm.tags),
        axes: parse_axes(&fm.axes),
        formatted_date: format_date(date),
    };

    Ok(FullPost {
        summary,
        content_html: render_markdown(&body),
    })
}

/// Load + render every `.md` post, sorted newest-first. Errors fail the build.
pub fn load_posts(blog_dir: &Path) -> Result<Vec<FullPost>, String> {
    let mut posts = Vec::new();
    for entry in std::fs::read_dir(blog_dir).map_err(|e| format!("read blog dir: {e}"))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            posts.push(parse_one(&path)?);
        }
    }
    // Sort by date string descending (YYYY-MM-DD is lexicographically chronological).
    posts.sort_by(|a, b| b.summary.date.cmp(&a.summary.date));
    Ok(posts)
}
