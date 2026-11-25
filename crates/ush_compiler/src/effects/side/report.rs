//! How a function's row reads in `ush effects` and editor hovers.

use super::FunctionEffects;
use crate::types::OutputString;

impl FunctionEffects {
    /// The row as written after `/`: error types, then effects, or
    /// `pure` when there are neither.
    pub fn row(&self) -> OutputString {
        render(&self.raises, &self.inferred)
    }

    /// The written row, when the signature has one.
    pub fn declared_row(&self) -> Option<OutputString> {
        let effects = self.declared.as_ref()?;
        Some(render(
            self.declared_raises.as_deref().unwrap_or_default(),
            effects,
        ))
    }
}

fn render(raises: &[OutputString], effects: &super::EffectSet) -> OutputString {
    match (raises.is_empty(), effects.is_empty()) {
        (true, _) => format!("{effects}"),
        (false, true) => raises.join(", "),
        (false, false) => format!("{}, {effects}", raises.join(", ")),
    }
}
