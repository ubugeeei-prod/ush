//! A signature without `/ row` gets an inferred row; a written row stays
//! a checked contract.

mod support;

use support::modules::Project;
use ush_compiler::{EffectReport, UshCompiler};

fn report(source: &str) -> EffectReport {
    UshCompiler
        .effects_source(source)
        .unwrap_or_else(|error| panic!("{error:#}"))
}

/// `name`'s row as `ush effects` prints it.
fn row(report: &EffectReport, name: &str) -> String {
    report
        .functions
        .iter()
        .find(|function| function.name == name)
        .unwrap_or_else(|| panic!("no function {name}"))
        .row()
}

fn compile_error(source: &str) -> String {
    format!("{:#}", UshCompiler.compile_source(source).unwrap_err())
}

const CONFIG: &str = "enum ConfigError {\n  Missing,\n}\n";

#[test]
fn commands_and_typed_failures_are_inferred() {
    let source = format!(
        "use $bin::*\n{CONFIG}fn head_sha() -> String {{\n  git \"rev-parse\" \"HEAD\"\n}}\nfn load(path: String) -> String {{\n  if path == \"\" {{\n    fail ConfigError::Missing\n  }}\n  path\n}}\nfn both(path: String) -> String {{\n  let found = load path?\n  found + head_sha\n}}\nfn summary(text: String) -> () {{\n  print text\n}}\n"
    );
    let report = report(&source);
    assert_eq!(
        row(&report, "head_sha"),
        "unknown, stdin, stderr, fs, env, net, exec"
    );
    assert_eq!(row(&report, "load"), "ConfigError");
    assert_eq!(
        row(&report, "both"),
        "ConfigError, unknown, stdin, stderr, fs, env, net, exec"
    );
    assert_eq!(row(&report, "summary"), "stdout");
    assert!(UshCompiler.compile_source(&source).is_ok());
}

#[test]
fn mutually_recursive_functions_share_the_least_row() {
    let report = report(&format!(
        "{CONFIG}fn ping(s: String) -> String {{\n  if s == \"stop\" {{\n    return \"done\"\n  }}\n  pong \"stop\"\n}}\nfn pong(s: String) -> String {{\n  if s == \"\" {{\n    fail ConfigError::Missing\n  }}\n  ping s\n}}\nfn quiet(s: String) -> String {{\n  quiet s\n}}\n"
    ));
    assert_eq!(row(&report, "ping"), "ConfigError");
    assert_eq!(row(&report, "pong"), "ConfigError");
    assert_eq!(row(&report, "quiet"), "pure");
}

#[test]
fn callbacks_bring_their_rows_to_the_caller() {
    let source = format!(
        "{CONFIG}fn check(name: String) -> String {{\n  if name == \"\" {{\n    fail ConfigError::Missing\n  }}\n  name\n}}\nfn map<A, B>(items: List<A>, f: Fn<A, B>) -> List<B> {{\n  [f x for x in items]\n}}\nfn all() -> List<String> {{\n  map [\"a\"] &check\n}}\nfn loud(name: String) -> String {{\n  print name\n  name\n}}\nfn sealed() -> List<String> / () {{\n  map [\"a\"] &loud\n}}\n"
    );
    let error = compile_error(&source);
    assert!(error.contains("`sealed` needs `stdout`"), "{error}");
    let report = report(&source.replace(" / () {\n  map [\"a\"] &loud", " {\n  map [\"a\"] &loud"));
    assert_eq!(row(&report, "all"), "ConfigError");
    assert_eq!(row(&report, "sealed"), "stdout");
}

