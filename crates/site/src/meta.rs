//! Per-page `<head>` metadata + JSON-LD rendering.
//! Ports `src/lib/metadata.ts`, `src/components/json-ld.tsx`, and the inline
//! metadata objects from the route pages. Global tags (charset, viewport, CSS,
//! fonts, RSS alternate) are added by the build shell, not here.

use crate::data::{
    AUTHOR_NAME, DESCRIPTION_FULL, OG_IMAGE, SITE_NAME, SITE_TITLE, SITE_TITLE_TEMPLATE, SITE_URL,
};
use serde_json::Value;

#[derive(Clone, Copy, PartialEq)]
pub enum OgType {
    Website,
    Article,
}

impl OgType {
    fn as_str(self) -> &'static str {
        match self {
            OgType::Website => "website",
            OgType::Article => "article",
        }
    }
}

/// Fully-resolved head metadata for one page.
pub struct HeadMeta {
    pub title: String,
    pub description: String,
    pub canonical: String,
    pub og_title: String,
    pub og_type: OgType,
    pub og_url: String,
    pub og_image: String,
    /// Default OG image carries dimensions + alt; post images do not.
    pub og_image_dims: bool,
    pub twitter_title: String,
    pub twitter_image: String,
    pub json_ld: Vec<Value>,
}

/// Absolute URL for a site-relative path.
pub fn absolute_url(path: &str) -> String {
    format!("{SITE_URL}{path}")
}

/// Port of `buildPageMetadata` (src/lib/metadata.ts).
///
/// `absolute_title` true => `<title>` is the raw title; otherwise the site
/// template (`%s | Joonho Lim`) is applied. OG/Twitter title get the
/// `… | Joonho Lim` author suffix unless absolute.
pub fn build_page_meta(
    title: &str,
    description: &str,
    path: &str,
    og_type: OgType,
    absolute_title: bool,
    json_ld: Vec<Value>,
) -> HeadMeta {
    let abs = absolute_url(path);
    let title_tag = if absolute_title {
        title.to_string()
    } else {
        SITE_TITLE_TEMPLATE.replace("%s", title)
    };
    let og_title = if absolute_title {
        title.to_string()
    } else {
        format!("{title} | {AUTHOR_NAME}")
    };
    let og_image = absolute_url(OG_IMAGE);
    HeadMeta {
        title: title_tag,
        description: description.to_string(),
        canonical: abs.clone(),
        og_title: og_title.clone(),
        og_type,
        og_url: abs,
        og_image: og_image.clone(),
        og_image_dims: true,
        twitter_title: og_title,
        twitter_image: og_image,
        json_ld,
    }
}

pub fn article_meta(
    title: &str,
    description: &str,
    path: &str,
    image: Option<&str>,
    json_ld: Vec<Value>,
) -> HeadMeta {
    let canonical = absolute_url(path);
    let image_url = absolute_url(image.unwrap_or(OG_IMAGE));
    HeadMeta {
        title: SITE_TITLE_TEMPLATE.replace("%s", title),
        description: description.to_string(),
        canonical: canonical.clone(),
        og_title: title.to_string(),
        og_type: OgType::Article,
        og_url: canonical,
        og_image: image_url.clone(),
        og_image_dims: false,
        twitter_title: title.to_string(),
        twitter_image: image_url,
        json_ld,
    }
}

/// The layout default head (used by pages with no metadata export, e.g. 404).
pub fn default_meta(path: &str) -> HeadMeta {
    let abs = absolute_url(path);
    let og_image = absolute_url(OG_IMAGE);
    HeadMeta {
        title: SITE_TITLE.to_string(),
        description: DESCRIPTION_FULL.to_string(),
        canonical: abs.clone(),
        og_title: SITE_TITLE.to_string(),
        og_type: OgType::Website,
        og_url: abs,
        og_image: og_image.clone(),
        og_image_dims: true,
        twitter_title: SITE_TITLE.to_string(),
        twitter_image: og_image,
        json_ld: Vec::new(),
    }
}

