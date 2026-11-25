//! Sourcemaps and `ush explain` locate the shell behind command conditions,
//! `status_of`, `spawn`, stderr stages and deferred blocks.

use ush_compiler::UshCompiler;

const SOURCE: &str = "use $bin::*
use std::io::discard_stderr

defer {
  rm \"-f\" \"lock\"
}
if ! git \"diff\" \"--quiet\" | discard_stderr {
  print \"dirty\"
}
let code = status_of cargo \"test\"
let server = spawn sleep \"30\"
server.kill()
";

#[test]
fn generated_lines_map_back_to_their_statements() {
    let compiled = UshCompiler::default()
        .compile_source_with_sourcemap(SOURCE)
        .expect("compile");
    let line_of = |needle: &str| {
        compiled
            .shell
            .lines()
            .position(|line| line.contains(needle))
            .map(|index| index + 1)
            .unwrap_or_else(|| panic!("missing generated line: {needle}"))
    };
    for (needle, source_line) in [
        ("command rm '-f' 'lock'", 5),
        ("(command git 'diff' '--quiet') 2>/dev/null", 7),
        ("if command cargo 'test'; then", 10),
        ("command sleep '30'", 11),
        ("__ush_task_kill ", 12),
    ] {
        let generated = line_of(needle);
        assert_eq!(
            compiled.sourcemap.source_line(generated),
            Some(source_line),
            "{needle}"
        );
    }
}
