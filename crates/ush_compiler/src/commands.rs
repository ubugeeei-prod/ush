//! The typed process boundary. A command receives argv words directly.
use crate::{
    ast::Type,
    effects::{Effect, EffectSet},
};
use anyhow::{Result, bail};

#[derive(Clone, Copy)]
pub(crate) struct CommandSpec<'a> {
    pub word: &'a str,
    pub min: usize,
    pub max: Option<usize>,
    pub output: Output,
    pub effects: &'static [Effect],
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Output {
    Text,
    Unit,
    Predicate,
}

impl CommandSpec<'_> {
    pub fn ty(self) -> Type {
        match self.output {
            Output::Text => Type::String,
            Output::Unit => Type::Unit,
            Output::Predicate => Type::Bool,
        }
    }
    pub fn effects(self, capture: bool) -> EffectSet {
        let mut set = EffectSet::of(Effect::Exec);
        set.insert(Effect::Stderr);
        if !capture && self.output == Output::Text {
            set.insert(Effect::Stdout);
        }
        for &effect in self.effects {
            set.insert(effect);
        }
        set
    }
    pub fn validate(self, count: usize) -> Result<()> {
        if count < self.min || self.max.is_some_and(|max| count > max) {
            bail!(
                "wrong number of arguments for `$bin::{}`: got {count}",
                self.word
            );
        }
        Ok(())
    }
}

use Effect::{Env, Fs, Net, Stdin};
use Output::{Predicate, Text, Unit};

macro_rules! commands {
    ($(($min:expr, $max:expr, $ty:expr, $effects:expr) => [$($name:literal),+]),+ $(,)?) => {
        static CATALOG: phf::Map<&'static str, CommandSpec<'static>> = phf::phf_map! {
            $($($name => CommandSpec { word: $name, min: $min, max: $max, output: $ty, effects: $effects },)+)+
        };
    };
}

commands! {
    (0, Some(1), Unit, &[Fs, Env]) => ["cd"],
    (0, None, Text, &[Env]) => [
        "alias", "cal", "date", "fc", "getconf", "getopts", "gettext", "groups", "hash",
        "hostname", "id", "ipcrm", "ipcs", "jobs", "locale", "logname", "ngettext",
        "printenv", "ps", "pwd", "stty", "times", "tput", "tty", "type", "ulimit",
        "umask", "uname", "uptime", "who", "whoami"
    ],
    (0, None, Text, &[]) => ["echo"],
    (0, None, Unit, &[]) => ["false", "true"],
    (1, None, Text, &[]) => ["basename", "dirname", "expr", "printf", "seq"],
    (0, None, Predicate, &[Fs]) => ["test"],
    (1, Some(1), Unit, &[]) => ["sleep"],
    (2, None, Unit, &[Fs]) => ["chgrp", "chmod", "chown", "cp", "install", "link", "ln", "mv"],
    (1, None, Unit, &[Fs]) => ["mkdir", "mkfifo", "rm", "rmdir", "touch", "truncate", "unlink"],
    (0, None, Text, &[Fs]) => [
        "df", "du", "file", "ls", "lsattr", "lsb_release", "lsblk", "lsof", "mktemp",
        "nm", "objdump", "pathchk", "readelf", "readlink", "realpath", "size", "stat",
        "strings", "tree", "what"
    ],
    (0, None, Text, &[Stdin, Fs]) => [
        "ar", "asa", "awk", "base64", "bc", "bzip2", "cat", "cksum", "cmp", "column",
        "comm", "compress", "cpio", "csplit", "cut", "dc", "dd", "diff", "ed", "egrep",
        "ex", "expand", "fd", "fgrep", "fmt", "fold", "grep", "gunzip", "gzip", "head",
        "hexdump", "iconv", "join", "jq", "less", "md5sum", "more", "nl", "od", "paste",
        "patch", "pax", "pr", "rg", "sed", "sha256sum", "shasum", "sort", "split",
        "sponge", "sum", "tabs", "tail", "tar", "tee", "tr", "tsort", "uncompress",
        "unexpand", "uniq", "unzip", "uudecode", "uuencode", "vi", "wc", "xxd", "xz",
        "yq", "zcat", "zip"
    ],
    (0, None, Text, &[Stdin, Fs, Net, Env]) => [
        "admin", "apk", "apt", "ash", "at", "bash", "batch", "brew", "bun", "c17", "c99",
        "cancel", "cargo", "cc", "cflow", "chroot", "clang", "cmake", "crontab", "ctags",
        "curl", "cxref", "dash", "delta", "deno", "dig", "dnf", "docker", "env",
        "envsubst", "find", "fort77", "fuser", "gcc", "gencat", "get", "gh", "git", "go",
        "gpg", "hg", "host", "journalctl", "kubectl", "lex", "localedef", "logger", "lp",
        "lpstat", "m4", "mailx", "make", "man", "mesg", "msgfmt", "nc", "newgrp", "nice",
        "ninja", "nix", "node", "nohup", "npm", "npx", "nslookup", "openssl", "parallel",
        "perl", "ping", "pnpm", "podman", "prs", "python", "python3", "qalter", "qdel",
        "qhold", "qmove", "qmsg", "qrerun", "qrls", "qselect", "qsig", "qstat", "qsub",
        "ranlib", "rmdel", "rsync", "ruby", "rustc", "sact", "sccs", "scp", "screen",
        "service", "sftp", "sh", "ssh", "strip", "su", "sudo", "svn", "systemctl", "talk",
        "time", "timeout", "tmux", "tset", "unget", "users", "uucp", "uustat", "uux",
        "val", "watch", "wget", "whois", "write", "xargs", "xgettext", "yacc", "yarn",
        "zsh"
    ],
    (0, None, Text, &[Stdin, Fs, Net, Env]) => ["command", "eval", "exec"],
    (0, None, Unit, &[Stdin, Env]) => ["read"],
    (0, None, Unit, &[Env]) => [
        "bg", "break", "continue", "exit", "export", "fg", "kill", "readonly", "renice",
        "return", "set", "shift", "trap", "unalias", "unset", "wait"
    ],
}

/// A program named explicitly, as `use $bin::{vp}` or `use $bin::{"apt-get" as apt_get}`,
/// need not be in the catalog: it takes any argv, may print, and may use every capability.
const DECLARED: CommandSpec<'static> = CommandSpec {
    word: "",
    min: 0,
    max: None,
    output: Text,
    effects: &[Stdin, Fs, Net, Env],
};

pub(crate) fn lookup(path: &str) -> Option<CommandSpec<'_>> {
    let word = path.strip_prefix("$bin::")?;
    if let Some(spec) = CATALOG.get(word) {
        return Some(*spec);
    }
    is_program_name(word).then_some(CommandSpec { word, ..DECLARED })
}

/// A program name is one shell word that needs no quoting and is not an option.
pub(crate) fn is_program_name(word: &str) -> bool {
    !word.is_empty()
        && !word.starts_with('-')
        && word
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'+'))
}

pub(super) fn contains(word: &str) -> bool {
    CATALOG.contains_key(word)
}

pub fn command_names() -> impl Iterator<Item = &'static str> {
    CATALOG.keys().copied()
}
