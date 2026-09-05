use alloc::string::ToString;

use crate::UshCompiler;

use super::{Effect, EffectSet};

fn effects_of(source: &str, function: &str) -> EffectSet {
    let report = UshCompiler.effects_source(source).expect("analyze");
    report
        .functions
        .iter()
        .find(|item| item.name == function)
        .unwrap_or_else(|| panic!("no function `{function}` in report"))
        .inferred
        .clone()
}

#[test]
fn a_function_that_only_moves_values_around_needs_nothing() {
    let effects = effects_of(
        "fn slug(name: String) -> String {\n  name + \"-x\"\n}\n",
        "slug",
    );

    assert!(effects.is_empty());
    assert_eq!(effects.to_string(), "pure");
}

#[test]
fn stdlib_calls_classify_by_module() {
    let source = concat!(
        "fn a() -> String {\n  std::fs::read_text(\"f\")\n}\n",
        "fn b() -> String {\n  std::http::get(\"u\")\n}\n",
        "fn c() -> String {\n  std::env::get(\"HOME\")\n}\n",
        "fn d() -> String {\n  std::command::capture(\"echo\")\n}\n",
        "fn e() -> String {\n  std::string::replace(\"a\", \"b\", \"c\")\n}\n",
    );

    assert!(effects_of(source, "a").contains(Effect::Fs));
    assert!(effects_of(source, "b").contains(Effect::Net));
    assert!(effects_of(source, "c").contains(Effect::Env));
    assert!(effects_of(source, "d").contains(Effect::Exec));
    assert!(effects_of(source, "e").is_empty());
}

#[test]
fn print_and_inline_shell_are_effects_of_their_own() {
    let source = concat!(
        "fn shout(name: String) {\n  print name\n}\n",
        "fn poke() {\n  $ echo hi\n}\n",
    );

    assert_eq!(effects_of(source, "shout"), EffectSet::of(Effect::Io));
    assert_eq!(effects_of(source, "poke"), EffectSet::of(Effect::Exec));
}

#[test]
fn effects_propagate_through_calls() {
    let source = concat!(
        "fn fetch(url: String) -> String {\n  std::http::get(url)\n}\n",
        "fn save(url: String) -> String {\n  let body = fetch(url)\n  std::fs::write_text(\"f\", body)\n  body\n}\n",
        "fn run(url: String) {\n  print save(url)\n}\n",
    );

    let run = effects_of(source, "run");
    assert!(run.contains(Effect::Net), "{run}");
    assert!(run.contains(Effect::Fs), "{run}");
    assert!(run.contains(Effect::Io), "{run}");
}

#[test]
fn mutual_recursion_reaches_a_fixpoint() {
    // `ping` only touches the network through `pong`, and `pong` only
    // through `ping`. A single pass would report one of them pure.
    let source = concat!(
        "fn ping(url: String) -> String {\n  pong(url)\n}\n",
        "fn pong(url: String) -> String {\n  let body = std::http::get(url)\n  ping(body)\n}\n",
    );

    assert!(effects_of(source, "ping").contains(Effect::Net));
    assert!(effects_of(source, "pong").contains(Effect::Net));
}

#[test]
fn an_effect_row_that_does_not_cover_the_body_is_rejected() {
    let error = UshCompiler
        .compile_source(
            "fn fetch(url: String) -> String / { fs } {\n  std::http::get(url)\n}\nprint fetch(\"u\")\n",
        )
        .expect_err("a row that misses an effect should not compile");

    let message = format!("{error:#}");
    assert!(message.contains("needs `net`"), "{message}");
    assert!(message.contains("{ fs }"), "{message}");
}

#[test]
fn a_row_that_covers_the_body_compiles() {
    UshCompiler
        .compile_source(
            "fn fetch(url: String) -> () / { net, io } {\n  print std::http::get(url)\n}\nfetch(\"u\")\n",
        )
        .expect("declared effects cover the body");
}

#[test]
fn declaring_more_than_the_body_uses_is_allowed() {
    // A row is an upper bound, so a signature can stay stable while
    // an implementation stops needing something.
    UshCompiler
        .compile_source("fn slug(a: String) -> String / { net } {\n  a\n}\nprint slug(\"x\")\n")
        .expect("over-declaration is not an error");
}

