//! sitemap.xml + feed.xml generation. Ports `scripts/generate-sitemap.cts`
//! and `scripts/generate-feed.cts`.

use site::data::SITE_URL;
use site::dates::parse_date_only;
use site::types::BlogPostSummary;
use std::fs;
use std::path::Path;
use time::{Date, OffsetDateTime};

const WEEKDAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// "Mon, 07 Apr 2026 00:00:00 GMT" — matches JS Date.toUTCString().
fn to_utc_string(date: Date, h: u8, m: u8, s: u8) -> String {
    let wd = WEEKDAYS[date.weekday().number_days_from_monday() as usize];
    let mon = MONTHS[(date.month() as u8 - 1) as usize];
    format!(
        "{wd}, {:02} {mon} {:04} {:02}:{:02}:{:02} GMT",
        date.day(),
        date.year(),
        h,
        m,
        s
    )
}

fn escape_loc(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Build-time static routes (in the order generate-sitemap.cts emits them).
fn sitemap_url(route: &str, lastmod: Option<&str>) -> String {
    let loc = escape_loc(&format!("{SITE_URL}{route}"));
    match lastmod {
        Some(d) => format!("  <url>\n    <loc>{loc}</loc>\n    <lastmod>{d}</lastmod>\n  </url>"),
        None => format!("  <url>\n    <loc>{loc}</loc>\n  </url>"),
    }
}

pub fn write_sitemap(dist: &Path, posts: &[BlogPostSummary]) {
    // Static routes: "/", other statics (alphabetical), then "/blog".
    let other_static = ["/cv", "/engineer", "/links", "/physician"];

    // Blog routes sorted by route ascending, lastmod = post date.
    let mut blog: Vec<_> = posts
        .iter()
        .map(|p| (format!("/blog/{}", p.slug), p.date.clone()))
        .collect();
    blog.sort_by(|a, b| a.0.cmp(&b.0));

    let mut lines = vec![
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string(),
        "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">".to_string(),
        sitemap_url("/", None),
    ];
    for route in other_static {
        lines.push(sitemap_url(route, None));
    }
    lines.push(sitemap_url("/blog", None));
    for (route, lastmod) in &blog {
        lines.push(sitemap_url(route, Some(lastmod)));
    }
    lines.push("</urlset>".to_string());
    lines.push(String::new());

    fs::write(dist.join("sitemap.xml"), lines.join("\n")).expect("write sitemap");
}

pub fn write_feed(dist: &Path, posts: &[BlogPostSummary]) {
    // posts arrive newest-first already.
    let now = OffsetDateTime::now_utc();
    let last_build = to_utc_string(now.date(), now.hour(), now.minute(), now.second());

    let mut items = Vec::new();
    for post in posts {
        let link = format!("{SITE_URL}/blog/{}", post.slug);
        let date = parse_date_only(&post.date, &format!("post {}", post.slug))
            .expect("posts were validated while loading");
        let pub_date = to_utc_string(date, 0, 0, 0);
        items.push(
            [
                "    <item>".to_string(),
                format!("      <title>{}</title>", escape_xml(&post.title)),
                format!("      <link>{}</link>", escape_xml(&link)),
                format!(
                    "      <description>{}</description>",
                    escape_xml(&post.description)
                ),
                format!("      <pubDate>{pub_date}</pubDate>"),
                format!(
                    "      <guid isPermaLink=\"true\">{}</guid>",
                    escape_xml(&link)
                ),
                "    </item>".to_string(),
            ]
            .join("\n"),
        );
    }

    let mut lines = vec![
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_string(),
        "<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">".to_string(),
        "  <channel>".to_string(),
        "    <title>Joonho Lim - Blog</title>".to_string(),
        format!("    <link>{SITE_URL}/blog</link>"),
        format!("    <atom:link href=\"{SITE_URL}/feed.xml\" rel=\"self\" type=\"application/rss+xml\" />"),
        "    <description>Writing by Joonho Lim on medicine, engineering, and the slower path.</description>".to_string(),
        "    <language>en-us</language>".to_string(),
        format!("    <lastBuildDate>{last_build}</lastBuildDate>"),
    ];
    lines.extend(items);
    lines.push("  </channel>".to_string());
    lines.push("</rss>".to_string());
    lines.push(String::new());

    fs::write(dist.join("feed.xml"), lines.join("\n")).expect("write feed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_utc_dates_with_correct_weekdays() {
        let leap_day = parse_date_only("2024-02-29", "test").unwrap();
        assert_eq!(
            to_utc_string(leap_day, 0, 0, 0),
            "Thu, 29 Feb 2024 00:00:00 GMT"
        );

        let new_year = parse_date_only("2026-01-01", "test").unwrap();
        assert_eq!(
            to_utc_string(new_year, 23, 59, 58),
            "Thu, 01 Jan 2026 23:59:58 GMT"
        );
    }
}
