mod convert;
mod document;
mod paths;
mod server;

use anyhow::Result;
use lsp_server::Connection;

const HELP: &str = "\
ush_lsp — Language Server Protocol server for .ush files

Usage:
  ush_lsp                  speak LSP over stdio (default)
  ush_lsp --version | -V   print version and exit
  ush_lsp --help    | -h   print this help and exit
";

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    if let Some(flag) = args.next() {
        match flag.as_str() {
            "--version" | "-V" => {
                println!("ush_lsp {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(());
            }
            other => {
                eprintln!("ush_lsp: unknown argument: {other}");
                eprint!("{HELP}");
                std::process::exit(2);
            }
        }
    }

    let (connection, io_threads) = Connection::stdio();
    server::run(connection)?;
    io_threads.join()?;
    Ok(())
}
