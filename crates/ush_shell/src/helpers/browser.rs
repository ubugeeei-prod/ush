use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, anyhow, bail};
use quick_xml::{Reader, Writer, events::Event};

use crate::commands::{CommandSearch, find_external_command};

use super::ValueStream;

pub(super) fn open_in_browser(input: &ValueStream) -> Result<()> {
    let html = render_html(input)?;
    let path = write_temp_html(&html)?;
    let command = browser_command()
        .ok_or_else(|| anyhow!("no browser opener found (tried `open` and `xdg-open`)"))?;
    let status = Command::new(command)
        .arg(&path)
        .status()
        .with_context(|| format!("failed to launch browser opener `{command}`"))?;
    if !status.success() {
        bail!("browser opener `{command}` failed with status {status}");
    }
    Ok(())
}

pub(super) fn render_html(input: &ValueStream) -> Result<String> {
    let text = input.to_text()?;
    let trimmed = text.trim_start();
    if looks_like_html_document(trimmed) {
        return Ok(text);
    }
    if looks_like_html_fragment(trimmed) {
        return Ok(format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>ush html</title></head><body>{}</body></html>",
            text
        ));
    }
    Ok(wrap_pre(
        if matches!(input, ValueStream::Json(_)) {
            "ush json"
        } else {
            "ush html"
        },
        &text,
    ))
}

pub(super) fn format_xml(source: &str) -> Result<String> {
    let mut reader = Reader::from_str(source);
    let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);
    let mut buffer = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Eof) => break,
            Ok(event) => writer.write_event(event.into_owned())?,
            Err(error) => bail!("failed to parse xml from pipeline: {error}"),
        }
        buffer.clear();
    }

    let mut output = String::from_utf8(writer.into_inner())
        .map_err(|error| anyhow!("formatted xml is not valid utf-8: {error}"))?;
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn browser_command() -> Option<&'static str> {
    let search = CommandSearch::from_process_env();
    #[cfg(target_os = "macos")]
    if find_external_command("open", &search).is_some() {
        return Some("open");
    }
    if find_external_command("xdg-open", &search).is_some() {
        return Some("xdg-open");
    }
    #[cfg(not(target_os = "macos"))]
    if find_external_command("open", &search).is_some() {
        return Some("open");
    }
    None
}

fn write_temp_html(html: &str) -> Result<PathBuf> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let path = env::temp_dir().join(format!("ush-html-{}-{nonce}.html", std::process::id()));
    fs::write(&path, html).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

fn wrap_pre(title: &str, text: &str) -> String {
    format!(
        concat!(
            "<!doctype html><html><head><meta charset=\"utf-8\">",
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">",
            "<title>{title}</title><style>",
            "body{{margin:0;background:#f6f4ee;color:#1d1b19;",
            "font:15px/1.6 ui-monospace,SFMono-Regular,Menlo,monospace;}}",
            "main{{padding:24px;}}pre{{white-space:pre-wrap;word-break:break-word;}}",
            "</style></head><body><main><pre>{body}</pre></main></body></html>"
        ),
        title = escape_html(title),
        body = escape_html(text)
    )
}

fn looks_like_html_document(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    lower.starts_with("<!doctype html") || lower.contains("<html") || lower.contains("<body")
}

fn looks_like_html_fragment(source: &str) -> bool {
    source.starts_with('<') && source.contains('>')
}

fn escape_html(source: &str) -> String {
    source
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{format_xml, render_html, wrap_pre};
    use crate::helpers::ValueStream;

    #[test]
    fn wraps_plain_text_in_html_document() {
        let html = render_html(&ValueStream::Text("hello <ush>".into())).expect("render");
        assert!(html.contains("<pre>hello &lt;ush&gt;</pre>"));
    }

    #[test]
    fn passes_through_full_html_documents() {
        let source = "<!doctype html><html><body>ok</body></html>";
        let html = render_html(&ValueStream::Text(source.into())).expect("render");
        assert_eq!(html, source);
    }

    #[test]
    fn renders_json_as_preformatted_html() {
        let html = render_html(&ValueStream::Json(json!({"name":"ush"}))).expect("render");
        assert!(html.contains("\"name\": \"ush\""));
    }

    #[test]
    fn wrap_pre_escapes_markup() {
        let html = wrap_pre("ush", "<div>ok</div>");
        assert!(html.contains("&lt;div&gt;ok&lt;/div&gt;"));
    }

    #[test]
    fn formats_xml_with_indentation() {
        let xml = format_xml("<root><item>ok</item></root>").expect("format");
        assert!(xml.contains("<root>"));
        assert!(xml.contains("  <item>ok</item>"));
    }
}