#[test]
fn an_empty_row_rejects_any_effect_at_all() {
    let error = UshCompiler
        .compile_source("fn shout(a: String) -> () / {} {\n  print a\n}\nshout(\"x\")\n")
        .expect_err("`/ {}` should reject `print`");

    assert!(format!("{error:#}").contains("needs `io`"));
}

#[test]
fn an_unknown_effect_name_is_reported_with_the_known_ones() {
    let error = UshCompiler
        .compile_source("fn f() -> String / { nope } {\n  \"x\"\n}\nprint f()\n")
        .expect_err("unknown effect");

    let message = format!("{error:#}");
    assert!(message.contains("unknown effect `nope`"), "{message}");
    assert!(message.contains("effect nope"), "{message}");
}

// --- user effects, `do`, and handlers ---------------------------------

const LOGGED: &str = concat!(
    "effect log(message: String) -> ()\n",
    "fn greet(name: String) -> String / { log } {\n",
    "  do log(\"greeting \" + name)\n",
    "  \"hello \" + name\n",
    "}\n",
);

#[test]
fn performing_a_declared_effect_puts_it_in_the_row() {
    let source = alloc::format!(
        "{LOGGED}try {{\n  print greet(\"x\")\n}} with log {{ (message) =>\n  print message\n}}\n"
    );
    let effects = effects_of(&source, "greet");

    assert!(effects.contains_user("log"), "{effects}");
    assert_eq!(effects.to_string(), "log");
}

#[test]
fn a_handler_discharges_the_effect_for_the_block_it_wraps() {
    let source = alloc::format!(
        "{LOGGED}try {{\n  print greet(\"x\")\n}} with log {{ (message) =>\n  print message\n}}\n"
    );

    UshCompiler
        .compile_source(&source)
        .expect("a handled effect compiles");
}

#[test]
fn an_effect_that_reaches_the_top_of_the_program_is_rejected() {
    let error = UshCompiler
        .compile_source(&alloc::format!("{LOGGED}print greet(\"x\")\n"))
        .expect_err("an unhandled effect should not compile");

    let message = format!("{error:#}");
    assert!(message.contains("never handled"), "{message}");
    assert!(message.contains("try {"), "{message}");
}

#[test]
fn a_handler_body_contributes_its_own_effects() {
    // Handling `log` by printing swaps one effect for another: the
    // row loses `log` and gains `io`.
    let source = alloc::format!(
        "{LOGGED}fn run() -> () / {{ io }} {{\n  try {{\n    print greet(\"x\")\n  }} with log {{ (message) =>\n    print message\n  }}\n}}\nrun()\n"
    );

    let effects = effects_of(&source, "run");
    assert!(!effects.contains_user("log"), "{effects}");
    assert!(effects.contains(Effect::Io), "{effects}");
}

#[test]
fn a_row_naming_an_undeclared_effect_is_rejected() {
    let error = UshCompiler
        .compile_source("fn f() -> String / { log } {\n  \"x\"\n}\nprint f()\n")
        .expect_err("undeclared effect in a row");

    assert!(format!("{error:#}").contains("unknown effect `log`"));
}

#[test]
fn a_builtin_effect_cannot_be_declared() {
    let error = UshCompiler
        .compile_source("effect io(message: String) -> ()\nprint \"x\"\n")
        .expect_err("`io` is built in");

    assert!(format!("{error:#}").contains("built-in effect"));
}

#[test]
fn a_row_renders_the_way_it_is_written() {
    let set = [Effect::Net, Effect::Io, Effect::Fs]
        .into_iter()
        .collect::<EffectSet>();

    assert_eq!(set.to_string(), "io, fs, net");
    assert_eq!(set.render_row(), "{ io, fs, net }");
    assert_eq!(EffectSet::empty().to_string(), "pure");
    assert_eq!(EffectSet::empty().render_row(), "{}");
}

#[test]
fn difference_reports_only_what_is_missing() {
    let inferred = [Effect::Net, Effect::Fs].into_iter().collect::<EffectSet>();
    let declared = EffectSet::of(Effect::Fs);

    assert_eq!(inferred.difference(&declared), EffectSet::of(Effect::Net));
    assert!(declared.difference(&inferred).is_empty());
}
