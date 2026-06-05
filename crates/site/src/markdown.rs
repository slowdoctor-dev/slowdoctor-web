//! Build-time blog loading + Markdown rendering (SSR only).
//! Ports `src/lib/blog.ts`: frontmatter parse, validation, slug derivation,
//! UTC date formatting, and content rendering (comrak + syntect).

use crate::dates::{parse_date_only, strip_date_prefix};
use crate::frontmatter::split_frontmatter;
use crate::types::{Axes, BlogPostSummary};
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use serde::Deserialize;
use std::path::Path;
use time::Date;

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
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// "Month D, YYYY" in en-US, UTC (matches Intl.DateTimeFormat). (formatDate)
fn format_date(d: Date) -> String {
    format!(
        "{} {}, {}",
        MONTHS[(d.month() as u8 - 1) as usize],
        d.day(),
        d.year()
    )
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
    Some(Axes {
        physician: p,
        engineer: e,
        life: l,
    })
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
        .replace_all(
            &html,
            r#"<a target="_blank" rel="noopener noreferrer" href="$1""#,
        )
        .into_owned()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_require_integer_values_summing_to_ten() {
        let valid = Some(RawAxes {
            physician: Some(4.0),
            engineer: Some(3.0),
            life: Some(3.0),
        });
        assert_eq!(parse_axes(&valid).unwrap().physician, 4);

        let invalid = Some(RawAxes {
            physician: Some(4.5),
            engineer: Some(3.0),
            life: Some(2.5),
        });
        assert!(parse_axes(&invalid).is_none());

        let wrong_sum = Some(RawAxes {
            physician: Some(4.0),
            engineer: Some(3.0),
            life: Some(2.0),
        });
        assert!(parse_axes(&wrong_sum).is_none());
    }

    #[test]
    fn tags_are_trimmed_deduplicated_and_empties_removed() {
        let raw = Some(vec![
            " Rust ".into(),
            "".into(),
            "Rust".into(),
            "WASM".into(),
        ]);
        assert_eq!(parse_tags(&raw), Some(vec!["Rust".into(), "WASM".into()]));
    }
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
