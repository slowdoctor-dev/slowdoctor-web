//! Asset pipeline: copy `public/` into `dist/`, content-hash the CSS.

use std::fs;
use std::path::Path;

/// FNV-1a 64-bit, lowered to 8 hex chars — a content hash for cache busting.
fn hash_bytes(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:08x}", (hash & 0xffff_ffff) as u32)
}

/// Recursively copy `src` into `dst` (creating `dst`).
pub fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create dst dir");
    for entry in fs::read_dir(src).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// Hash `dist/_assets/app.css`, rename it to `app-<hash>.css`, and return the
/// site-relative href. Falls back to any existing `app-*.css`, then to the
/// unhashed path (so `build-site` still works if Tailwind hasn't run).
pub fn hash_and_rename_css(dist: &Path) -> String {
    let assets = dist.join("_assets");
    let plain = assets.join("app.css");
    if let Ok(bytes) = fs::read(&plain) {
        let hashed = format!("app-{}.css", hash_bytes(&bytes));
        fs::rename(&plain, assets.join(&hashed)).expect("rename css");
        return format!("/_assets/{hashed}");
    }
    if let Ok(read) = fs::read_dir(&assets) {
        for entry in read.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("app-") && name.ends_with(".css") {
                return format!("/_assets/{name}");
            }
        }
    }
    "/_assets/app.css".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv_hash_is_stable() {
        assert_eq!(hash_bytes(b""), "84222325");
        assert_eq!(hash_bytes(b"hello"), "80aabd0b");
    }
}
