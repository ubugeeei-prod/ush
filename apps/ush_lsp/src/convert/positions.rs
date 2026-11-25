use lsp_types::Position;
// The tooling API uses UTF-8 byte columns; LSP defaults to UTF-16 code units.
pub fn utf16_column(source: &str, line: u32, byte: u32) -> u32 {
    source
        .lines()
        .nth(line as usize)
        .unwrap_or("")
        .char_indices()
        .take_while(|(offset, _)| *offset < byte as usize)
        .map(|(_, c)| c.len_utf16() as u32)
        .sum()
}
pub fn byte_character(source: &str, position: lsp_types::Position) -> u32 {
    let line = source.lines().nth(position.line as usize).unwrap_or("");
    let mut units = 0;
    for (offset, c) in line.char_indices() {
        if units + c.len_utf16() as u32 > position.character {
            return offset as u32;
        }
        units += c.len_utf16() as u32;
    }
    line.len() as u32
}

pub(super) fn end_position(source: &str) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;
    for ch in source.chars() {
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += ch.len_utf16() as u32;
        }
    }
    Position::new(line, col)
}
