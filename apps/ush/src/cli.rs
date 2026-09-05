use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ush", version, about = "ush (ubugeeei shell)")]
pub struct Cli {
    /// Force the stylish (human-formatted) output renderer.
    #[arg(short = 's', long = "stylish")]
    pub stylish: bool,

    /// Force plain (POSIX-friendly) output, overriding stylish.
    #[arg(long = "plain")]
    pub plain: bool,

    /// Suppress interactive prompts (the same as USH_INTERACTION=false).
    #[arg(long = "no-interaction")]
    pub no_interaction: bool,

    /// Path to the ush config file. Defaults to ~/.config/ush/config.toml.
    #[arg(long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Treat this invocation as a login shell (loads the profile file).
    ///
    /// A leading `-` in `argv[0]` — how terminal emulators mark a
    /// login shell — has the same effect, see `Cli::parse_argv`.
    #[arg(short = 'l', long = "login")]
    pub login: bool,

    /// Force interactive behaviour (loads the rc file).
    ///
    /// Accepted for POSIX compatibility. Editor terminals and agent
    /// runners spawn `$SHELL -i -c ...`, and rejecting `-i` made
    /// every such session fail before it started.
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Skip the env file (~/.config/ush/env.sh etc.).
    #[arg(long = "no-env", alias = "noenv", conflicts_with = "env_file")]
    pub no_env: bool,

    /// Use FILE as the env file instead of the default lookup.
    #[arg(long = "env-file", value_name = "FILE", conflicts_with = "no_env")]
    pub env_file: Option<PathBuf>,

    /// Skip the profile file even in login mode.
    #[arg(
        long = "no-profile",
        alias = "noprofile",
        conflicts_with = "profile_file"
    )]
    pub no_profile: bool,

    /// Use FILE as the profile file instead of the default lookup.
    #[arg(
        long = "profile-file",
        value_name = "FILE",
        conflicts_with = "no_profile"
    )]
    pub profile_file: Option<PathBuf>,

    /// Skip the rc file (~/.config/ush/.config.ush etc.).
    #[arg(long = "no-rc", alias = "norc", conflicts_with = "rc_file")]
    pub no_rc: bool,

    /// Use FILE as the rc file instead of the default lookup.
    #[arg(long = "rc-file", value_name = "FILE", conflicts_with = "no_rc")]
    pub rc_file: Option<PathBuf>,

    /// Run COMMAND non-interactively and exit (POSIX `sh -c`).
    #[arg(short = 'c', value_name = "COMMAND")]
    pub command: Option<String>,

    /// Print the parsed `.ush` AST and exit (debugging aid).
    #[arg(long = "print-ast")]
    pub print_ast: bool,

    #[command(subcommand)]
    pub action: Option<Action>,

    /// `.ush` or `.sh` script to execute (positional).
    #[arg(value_name = "SCRIPT")]
    pub script: Option<PathBuf>,

    /// Arguments forwarded to the script as `$1`, `$2`, ….
    #[arg(
        value_name = "ARGS",
        allow_hyphen_values = true,
        trailing_var_arg = true
    )]
    pub script_args: Vec<String>,
}

impl Cli {
    /// Parses the process arguments, honouring the POSIX convention
    /// that a login shell is launched with a `-` in front of
    /// `argv[0]`.
    ///
    /// Ghostty, Terminal.app, and most editor terminals start a login
    /// shell that way rather than by passing `-l`. Ignoring it meant
    /// the profile never ran, which is why a `PATH` set up in
    /// `~/.ush_profile` looked like it had been thrown away.
    pub fn parse_argv() -> Self {
        let argv0_is_login = std::env::args_os()
            .next()
            .map(|arg| arg.to_string_lossy().starts_with('-'))
            .unwrap_or(false);

        let mut cli = Self::parse();
        cli.login |= argv0_is_login;
        cli
    }

    /// True when this session should behave like an interactive
    /// shell: an explicit `-i`, or a bare `ush` that drops into the
    /// REPL.
    pub fn is_interactive(&self) -> bool {
        self.interactive
            || (self.action.is_none() && self.script.is_none() && self.command.is_none())
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    /// Lower a `.ush` source file to POSIX `sh`.
    Compile {
        input: PathBuf,
        /// Write generated `sh` to FILE instead of stdout.
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
        /// Also emit a sourcemap to FILE.
        #[arg(long = "sourcemap", value_name = "FILE")]
        sourcemap: Option<PathBuf>,
    },
    /// Format a `.ush` source file in-place (or print the result).
    Format {
        input: PathBuf,
        /// Exit non-zero if the file is not already formatted.
        #[arg(long)]
        check: bool,
        /// Print the formatted file to stdout instead of writing back.
        #[arg(long)]
        stdout: bool,
    },
    /// Type-check a `.ush` source file without producing output.
    Check { input: PathBuf },
    /// Map a generated-shell line back to the `.ush` code behind it.
    ///
    /// Takes the line numbers `/bin/sh` prints in its own errors, or
    /// the `G####` ids from a sourcemap listing.
    Explain {
        input: PathBuf,
        #[arg(value_name = "LINE", required = true)]
        lines: Vec<String>,
    },
    /// Run inline `#[test]` blocks in one or more `.ush` files.
    Test {
        #[arg(value_name = "TARGET")]
        targets: Vec<String>,
    },
}
