//! Loads a script's modules depth-first and inlines them, dependencies
//! first, ahead of the script's own statements.

use alloc::{rc::Rc, string::String};
use anyhow::{Error, Result, anyhow};

use super::{
    ModuleLoader, Scope, SourceSpace,
    checks::{distinct_shell_names, ensure_no_shadowed_namespace},
    exports::Exports,
    reach,
    relocate::relocate,
    rename::rename,
    space,
};
use crate::{
    ast::{ModuleDecl, Statement, StatementKind as S},
    parse::{FileRole, parse_file},
    types::{AstString as Name, HeapVec as Vec, Map, Set},
};

pub(crate) struct Linked {
    pub program: Vec<Statement>,
    pub space: SourceSpace,
}

pub(crate) fn link(root: &str, loader: &mut dyn ModuleLoader) -> Result<Linked> {
    let program = parse_file(root, FileRole::Script)?;
    let mut space = SourceSpace::default();
    if !program.iter().any(|s| matches!(s.kind, S::Module(_))) {
        // Without modules the script is resolved on its own, as before.
        space.push(root, String::new(), String::new(), 0);
        let program = crate::imports::resolve_file(program, &Scope::default())?;
        return Ok(Linked { program, space });
    }
    let (key, display) = loader.root();
    let (key, display) = (String::from(key), String::from(display));
    let mut linker = Linker {
        loader,
        space,
        files: Vec::new(),
        keys: Map::default(),
        stack: vec![0],
        namespaces: Set::default(),
        modules: Vec::new(),
    };
    linker.space.push(root, display.clone(), display, 0);
    linker.keys.insert(key.clone(), 0);
    linker.files.push(File {
        key,
        namespace: None,
        dir: String::new(),
        exports: None,
    });
    let mut program = linker.link_file(0, program)?;
    let mut modules = core::mem::take(&mut linker.modules);
    if !modules.is_empty() {
        reach::shake(&mut modules, &mut program);
        modules.append(&mut program);
        program = modules;
        distinct_shell_names(&program)?;
    }
    Ok(Linked {
        program,
        space: linker.space,
    })
}

struct File {
    key: String,
    /// `None` for the root script.
    namespace: Option<Name>,
    dir: String,
    /// Set once the file is linked; importers read it.
    exports: Option<Rc<Exports>>,
}

struct Linker<'l> {
    loader: &'l mut dyn ModuleLoader,
    space: SourceSpace,
    files: Vec<File>,
    keys: Map<String, usize>,
    /// Files being linked, for cycle detection.
    stack: Vec<usize>,
    namespaces: Set<Name>,
    /// Linked module statements, dependencies first.
    modules: Vec<Statement>,
}

impl Linker<'_> {
    fn link_file(&mut self, index: usize, mut program: Vec<Statement>) -> Result<Vec<Statement>> {
        if index == 0 {
            // A module namespace must not shadow `extern "x.sh" as name`.
            for statement in &program {
                if let S::Function(def) = &statement.kind
                    && let Some((namespace, _)) = def.name.split_once("::")
                {
                    self.namespaces.insert(namespace.into());
                }
            }
        }
        let mut aliases = Map::default();
        for statement in &program {
            if let S::Module(decl) = &statement.kind {
                let child = self.import(index, statement.line, decl)?;
                let exports = self.files[child].exports.clone().expect("linked module");
                if aliases.insert(decl.alias.clone(), exports).is_some() {
                    let error = anyhow!("duplicate module alias: {}", decl.alias);
                    return Err(
                        self.locate(index, error.context(format!("line {}", statement.line)))
                    );
                }
            }
        }
        let namespace = self.files[index].namespace.clone();
        // Only modules are imported; the root's items keep their names.
        let exports = (index > 0).then(|| {
            Exports::collect(
                &program,
                namespace.as_deref(),
                &self.space.file(index).display,
            )
        });
        let mut scope = Scope {
            namespace: namespace.as_deref(),
            modules: aliases,
            ..Scope::default()
        };
        let mut program = (|| {
            ensure_no_shadowed_namespace(&program, &scope)?;
            scope.take_imports(&mut program)?;
            let mut program = crate::imports::resolve_file(program, &scope)?;
            // A script without modules has no type names to change.
            if namespace.is_some() || !scope.modules.is_empty() {
                rename(&mut program, &scope)?;
            }
            Ok(program)
        })()
        .map_err(|error| self.locate(index, error))?;
        program.retain(|statement| match &statement.kind {
            S::Module(_) => false,
            S::Use(items) => index == 0 && !items.is_empty(),
            _ => true,
        });
        if index > 0 {
            let base = self.space.file(index).base;
            relocate(&mut program, base, &self.files[index].dir);
        }
        self.files[index].exports = exports.map(Rc::new);
        Ok(program)
    }

    /// Loads and links the module behind `mod`, or reuses it.
    fn import(&mut self, importer: usize, line: usize, decl: &ModuleDecl) -> Result<usize> {
        let at = |error: Error| error.context(format!("line {line}"));
        let source = self
            .loader
            .load(&self.files[importer].key, &decl.path)
            .map_err(|error| self.locate(importer, at(error)))?;
        if let Some(&existing) = self.keys.get(&source.key) {
            if let Some(start) = self.stack.iter().position(|&file| file == existing) {
                let mut cycle = self.stack[start..]
                    .iter()
                    .map(|&file| self.space.file(file).display.as_str())
                    .collect::<Vec<_>>();
                cycle.push(&source.display);
                let error = anyhow!("import cycle: {}", cycle.join(" -> "));
                return Err(self.locate(importer, at(error)));
            }
            return Ok(existing);
        }
        let child = self.files.len();
        let root_line = match importer {
            0 => line,
            _ => self.space.file(importer).root_line,
        };
        self.space.push(
            &source.text,
            source.display.clone(),
            source.origin,
            root_line,
        );
        let namespace = self.unique_namespace(&decl.alias);
        self.keys.insert(source.key.clone(), child);
        self.files.push(File {
            key: source.key,
            namespace: Some(namespace),
            dir: source.dir,
            exports: None,
        });
        let role = FileRole::Module {
            dir: &self.files[child].dir,
        };
        let program = parse_file(&source.text, role).map_err(|error| self.locate(child, error))?;
        self.stack.push(child);
        let program = self.link_file(child, program)?;
        self.stack.pop();
        self.modules.extend(program);
        Ok(child)
    }

    /// The importing alias, numbered when another module already has it.
    fn unique_namespace(&mut self, alias: &str) -> Name {
        let mut namespace = Name::from(alias);
        let mut suffix = 2;
        while !self.namespaces.insert(namespace.clone()) {
            namespace = format!("{alias}_{suffix}").into();
            suffix += 1;
        }
        namespace
    }

    /// Names the module file and line of an error raised while reading it.
    fn locate(&self, index: usize, error: Error) -> Error {
        if index == 0 {
            return error;
        }
        let file = self.space.file(index);
        space::relocate(error, |line| Some(SourceSpace::describe(file, line)))
    }
}
