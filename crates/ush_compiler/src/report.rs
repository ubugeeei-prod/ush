//! `ush effects`: the row every function has, written or inferred.

use alloc::{string::String, vec::Vec};
use anyhow::Result;

#[cfg(feature = "std")]
use anyhow::Context;
#[cfg(feature = "std")]
use std::{fs, path::Path};

use crate::{EffectSet, FunctionEffects, UshCompiler, codegen, contracts, effects, modules};

/// What a whole program touches, function by function.
#[derive(Debug, Clone)]
pub struct EffectReport {
    pub functions: Vec<FunctionEffects>,
    pub top_level: EffectSet,
}

impl UshCompiler {
    /// The row of every function in `path`, plus the effects of the
    /// top-level program.
    #[cfg(feature = "std")]
    pub fn effects_file(&self, path: &Path) -> Result<EffectReport> {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let mut loader = crate::file::FsLoader::new(path, path.to_str().unwrap_or("<source>"))?;
        self.effects_with(&source, &mut loader)
            .with_context(|| format!("failed to analyze {}", path.display()))
    }

    /// Like [`Self::effects_file`] for a buffer holding `path`'s text,
    /// as editors have it before saving.
    #[cfg(feature = "std")]
    pub fn effects_source_at(&self, source: &str, path: &Path) -> Result<EffectReport> {
        let mut loader = crate::file::FsLoader::new(path, path.to_str().unwrap_or("<source>"))?;
        self.effects_with(source, &mut loader)
    }

    pub fn effects_source(&self, source: &str) -> Result<EffectReport> {
        self.effects_with(source, &mut modules::NoModules)
    }

    fn effects_with(
        &self,
        source: &str,
        loader: &mut dyn modules::ModuleLoader,
    ) -> Result<EffectReport> {
        let modules::Linked { program, space } =
            modules::link(source, loader).map_err(|error| anyhow::anyhow!("{error:#}"))?;
        let report = || {
            crate::bindings::validate(&program)?;
            let mut functions = effects::describe_function_effects(&program)?;
            let lowered = contracts::elaborate(program.clone())?;
            // A generic function's instances report under its own name.
            for actual in effects::describe_function_effects(&lowered)? {
                if let Some(original) = functions
                    .iter_mut()
                    .find(|f| f.name == template(&actual.name))
                {
                    original.inferred.merge(&actual.inferred);
                }
            }
            for (name, errors) in codegen::error_rows(&lowered)? {
                if let Some(original) = functions.iter_mut().find(|f| f.name == template(&name)) {
                    for error in errors.iter() {
                        let error = String::from(error.render().as_str());
                        if !original.raises.contains(&error) {
                            original.raises.push(error);
                        }
                    }
                    original.raises.sort();
                }
            }
            Ok(EffectReport {
                functions,
                top_level: effects::top_level_effects(&lowered)?,
            })
        };
        report().map_err(|error| space.render_error(error))
    }
}

/// `__ush_inst_map_0` is an instance of `map`.
fn template(name: &str) -> &str {
    name.strip_prefix("__ush_inst_")
        .and_then(|name| name.rsplit_once('_').map(|(name, _)| name))
        .unwrap_or(name)
}