#[test]
fn written_rows_stay_checked_contracts() {
    let error =
        compile_error("use $bin::*\nfn sha() -> String / () {\n  git \"rev-parse\" \"HEAD\"\n}\n");
    assert!(
        error.contains("function `sha` declares `String / ()` but can raise `unknown`"),
        "{error}"
    );
    let error = compile_error(&format!(
        "{CONFIG}fn load() -> String / stdout {{\n  fail ConfigError::Missing\n}}\n"
    ));
    assert!(error.contains("but can raise `ConfigError`"), "{error}");
    let report = report("fn wide() -> () / unknown, stdout {\n  print \"x\"\n}\n");
    let wide = &report.functions[0];
    assert_eq!(wide.row(), "stdout");
    assert_eq!(wide.declared_row().as_deref(), Some("unknown, stdout"));
}

#[test]
fn inferred_failures_propagate_with_question_marks() {
    let (status, stdout, stderr) = support::try_run(&format!(
        "{CONFIG}fn load(path: String) -> String {{\n  if path == \"\" {{\n    fail ConfigError::Missing\n  }}\n  path\n}}\nfn twice(path: String) -> String {{\n  let first = load path?\n  first + first\n}}\nprint twice \"ab\"\nprint twice \"\"?\nprint \"unreachable\"\n"
    ));
    assert_eq!(status, 1);
    assert_eq!(stdout, "abab\n");
    assert!(stderr.contains("ConfigError::Missing"), "{stderr}");
}

#[test]
fn public_module_functions_infer_their_rows() {
    let project = Project::new(&[
        (
            "lib/github.ush",
            "use $bin::*\n\npub fn summary(text: String) -> () {\n  print \"## \" + text\n}\n\npub fn head_sha() -> String {\n  git \"rev-parse\" \"--is-inside-work-tree\"\n}\n\npub fn label(name: String) -> String {\n  if name == \"\" {\n    fail 2 \"empty label\"\n  }\n  name\n}\n",
        ),
        (
            "main.ush",
            "mod \"lib/github.ush\"\ngithub::summary (github::label \"report\"?)\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "## report\n");
    let report = UshCompiler
        .effects_file(&project.root.join("main.ush"))
        .unwrap();
    assert_eq!(row(&report, "github::label"), "unknown, stderr");
    assert_eq!(row(&report, "github::summary"), "stdout");
}

#[test]
fn written_rows_may_name_imported_error_types() {
    let project = Project::new(&[
        (
            "cfg.ush",
            "pub enum ConfigError {\n  Missing,\n}\npub fn load(path: String) -> String / ConfigError {\n  if path == \"\" {\n    fail ConfigError::Missing\n  }\n  path\n}\n",
        ),
        (
            "main.ush",
            "mod \"cfg.ush\"\nuse cfg::{ConfigError}\nfn wrap(path: String) -> String / ConfigError {\n  let value = cfg::load path?\n  value\n}\nprint wrap \"ok\"\n",
        ),
    ]);
    assert_eq!(project.run_everywhere("main.ush"), "ok\n");
    let report = UshCompiler
        .effects_file(&project.root.join("main.ush"))
        .unwrap();
    assert_eq!(row(&report, "wrap"), "cfg::ConfigError");
}

#[test]
fn assignments_contribute_to_the_inferred_row() {
    let source = format!(
        "use $bin::*\n{CONFIG}fn load(path: String) -> String {{\n  if path == \"\" {{\n    fail ConfigError::Missing\n  }}\n  path\n}}\nfn collect(paths: List<String>) -> String {{\n  let mut joined = \"\"\n  for path in paths {{\n    joined = joined + load path?\n  }}\n  joined\n}}\nfn stamp() -> String {{\n  let mut value = \"none\"\n  value = uname \"-s\"\n  value\n}}\n"
    );
    let report = report(&source);
    assert_eq!(row(&report, "collect"), "ConfigError");
    assert!(
        row(&report, "stamp").starts_with("unknown, "),
        "{}",
        row(&report, "stamp")
    );
    assert!(row(&report, "stamp").ends_with("exec"));
    assert_eq!(
        support::run(&format!("{source}print collect [\"a\", \"b\"]\n")),
        "ab\n"
    );
}
