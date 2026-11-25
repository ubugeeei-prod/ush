mod cli;
mod effects_report;
mod explain;
mod panic_hook;
mod script_docs;
mod sourcemap_json;
mod test_runner;

use std::path::Path;
use std::process;

use anyhow::{Context, Result};

use ush_compiler::UshCompiler;
use ush_config::UshConfig;
use ush_shell::{SessionStartup, Shell, ShellOptions, run_posix_script};
use ush_tooling::{check_file, format_source};

use crate::cli::{Action, Cli};
use crate::sourcemap_json::write_sourcemap_file;

enum ScriptMode {
    Ush,
    Posix,
}

fn main() -> Result<()> {
    panic_hook::install();

    if script_docs::handle_raw_doc_request()? {
        return Ok(());
    }

    let cli = Cli::parse_argv();
    let config = UshConfig::load(cli.config.as_deref())?;

    if let Some(action) = &cli.action {
        match action {
            Action::Compile {
                target,
                input,
                output,
                sourcemap,
                checked_in,
            } => {
                compile_action(
                    input,
                    output.as_deref(),
                    sourcemap.as_deref(),
                    *checked_in,
                    *target,
                )?;
                return Ok(());
            }
            Action::Format {
                input,
                check,
                stdout,
            } => process::exit(format_action(input, *check, *stdout)?),
            Action::Check { input } => process::exit(check_action(input)?),
            Action::Effects { input, undeclared } => {
                process::exit(effects_report::report(input, *undeclared)?)
            }
            Action::Explain { input, lines } => {
                process::exit(explain::explain(input, lines, cli.target)?)
            }
            Action::Test { targets } => process::exit(test_runner::run(
                targets,
                cli.config.as_deref(),
                cli.target,
            )?),
        }
    }

    let options = ShellOptions::resolve(
        cli.stylish,
        cli.plain,
        cli.no_interaction,
        cli.print_ast,
        &config,
    );
    let startup = session_startup(&cli);

    if let Some(script) = &cli.script {
        let status = match script_mode(script) {
            ScriptMode::Ush => {
                if script_docs::handle_script_doc_request(script, &cli.script_args)? {
                    return Ok(());
                }
                let compiled = UshCompiler::for_target(cli.target).compile_file(script)?;
                let mut shell = Shell::new(config, options)?;
                shell.load_session_startup(&startup)?;
                shell.run_compiled_script_with(
                    script,
                    &compiled,
                    &cli.script_args,
                    cli.target.name(),
                )?
            }
            ScriptMode::Posix => run_posix_script(script, &cli.script_args, &options)?,
        };
        process::exit(status);
    }

    let mut shell = Shell::new(config, options)?;
    shell.load_session_startup(&startup)?;
    if let Some(command) = &cli.command {
        process::exit(shell.execute(command)?);
    }

    shell.run_repl()?;
    Ok(())
}

fn compile_action(
    input: &Path,
    output: Option<&Path>,
    sourcemap: Option<&Path>,
    checked_in: bool,
    target: ush_compiler::ShellTarget,
) -> Result<()> {
    let compiler = UshCompiler::for_target(target);
    let artifact = if checked_in {
        Some(output.expect("clap requires output with --checked-in"))
    } else {
        None
    };
    let shell = if let Some(map) = sourcemap {
        let compiled = match artifact {
            Some(path) => compiler.compile_checked_in_file_with_sourcemap(input, path)?,
            None => compiler.compile_file_with_sourcemap(input)?,
        };
        write_sourcemap_file(map, input, output, &compiled.sourcemap)?;
        compiled.shell
    } else if let Some(path) = artifact {
        compiler.compile_checked_in_file(input, path)?
    } else {
        compiler.compile_file(input)?
    };
    if let Some(output) = output {
        std::fs::write(output, shell)
            .with_context(|| format!("failed to write {}", output.display()))?;
    } else {
        print!("{shell}");
    }
    Ok(())
}

fn format_action(input: &std::path::Path, check: bool, stdout: bool) -> Result<i32> {
    let source = std::fs::read_to_string(input)
        .with_context(|| format!("failed to read {}", input.display()))?;
    let formatted = format_source(&source);
    if check {
        if source == formatted {
            return Ok(0);
        }
        eprintln!("ush: formatting required: {}", input.display());
        return Ok(1);
    }
    if stdout {
        print!("{formatted}");
        return Ok(0);
    }
    std::fs::write(input, formatted)
        .with_context(|| format!("failed to write {}", input.display()))?;
    Ok(0)
}

fn check_action(input: &std::path::Path) -> Result<i32> {
    let diagnostics = check_file(input)?;
    if diagnostics.is_empty() {
        return Ok(0);
    }
    for diagnostic in diagnostics {
        eprintln!(
            "{}:{}: {}",
            input.display(),
            diagnostic.line + 1,
            diagnostic.message
        );
    }
    Ok(1)
}

fn script_mode(path: &Path) -> ScriptMode {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("ush") => ScriptMode::Ush,
        Some("sh") => ScriptMode::Posix,
        _ => ScriptMode::Posix,
    }
}

fn session_startup(cli: &Cli) -> SessionStartup {
    SessionStartup {
        load_env: !cli.no_env || cli.env_file.is_some(),
        env_file: cli.env_file.clone(),
        load_profile: (cli.login && !cli.no_profile) || cli.profile_file.is_some(),
        load_system_profile: cli.login && !cli.no_profile,
        load_rc: (cli.is_interactive() && !cli.no_rc) || cli.rc_file.is_some(),
        profile_file: cli.profile_file.clone(),
        rc_file: cli.rc_file.clone(),
    }
}
