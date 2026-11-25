mod support;

use ush_compiler::UshCompiler;

fn outputs(source: &str, expected: &str) {
    support::each_program(source, |shell, _, (status, stdout, stderr)| {
        assert_eq!(status, 0, "{shell}: {stderr}");
        assert_eq!(stdout, expected, "{shell}");
        assert!(stderr.is_empty(), "{shell}: {stderr}");
    });
}

#[test]
fn portable_literals_and_positional_values() {
    outputs(
        r#"
echo
echo "hello" "two words" "" 42 true ()
echo "-n" "-e" "\\c" "%s" "$(touch injected)" "*"
print "legacy"
"#,
        "\nhello two words  42 true \n-n -e \\c %s $(touch injected) *\nlegacy\n",
    );
}

#[test]
fn capture_and_pipelines_preserve_stdout() {
    outputs(
        r#"
use $bin::*

let text = echo "hello" "world"
echo $text
echo "hello" | tr "a-z" "A-Z"
echo | wc "-l" | tr "-d" " "
fn greet() -> () {
    echo "from function"
}
greet
"#,
        "hello world\nHELLO\n1\nfrom function\n",
    );
}

#[test]
fn display_values_and_evaluation_order() {
    outputs(
        r#"
enum Label {
    Value(String)
}
impl Display for Label {
    fn fmt(self) -> String {
        match self {
            Label::Value(text) => text
        }
    }
}
fn value(text: String) -> String {
    print text
    text
}
let label = Label::Value("display")
echo $label (value "first") (value "second")
let text = echo $label 7
echo $text
"#,
        "first\nsecond\ndisplay first second\ndisplay 7\n",
    );
}

#[test]
fn argument_errors_are_not_silently_discarded() {
    let (status, stdout, stderr) =
        support::try_run("fn bad() -> String {\n    fail 7 \"bad argument\"\n}\necho (bad)\n");
    assert_eq!(status, 7, "{stderr}");
    assert!(stdout.is_empty(), "{stdout}");
    for (source, message) in [
        ("echo $missing\n", "unknown variable"),
        ("echo text: \"hello\"\n", "positional"),
        ("echo [1, 2]\n", "Display"),
        ("fn quiet() -> () / () {\n echo \"no\"\n}\n", "stdout"),
    ] {
        let error = UshCompiler::default().compile_source(source).unwrap_err();
        assert!(format!("{error:#}").contains(message), "{error:#}");
    }
}
