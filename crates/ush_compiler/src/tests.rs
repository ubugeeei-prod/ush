use super::UshCompiler;

#[test]
fn compiles_basic_program() {
    let compiler = UshCompiler;
    let output = compiler
        .compile_source(
            r#"
            let greeting = "hello"
            print greeting + " world"
            $ printf '%s\n' hi
        "#,
        )
        .expect("compile");

    assert!(output.contains("greeting='hello'"));
    assert!(output.contains("printf '%s\\n'"));
    assert!(output.contains("printf '%s\\n' hi"));
}

#[test]
fn concat_is_shell_quoted() {
    let compiler = UshCompiler;
    let output = compiler
        .compile_source(
            r#"
            let greeting = "hello"
            print greeting + " world"
        "#,
        )
        .expect("compile");

    assert!(output.contains("\"$(printf '%s' \"${greeting}\" ' world')\""));
}

#[test]
fn enum_is_lowered_to_tagged_shell_bindings() {
    let compiler = UshCompiler;
    let output = compiler
        .compile_source(
            r#"
            enum Option {
              None,
              Some(String),
            }
            let value = Option::Some("hello")
        "#,
        )
        .expect("compile");

    assert!(output.contains("value__tag='Option::Some'"));
    assert!(output.contains("value__0='hello'"));
}

#[test]
fn async_await_lowers_to_task_handle_and_result_read() {
    let compiler = UshCompiler;
    let output = compiler
        .compile_source(
            r#"
            fn worker(message: String) -> String {
              return message
            }
            let task = async worker "ok"
            let result = task.await
            print result
        "#,
        )
        .expect("compile");

    assert!(output.starts_with("#!/bin/sh\nset -eu\n"));
    assert!(output.contains("ush_fn_worker()"));
    assert!(output.contains("__ush_task_seq='0'"));
    assert!(
        output.contains("__ush_task_0__result=\"${TMPDIR:-/tmp}/__ush_task_0.$$.$__ush_task_seq\"")
    );
    assert!(
        output.contains("( __ush_return_path=\"${__ush_task_0__result}\"; ush_fn_worker 'ok' ) &")
    );
    assert!(output.contains("result=\"$(cat \"${__ush_task_0__result}\")\""));
    assert!(output.contains("rm -f \"$__ush_task_file\""));
    assert!(!output.contains("mktemp"));
    assert!(output.contains("wait \"$__ush_job\""));
}

#[test]
fn functional_calls_capture_sync_return_values() {
    let compiler = UshCompiler;
    let output = compiler
        .compile_source(
            r#"
            fn greet(message: String) -> String {
              return "<" + message + ">"
            }
            let value = greet "ush"
            print $ greet (value)
        "#,
        )
        .expect("compile");

    assert!(output.contains("__ush_value_0=\"$(__ush_capture_return='1' ush_fn_greet 'ush')\""));
    assert!(output.contains("value=\"${__ush_value_0}\""));
    assert!(
        output.contains("__ush_value_1=\"$(__ush_capture_return='1' ush_fn_greet \"${value}\")\"")
    );
    assert!(output.contains("printf '%s\\n' \"${__ush_value_1}\""));
}
