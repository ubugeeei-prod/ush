//! Editor documents are URIs; the compiler reads modules from paths.

use std::path::PathBuf;

use lsp_types::Uri;

/// The local file behind a `file://` URI, percent-decoded.
pub fn file_path(uri: &Uri) -> Option<PathBuf> {
    let rest = uri.as_str().strip_prefix("file://")?;
    // `file:///tmp/x` has an empty authority; `file://localhost/tmp/x` names this host.
    let path = rest.strip_prefix("localhost").unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let bytes = path.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && let Some(byte) = path
                .get(index + 1..index + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            decoded.push(byte);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok().map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_uris_decode_to_paths() {
        let path = |uri: &str| file_path(&uri.parse().unwrap());
        assert_eq!(
            path("file:///tmp/my%20dir/x.ush"),
            Some(PathBuf::from("/tmp/my dir/x.ush"))
        );
        assert_eq!(
            path("file://localhost/a.ush"),
            Some(PathBuf::from("/a.ush"))
        );
        assert_eq!(path("untitled:Untitled-1"), None);
    }
}
