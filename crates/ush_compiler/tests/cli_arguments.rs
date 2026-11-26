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
fn native_and_external_calls_accept_cli_words() {
    outputs(
        r#"
use $bin::*

fn greet(name: String) -> String {
  "hello " + name
}
let name = "two words"
echo Hello world
echo (greet $name)
echo (greet ush)
mkdir -p ./dist
printf "%s\n" ./dist --flag=value https://example.test/a?q=b+c a*b file?
echo -n -e a+b ./a..b
"#,
        "Hello world\nhello two words\nhello ush\n./dist\n--flag=value\nhttps://example.test/a?q=b+c\na*b\nfile?\n-n -e a+b ./a..b\n",
    );
}

#[test]
fn variable_expansion_never_splits_evaluates_or_globs() {
    outputs(
        r#"
use $bin::*

let name = "two words * $(touch injected); '"
let empty = ""
let count = 2
let enabled = true
printf "<%s>\n" $name $empty --output=${name}/file item-$count enabled-$enabled
echo (count + 1)
echo name
echo "literal $name"
"#,
        "<two words * $(touch injected); '>\n<>\n<--output=two words * $(touch injected); '/file>\n<item-2>\n<enabled-true>\n3\nname\nliteral $name\n",
    );
}

#[test]
fn captures_pipelines_and_named_arguments_keep_their_types() {
    outputs(
        r#"
use $bin::*

fn repeat(text: String, count: Int) -> String {
  std::string::repeat $text $count
}
let count = 2
let text = repeat text: hi count: $count
echo $text | tr a-z A-Z
let captured = echo bare words
echo $captured
let a = "a"
echo (repeat($a, 2))
"#,
        "HIHI\nbare words\naa\n",
    );
}

#[test]
fn undefined_explicit_variables_are_errors_and_words_are_stable() {
    outputs(
        "echo name\nlet name = \"value\"\necho name $name\n",
        "name\nname value\n",
    );
    for (source, message) in [
        ("echo $missing\n", "unknown variable: missing"),
        (
            "fn name() -> String {\n \"function\"\n}\necho $name\n",
            "unknown variable: name",
        ),
        ("echo ${missing}\n", "unknown variable: missing"),
        ("echo $\n", "invalid variable reference"),
        ("echo ${name\n", "unclosed variable reference"),
        (
            "fn add(value: Int) -> Int {\n value + 1\n}\nadd typo\n",
            "expected Int",
        ),
    ] {
        let error = UshCompiler.compile_source(source).unwrap_err();
        assert!(
            format!("{error:#}").contains(message),
            "{source}: {error:#}"
        );
    }
}

#[test]
fn echo_streams_when_process_control_observes_its_status() {
    outputs(
        r#"
fn report() -> Int / stdout {
  status_of echo status
}
let code = report
echo $code
allow_fail echo allowed
if echo condition {
  echo yes
}
let job = spawn echo background
job.await
defer echo deferred
"#,
        "status\n0\nallowed\ncondition\nyes\nbackground\ndeferred\n",
    );
}

#[test]
fn a_backslash_after_a_space_continues_the_command() {
    outputs(
        r#"
use $bin::*

let name = "two words"
printf "<%s>\n" first \
    # a comment between words is skipped
    $name \

  --flag=value |
  tr a-z A-Z
printf "<%s>\n" "ends\\" \
  'raw\' trailing\
echo done
"#,
        "<FIRST>\n<TWO WORDS>\n<--FLAG=VALUE>\n<ends\\>\n<raw\\>\n<trailing\\>\ndone\n",
    );
}

#[test]
fn a_command_cannot_continue_past_the_end_of_the_file() {
    let error = UshCompiler::default()
        .compile_source("use $bin::*\necho a \\\n")
        .expect_err("a trailing backslash needs a next line");
    assert!(
        format!("{error:#}").contains("line 2: the command continues past the end of the file"),
        "{error:#}"
    );
}

#[test]
fn a_list_argument_gives_each_element_its_own_word() {
    outputs(
        r#"
use $bin::*

let files = ["two words", "star*", ""]
let counts = [1, 2]
printf "<%s>\n" first $files middle $counts
let joined = printf "%s," $files | tr a-z A-Z
echo $joined
if printf "%s" $files {
  echo listed
}
"#,
        "<first>\n<two words>\n<star*>\n<>\n<middle>\n<1>\n<2>\nTWO WORDS,STAR*,,\ntwo wordsstar*listed\n",
    );
}

#[test]
fn a_test_predicate_takes_single_words() {
    let error = UshCompiler::default()
        .compile_source("use $bin::*\nlet xs = [\"a\"]\nif test -n $xs {\n  echo x\n}\n")
        .expect_err("a predicate cannot spread a list");
    assert!(
        format!("{error:#}").contains("`test` takes single words, not a List"),
        "{error:#}"
    );
}

#[test]
fn a_word_that_reads_as_a_quantity_reaches_a_program_as_written() {
    // `timeout 300s …`: a quantity has a meaning only for a ush parameter
    // of that unit, never for a program's argument.
    outputs(
        r#"
use $bin::*

printf "<%s>\n" 300s 5m 10ms
"#,
        "<300s>\n<5m>\n<10ms>\n",
    );
}
