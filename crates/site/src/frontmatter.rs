//! Shared gray-matter-style frontmatter splitting.

/// Split frontmatter into `(yaml, body)`, or return an empty YAML string.
pub fn split_frontmatter(raw: &str) -> (String, String) {
    let trimmed_start = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    if let Some(rest) = trimmed_start.strip_prefix("---") {
        if let Some(rest) = rest
            .strip_prefix('\n')
            .or_else(|| rest.strip_prefix("\r\n"))
        {
            if let Some((yaml_end, body_start)) = find_closing_fence(rest) {
                return (rest[..yaml_end].to_string(), rest[body_start..].to_string());
            }
        }
    }
    (String::new(), raw.to_string())
}

/// Locate the closing `---` fence; returns `(yaml_end, body_start)` byte offsets.
pub fn find_closing_fence(rest: &str) -> Option<(usize, usize)> {
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end_matches(['\n', '\r']) == "---" {
            return Some((offset, offset + line.len()));
        }
        offset += line.len();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_lf_crlf_and_bom_frontmatter() {
        assert_eq!(
            split_frontmatter("---\ntitle: Test\n---\nBody"),
            ("title: Test\n".into(), "Body".into())
        );
        assert_eq!(
            split_frontmatter("\u{feff}---\r\ntitle: Test\r\n---\r\nBody"),
            ("title: Test\r\n".into(), "Body".into())
        );
    }

    #[test]
    fn leaves_non_frontmatter_input_untouched() {
        let raw = "--- not a fence\nBody";
        assert_eq!(split_frontmatter(raw), (String::new(), raw.into()));
        assert_eq!(find_closing_fence("title: Test\n"), None);
        assert_eq!(find_closing_fence("title: Test\n---\nBody"), Some((12, 16)));
    }
}
