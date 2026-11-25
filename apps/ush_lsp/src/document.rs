use anyhow::{Result, anyhow, ensure};
use lsp_types::{TextDocumentContentChangeEvent, Uri};
use std::{collections::BTreeMap, sync::Arc};
use ush_tooling::SemanticToken;

#[derive(Default)]
pub struct DocumentStore {
    docs: BTreeMap<Uri, Document>,
}
struct Document {
    text: Arc<str>,
    version: i32,
    tokens: Option<Vec<SemanticToken>>,
}
impl DocumentStore {
    pub fn open(&mut self, uri: Uri, text: String, version: i32) {
        self.docs.insert(
            uri,
            Document {
                text: text.into(),
                version,
                tokens: None,
            },
        );
    }
    pub fn close(&mut self, uri: &Uri) {
        self.docs.remove(uri);
    }
    pub fn version(&self, uri: &Uri) -> Option<i32> {
        self.docs.get(uri).map(|d| d.version)
    }
    pub fn update(
        &mut self,
        uri: &Uri,
        version: i32,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<bool> {
        let doc = self
            .docs
            .get_mut(uri)
            .ok_or_else(|| anyhow!("document not open"))?;
        if version <= doc.version {
            return Ok(false);
        }
        let mut text = doc.text.to_string();
        for change in changes {
            if let Some(range) = change.range {
                let start = offset(&text, range.start)?;
                let end = offset(&text, range.end)?;
                ensure!(start <= end, "reversed document change range");
                text.replace_range(start..end, &change.text);
            } else {
                text = change.text;
            }
        }
        doc.text = text.into();
        doc.version = version;
        doc.tokens = None;
        Ok(true)
    }
    pub fn read(&self, uri: &Uri) -> Result<Arc<str>> {
        self.docs
            .get(uri)
            .map(|d| Arc::clone(&d.text))
            .ok_or_else(|| anyhow!("document not open: {}", uri.as_str()))
    }
    pub fn tokens(&mut self, uri: &Uri) -> Result<Vec<SemanticToken>> {
        let doc = self
            .docs
            .get_mut(uri)
            .ok_or_else(|| anyhow!("document not open"))?;
        Ok(doc
            .tokens
            .get_or_insert_with(|| ush_tooling::semantic_tokens(&doc.text))
            .clone())
    }
}
fn offset(source: &str, p: lsp_types::Position) -> Result<usize> {
    let mut start = 0;
    for _ in 0..p.line {
        start += source[start..]
            .find('\n')
            .ok_or_else(|| anyhow!("change line out of range"))?
            + 1;
    }
    let line = source[start..].split('\n').next().unwrap_or("");
    ensure!(
        p.character as usize <= line.encode_utf16().count(),
        "change column out of range"
    );
    let byte = crate::convert::byte_character(source, p) as usize;
    ensure!(
        line[..byte].encode_utf16().count() == p.character as usize,
        "change splits a Unicode character"
    );
    Ok(start + byte)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::{Position, Range};
    #[test]
    fn incremental_unicode_edits_are_atomic_versioned_and_invalidate_tokens() {
        let uri: Uri = "file:///tmp/unicode.ush".parse().unwrap();
        let mut docs = DocumentStore::default();
        docs.open(uri.clone(), "print \"😀\"\n".into(), 1);
        let before = docs.tokens(&uri).unwrap();
        let edit = |start, end, text: &str| TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, start), Position::new(0, end))),
            range_length: None,
            text: text.into(),
        };
        assert!(docs.update(&uri, 2, vec![edit(7, 8, "x")]).is_err());
        assert_eq!(docs.version(&uri), Some(1));
        assert!(docs.update(&uri, 2, vec![edit(7, 9, "日本")]).unwrap());
        assert_eq!(&*docs.read(&uri).unwrap(), "print \"日本\"\n");
        assert_ne!(before, docs.tokens(&uri).unwrap());
        assert!(!docs.update(&uri, 1, vec![edit(7, 9, "old")]).unwrap());
        docs.close(&uri);
        assert!(docs.read(&uri).is_err());
    }
}
