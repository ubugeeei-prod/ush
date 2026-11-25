use super::{Piece, parse_interpolated, parse_string_literal};

fn shape(source: &str) -> alloc::vec::Vec<alloc::string::String> {
    parse_interpolated(source)
        .expect("an interpolating string")
        .expect("well formed")
        .iter()
        .map(|piece| match piece {
            Piece::Text(text) => format!("text:{text}"),
            Piece::Hole(code) => format!("hole:{code}"),
        })
        .collect()
}

#[test]
fn splits_text_and_holes_and_decodes_escapes() {
    assert_eq!(
        shape(r#"$"a $b ${c}.d \${e} \$f""#),
        ["text:a ", "hole:$b", "text: ", "hole:$c", "text:.d ${e} $f"]
    );
    assert_eq!(
        shape(r#"$"$env::HOME/${env::TMP}x""#),
        ["hole:$env::HOME", "text:/", "hole:$env::TMP", "text:x"]
    );
    assert_eq!(shape(r#"$"${f "}" x}!""#), ["hole:f \"}\" x", "text:!"]);
    assert_eq!(shape(r#"$"${$"$x"}""#), [r#"hole:$"$x""#]);
    assert_eq!(
        shape("$\"\"\"\n  a ${b + 1}\n    c\n\"\"\""),
        ["text:a ", "hole:b + 1", "text:\n  c"]
    );
}

#[test]
fn plain_strings_do_not_interpolate() {
    assert_eq!(
        parse_string_literal(r#""plain ${x} $y \$z""#).as_deref(),
        Some("plain ${x} $y \\$z")
    );
    assert_eq!(parse_string_literal("'${raw}'").as_deref(), Some("${raw}"));
    assert_eq!(
        parse_string_literal("\"\"\"\n  ${raw}\n\"\"\"").as_deref(),
        Some("${raw}")
    );
    assert!(parse_interpolated(r#""${x}""#).is_none());
    assert!(parse_interpolated(r#"$"a" + $"b""#).is_none());
}

#[test]
fn malformed_holes_are_errors() {
    for (source, message) in [
        (r#"$"${""#, "unterminated interpolation"),
        (r#"$"${ }""#, "empty interpolation"),
        (r#"$"$5""#, "write `\\$` for a dollar sign"),
        (r#"$"a $""#, "write `\\$` for a dollar sign"),
    ] {
        let error = parse_interpolated(source).unwrap().unwrap_err();
        assert!(format!("{error}").contains(message), "{error}");
    }
}
