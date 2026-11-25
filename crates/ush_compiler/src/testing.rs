use crate::{
    UshCompiler,
    ast::{Expr, StatementKind},
    types::HeapVec as Vec,
};
use anyhow::Result;

/// A named or anonymous source-level test block.
#[derive(Clone, Debug)]
pub struct TestCase {
    pub name: alloc::string::String,
    pub line: usize,
}

impl UshCompiler {
    pub fn test_cases_source(&self, source: &str) -> Result<Vec<TestCase>> {
        Ok(crate::parse::parse_program(source)?
            .into_iter()
            .filter_map(|s| {
                let StatementKind::Function(f) = s.kind else {
                    return None;
                };
                let label = f.attrs.iter().find(|a| a.name == "test")?;
                let Some(Expr::String(name)) = &label.value else {
                    return None;
                };
                Some(TestCase {
                    name: name.as_str().into(),
                    line: s.line,
                })
            })
            .collect())
    }

    #[cfg(feature = "std")]
    pub fn compile_test_file(
        &self,
        path: &std::path::Path,
        line: usize,
    ) -> Result<alloc::string::String> {
        use anyhow::{Context, ensure};
        let mut source = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        ensure!(
            self.test_cases_source(&source)?
                .iter()
                .any(|case| case.line == line),
            "no test block at line {line}"
        );
        source.push_str(&format!("\n__ush_test_{line}\n"));
        let absolute = std::fs::canonicalize(path)?;
        self.compile_with_context(
            &source,
            None,
            crate::codegen::SourceContext::Absolute {
                directory: absolute.parent().and_then(|p| p.to_str()).unwrap_or(""),
                path: absolute.to_str().unwrap_or(""),
                origin: path.to_str().unwrap_or("<test>"),
            },
            false,
        )
        .map(|out| out.into_shell())
    }
}
