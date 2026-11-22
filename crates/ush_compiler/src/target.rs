use core::{fmt, str::FromStr};

/// Interpreter of the generated executable. The compiler uses POSIX shell syntax.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShellTarget {
    #[default]
    Sh,
    Bash,
    Zsh,
    Ash,
}

impl ShellTarget {
    pub fn name(self) -> &'static str {
        match self {
            Self::Sh => "sh",
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Ash => "ash",
        }
    }
}

impl fmt::Display for ShellTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for ShellTarget {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sh" => Ok(Self::Sh),
            "bash" => Ok(Self::Bash),
            "zsh" => Ok(Self::Zsh),
            "ash" => Ok(Self::Ash),
            _ => Err("expected sh, bash, zsh, or ash"),
        }
    }
}