/// Escape a string for use in an HTML attribute value or text.
pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Serialize + escape JSON-LD exactly like `components/json-ld.tsx`:
/// escape `<` and the JS line/paragraph separators so the payload cannot
/// terminate the `<script>` early.
pub fn stringify_json_ld(value: &Value) -> String {
    serde_json::to_string(value)
        .unwrap_or_default()
        .replace('<', "\\u003c")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// Render the per-page portion of `<head>` (title, description, canonical,
/// author/creator/publisher, OpenGraph, Twitter, JSON-LD scripts).
pub fn render_head(meta: &HeadMeta) -> String {
    let mut h = String::new();
    h.push_str(&format!("<title>{}</title>", esc(&meta.title)));
    h.push_str(&format!(
        "<meta name=\"description\" content=\"{}\"/>",
        esc(&meta.description)
    ));
    h.push_str(&format!(
        "<link rel=\"canonical\" href=\"{}\"/>",
        esc(&meta.canonical)
    ));
    h.push_str(&format!(
        "<meta name=\"author\" content=\"{}\"/>",
        esc(AUTHOR_NAME)
    ));
    h.push_str(&format!(
        "<meta name=\"creator\" content=\"{}\"/>",
        esc(AUTHOR_NAME)
    ));
    h.push_str(&format!(
        "<meta name=\"publisher\" content=\"{}\"/>",
        esc(SITE_NAME)
    ));

    // OpenGraph
    h.push_str(&format!(
        "<meta property=\"og:title\" content=\"{}\"/>",
        esc(&meta.og_title)
    ));
    h.push_str(&format!(
        "<meta property=\"og:description\" content=\"{}\"/>",
        esc(&meta.description)
    ));
    h.push_str(&format!(
        "<meta property=\"og:url\" content=\"{}\"/>",
        esc(&meta.og_url)
    ));
    h.push_str(&format!(
        "<meta property=\"og:type\" content=\"{}\"/>",
        meta.og_type.as_str()
    ));
    h.push_str(&format!(
        "<meta property=\"og:site_name\" content=\"{}\"/>",
        esc(SITE_NAME)
    ));
    h.push_str("<meta property=\"og:locale\" content=\"en_US\"/>");
    h.push_str(&format!(
        "<meta property=\"og:image\" content=\"{}\"/>",
        esc(&meta.og_image)
    ));
    if meta.og_image_dims {
        h.push_str("<meta property=\"og:image:width\" content=\"1200\"/>");
        h.push_str("<meta property=\"og:image:height\" content=\"630\"/>");
        h.push_str(&format!(
            "<meta property=\"og:image:alt\" content=\"{}\"/>",
            esc(SITE_NAME)
        ));
    }

    // Twitter
    h.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\"/>");
    h.push_str(&format!(
        "<meta name=\"twitter:title\" content=\"{}\"/>",
        esc(&meta.twitter_title)
    ));
    h.push_str(&format!(
        "<meta name=\"twitter:description\" content=\"{}\"/>",
        esc(&meta.description)
    ));
    h.push_str(&format!(
        "<meta name=\"twitter:image\" content=\"{}\"/>",
        esc(&meta.twitter_image)
    ));

    // JSON-LD
    for value in &meta.json_ld {
        h.push_str(&format!(
            "<script type=\"application/ld+json\">{}</script>",
            stringify_json_ld(value)
        ));
    }

    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn escapes_html_text_and_attributes() {
        assert_eq!(
            esc("<a href='x'>&\""),
            "&lt;a href=&#39;x&#39;&gt;&amp;&quot;"
        );
    }

    #[test]
    fn json_ld_escapes_script_breakouts_and_js_separators() {
        let rendered = stringify_json_ld(&json!({"value": "</script>\u{2028}\u{2029}"}));
        assert_eq!(rendered, r#"{"value":"\u003c/script>\u2028\u2029"}"#);
    }
}
