use crate::{
    ast::{Comprehension, ComprehensionClause, Expr, Type},
    codegen::{FunctionRegistry, binding_for_name, compile_runtime_primitive_expr},
    env::{CodegenState, EnumRegistry, Env, Storage},
    sourcemap::OutputBuffer,
    traits::TraitImplRegistry,
};
use anyhow::{Result, bail};

#[allow(clippy::too_many_arguments)]
pub(super) fn emit(
    target: &str,
    c: &Comprehension,
    item: &Type,
    env: &Env,
    functions: &FunctionRegistry,
    impls: &TraitImplRegistry,
    enums: &EnumRegistry,
    state: &mut CodegenState,
    inside_function: bool,
    out: &mut OutputBuffer,
) -> Result<()> {
    let count = state.temp_var("comprehension_count");
    out.push_str(&format!("{count}=0\n"));
    let mut emitter = Emitter {
        functions,
        impls,
        enums,
        state,
        inside_function,
        target,
        count: &count,
        item,
        value: &c.value,
    };
    emitter.clauses(&c.clauses, env, out)?;
    out.push_str(&format!("{target}__len=\"${{{count}}}\"\n"));
    Ok(())
}
struct Emitter<'a> {
    functions: &'a FunctionRegistry,
    impls: &'a TraitImplRegistry,
    enums: &'a EnumRegistry,
    state: &'a mut CodegenState,
    inside_function: bool,
    target: &'a str,
    count: &'a str,
    item: &'a Type,
    value: &'a Expr,
}
impl Emitter<'_> {
    fn clauses(
        &mut self,
        clauses: &[ComprehensionClause],
        env: &Env,
        out: &mut OutputBuffer,
    ) -> Result<()> {
        let Some((clause, rest)) = clauses.split_first() else {
            return self.append(env, out);
        };
        match clause {
            ComprehensionClause::If(e) => {
                let condition = compile_runtime_primitive_expr(
                    e,
                    env,
                    self.functions,
                    self.impls,
                    self.enums,
                    self.state,
                    self.inside_function,
                    out,
                )?;
                out.push_str(&format!("if [ {condition} = true ]; then\n"));
                self.clauses(rest, env, out)?;
                out.push_str("fi\n");
            }
            ComprehensionClause::For { name, iterable } => {
                let binding = super::materialize_expr(
                    iterable,
                    env,
                    self.functions,
                    self.impls,
                    self.enums,
                    self.state,
                    self.inside_function,
                    out,
                )?;
                let (Type::List(item), Storage::Aggregate(prefix)) = (binding.ty, binding.storage)
                else {
                    bail!("comprehension expects a list or range");
                };
                let index = self.state.temp_var("comprehension_index");
                let value = self.state.temp_var("comprehension_item");
                out.push_str(&format!(
                    "{index}=0\nwhile [ \"${{{index}}}\" -lt \"${{{prefix}__len}}\" ]; do\n"
                ));
                if item.is_structured() {
                    crate::codegen::wire::copy(
                        &item,
                        &format!("{prefix}__${{{index}}}"),
                        &value,
                        self.state,
                        out,
                    );
                } else {
                    out.push_str(&format!(
                        "eval \"{value}=\\\"\\${{{prefix}__${{{index}}}}}\\\"\"\n"
                    ));
                }
                let mut inner = env.clone();
                inner.insert(name.clone(), binding_for_name(&value, *item));
                self.clauses(rest, &inner, out)?;
                out.push_str(&format!("{index}=$(({index} + 1))\ndone\n"));
            }
        }
        Ok(())
    }
    fn append(&mut self, env: &Env, out: &mut OutputBuffer) -> Result<()> {
        let value = self.state.temp_var("comprehension_value");
        super::emit_value_to_target(
            &value,
            self.value,
            self.item,
            env,
            self.functions,
            self.impls,
            self.enums,
            self.state,
            self.inside_function,
            out,
        )?;
        let target = format!("{}__${{{}}}", self.target, self.count);
        if self.item.is_structured() {
            crate::codegen::wire::copy(self.item, &value, &target, self.state, out);
        } else {
            out.push_str(&format!("eval \"{target}=\\${value}\"\n"));
        }
        out.push_str(&format!("{}=$(({} + 1))\n", self.count, self.count));
        Ok(())
    }
}
