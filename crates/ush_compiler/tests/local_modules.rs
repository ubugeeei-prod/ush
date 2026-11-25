//! `mod "path.ush" as name`: linking, namespaces and visibility,
//! checked by running the generated script.

mod support;

use support::modules::Project;

#[test]
fn modules_with_the_same_function_name_do_not_collide() {
    let project = Project::new(&[
        ("a.ush", "pub fn name() -> String {\n  \"a\"\n}\n"),
        ("b.ush", "pub fn name() -> String {\n  \"b\"\n}\n"),
        (
            "main.ush",
            "mod \"a.ush\"\nmod \"b.ush\"\nfn name() -> String {\n  \"main\"\n}\nprint a::name\nprint b::name\nprint name\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "a\nb\nmain\n");
}

#[test]
fn use_imports_and_wildcards_bring_in_public_items() {
    let project = Project::new(&[
        (
            "lib/text.ush",
            "pub fn shout(text: String) -> String {\n  text + \"!\"\n}\npub fn whisper(text: String) -> String {\n  \"(\" + text + \")\"\n}\nfn hidden() -> String {\n  \"no\"\n}\n",
        ),
        (
            "explicit.ush",
            "mod \"lib/text.ush\" as t\nuse t::{shout, whisper as quiet}\nprint shout \"hi\"\nprint quiet \"hi\"\n",
        ),
        (
            "wildcard.ush",
            "mod \"lib/text.ush\"\nuse text::*\nprint shout (whisper \"hi\")\n",
        ),
        (
            "hidden.ush",
            "mod \"lib/text.ush\"\nuse text::*\nprint hidden\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("explicit.ush"), "hi!\n(hi)\n");
    assert_eq!(project.run_everywhere("wildcard.ush"), "(hi)!\n");
    assert!(project.error("hidden.ush").contains("unknown"));
}

#[test]
fn private_items_cannot_be_reached_from_outside() {
    let project = Project::new(&[
        ("lib/gh.ush", "fn token() -> String {\n  \"secret\"\n}\n"),
        ("qualified.ush", "mod \"lib/gh.ush\"\nprint gh::token\n"),
        ("imported.ush", "mod \"lib/gh.ush\"\nuse gh::{token}\n"),
        ("missing.ush", "mod \"lib/gh.ush\"\nuse gh::{nothing}\n"),
    ]);
    for script in ["qualified.ush", "imported.ush"] {
        let error = project.error(script);
        assert!(
            error.contains("line 2: `gh::token` is private to lib/gh.ush; mark it `pub`"),
            "{error}"
        );
    }
    assert!(
        project
            .error("missing.ush")
            .contains("module `gh` (lib/gh.ush) has no item `nothing`")
    );
}

#[test]
fn nested_and_shared_modules_are_linked_once() {
    let project = Project::new(&[
        (
            "lib/util.ush",
            "pub fn tag(text: String) -> String {\n  \"[\" + text + \"]\"\n}\n",
        ),
        (
            "lib/a.ush",
            "mod \"util.ush\"\npub fn a() -> String {\n  util::tag \"a\"\n}\n",
        ),
        (
            "lib/b.ush",
            "mod \"util.ush\" as u\npub fn b() -> String {\n  u::tag \"b\"\n}\n",
        ),
        (
            "main.ush",
            "mod \"lib/a.ush\"\nmod \"lib/b.ush\"\nprint a::a + b::b\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "[a][b]\n");
    let shell = project.compile("main.ush").unwrap();
    assert_eq!(shell.matches("ush_fn_util__tag() {").count(), 1);
}

#[test]
fn only_reachable_module_functions_are_emitted() {
    let project = Project::new(&[
        (
            "lib.ush",
            "pub fn used() -> String {\n  helper\n}\nfn helper() -> String {\n  \"used\"\n}\npub fn unused() -> String {\n  \"unused\"\n}\ntest \"lib\" {\n  print unused\n}\n",
        ),
        ("main.ush", "mod \"lib.ush\"\nprint lib::used\n"),
    ]);
    let shell = project.compile("main.ush").unwrap();
    assert!(shell.contains("ush_fn_lib__helper() {"));
    assert!(!shell.contains("ush_fn_lib__unused"));
    assert!(!shell.contains("__ush_test_"));
}

#[test]
fn types_enums_and_effects_cross_module_boundaries() {
    let project = Project::new(&[
        (
            "lib/model.ush",
            "pub enum Level {\n  Info,\n  Warn,\n}\npub type Entry {\n  text: String,\n}\npub effect log(message: String) -> ()\npub fn render(level: Level, entry: Entry) -> String {\n  match level {\n    Level::Info => \"info: \" + entry.text,\n    Level::Warn => \"warn: \" + entry.text,\n  }\n}\npub fn note(text: String) -> () / log {\n  do log text\n}\n",
        ),
        (
            "main.ush",
            "mod \"lib/model.ush\"\nuse model::{Level, log, note}\nenum Mode {\n  Local,\n}\nlet first = model::Entry { text: \"disk\" }\nprint model::render Level::Warn first\ntry {\n  note \"hello\"\n} with log(message) {\n  print \"logged \" + message\n}\n",
        ),
        (
            "clash.ush",
            "mod \"lib/model.ush\"\nuse model::{Level}\nenum Level {\n  Local,\n}\n",
        ),
    ]);
    assert_eq!(
        project.run_everywhere("main.ush"),
        "warn: disk\nlogged hello\n"
    );
    let error = project.error("clash.ush");
    assert!(
        error.contains("line 3: `Level` is both declared here and imported"),
        "{error}"
    );
}
