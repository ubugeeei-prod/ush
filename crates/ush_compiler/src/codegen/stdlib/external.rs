use super::{builtin, param};
use crate::{
    ast::{FunctionDef, Type},
    types::HeapVec as Vec,
};
pub(super) fn definitions() -> Vec<FunctionDef> {
    vec![
        builtin(
            "std::__extern::int",
            vec![param("value", Type::String)],
            Some(Type::Int),
        ),
        builtin(
            "std::__extern::bool",
            vec![param("value", Type::String)],
            Some(Type::Bool),
        ),
    ]
}
