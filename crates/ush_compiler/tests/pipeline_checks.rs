//! Compile-time checks and inferred effects of pipelines, `export` and
//! scoped blocks.

use ush_compiler::{Effect, UshCompiler};

const PRELUDE: &str = "use $bin::*\nuse std::io::{append_to, from_file, to_file}\n";

fn compile_error(body: &str) -> String {
    let source = format!("{PRELUDE}{body}");
    match UshCompiler::default().compile_source(&source) {
        Ok(_) => panic!("expected a compile error for:\n{body}"),
        Err(error) => format!("{error:#}"),
    }
}

#[test]
fn stages_must_fit_their_position() {
    for (body, message) in [
        (
            r#"to_file "x" | cat"#,
            "a file sink can only be the last stage",
        ),
        (
            r#"cat | from_file "x""#,
            "`from_file` can only be the first stage",
        ),
        (
            r#"from_file "x" |& cat"#,
            "`from_file` has no stderr to merge",
        ),
        (r#"printf "x" | "y""#, "a later stage must be a command"),
        (r#"1 | cat"#, "the first stage feeds stdin"),
        (
            "fn upper(text: String) -> String {\n  text\n}\nprintf \"x\" | upper \"y\"",
            "prints its output instead of returning it",
        ),
        (
            r#"let x = printf "x" | to_file "f""#,
            "ends in a file sink has no output to capture",
        ),
        (
            r#"printf "x" | to_file """#,
            "a file path must not be empty",
        ),
        (r#"printf "x" | to_file 1"#, "a file path must be a String"),
        (
            r#"printf "x"? | cat"#,
            "write it once, after the last stage",
        ),
        (r#"printf "x" | | cat"#, "pipeline stage 2 is empty"),
        (r#"printf "x" |"#, "continues past the end of the file"),
    ] {
        let error = compile_error(body);
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}

#[test]
fn sources_and_sinks_are_not_functions() {
    for body in [
        r#"to_file "x""#,
        r#"let x = from_file "x""#,
        r#"append_to "x""#,
    ] {
        let error = compile_error(body);
        assert!(error.contains("pipeline s"), "{body}\n=> {error}");
    }
}

#[test]
fn export_names_and_values_are_checked() {
    for (body, message) in [
        (
            r#"export __ush_x = "1""#,
            "reserved for the generated script",
        ),
        (r#"export PWD = "/""#, "use a `cd` block"),
        (r#"export IFS = ",""#, "cannot be exported"),
        (
            "let LABEL = \"a\"\nexport LABEL = \"b\"",
            "is a ush binding in this scope",
        ),
        (
            r#"export COUNT = 1"#,
            "the value of `COUNT` must be a String",
        ),
        ("cd 1 {\n}", "a `cd` block's directory must be a String"),
    ] {
        let error = compile_error(body);
        assert!(error.contains(message), "{body}\n=> {error}");
    }
}

#[test]
fn effects_follow_where_each_stage_writes() {
    let source = format!(
        "{PRELUDE}fn to_disk() {{\n  printf \"x\" | to_file \"f\"\n}}\n\
         fn to_terminal() {{\n  from_file \"f\" | sort;\n}}\n\
         fn captured() -> String {{\n  printf \"x\" | sort\n}}\n\
         fn configure() {{\n  export A = \"1\"\n}}\n\
         fn enter() {{\n  cd \"/\" {{\n  }}\n}}\n"
    );
    let report = UshCompiler::default().effects_source(&source).unwrap();
    let row = |name: &str| {
        let function = report.functions.iter().find(|f| f.name == name).unwrap();
        function.inferred.clone()
    };
    let to_disk = row("to_disk");
    assert!(to_disk.contains(Effect::Exec) && to_disk.contains(Effect::Fs));
    assert!(!to_disk.contains(Effect::Stdout), "a sink takes the output");
    let to_terminal = row("to_terminal");
    assert!(to_terminal.contains(Effect::Stdout) && to_terminal.contains(Effect::Fs));
    assert!(
        !row("captured").contains(Effect::Stdout),
        "the value is captured"
    );
    assert_eq!(row("configure").render_row(), "env");
    let enter = row("enter");
    assert!(enter.contains(Effect::Env) && enter.contains(Effect::Fs));
}
