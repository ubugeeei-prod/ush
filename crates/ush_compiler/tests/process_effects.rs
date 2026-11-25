//! `ush effects` rows for commands used for their status, background
//! commands, stderr stages and `defer`.

use ush_compiler::{Effect, UshCompiler};

const PRELUDE: &str = "use $bin::*\nuse std::io::{discard_stderr, stderr_to, to_stderr}\n";

fn rows(body: &str) -> impl Fn(&str) -> ush_compiler::EffectSet {
    let report = UshCompiler::default()
        .effects_source(&format!("{PRELUDE}{body}"))
        .unwrap_or_else(|error| panic!("{error:#}"));
    move |name: &str| {
        report
            .functions
            .iter()
            .find(|f| f.name == name)
            .unwrap_or_else(|| panic!("no function {name}"))
            .inferred
            .clone()
    }
}

#[test]
fn a_command_tested_for_its_status_streams_like_a_statement() {
    let row = rows(
        "fn condition() {\n  if git \"diff\" \"--quiet\" {\n  }\n}\n\
         fn status() -> Int {\n  status_of git \"status\"\n}\n\
         fn tolerate() {\n  allow_fail git \"fetch\"\n}\n\
         fn captured() -> Bool {\n  git \"branch\" == \"main\"\n}\n",
    );
    for name in ["condition", "status", "tolerate"] {
        let effects = row(name);
        assert!(effects.contains(Effect::Exec), "{name}");
        assert!(
            effects.contains(Effect::Stdout),
            "{name}: the output streams"
        );
    }
    assert!(
        !row("captured").contains(Effect::Stdout),
        "a compared command is captured"
    );
}

#[test]
fn stderr_stages_and_to_stderr_follow_where_output_goes() {
    let row = rows(
        "fn quiet() {\n  git \"log\" | discard_stderr;\n}\n\
         fn logged() {\n  git \"log\" | stderr_to \"err\";\n}\n\
         fn warn() {\n  printf \"x\" | to_stderr;\n}\n",
    );
    assert!(
        row("quiet").contains(Effect::Stdout),
        "stdout still streams"
    );
    assert!(row("logged").contains(Effect::Fs));
    let warn = row("warn");
    assert!(warn.contains(Effect::Stderr) && !warn.contains(Effect::Stdout));
}

#[test]
fn spawn_kill_and_defer_carry_their_bodies_effects() {
    let row = rows(
        "fn serve() {\n  let server = spawn python3 \"-m\" \"http.server\"\n  defer server.kill()\n}\n\
         fn cleanup() {\n  defer {\n    rm \"-f\" \"lock\"\n  }\n}\n",
    );
    let serve = row("serve");
    assert!(serve.contains(Effect::Task) && serve.contains(Effect::Net));
    let cleanup = row("cleanup");
    assert!(cleanup.contains(Effect::Fs) && cleanup.contains(Effect::Exec));
}
