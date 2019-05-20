use std::collections::HashMap;

use symbol::Symbol;

use crate::ast::*;
use crate::Type;

#[derive(Debug)]
enum EnvItem {
    StructType,
    FuncType(Type),
}

impl From<&Func> for EnvItem {
    fn from(func: &Func) -> Self {
        let mut ty = func.returns.clone();
        for FuncArg(_, ty0) in func.args.iter().rev() {
            ty = Type::Func(Box::new(ty0.clone()), Box::new(ty));
        }
        EnvItem::FuncType(ty)
    }
}

type Scope = HashMap<Symbol, EnvItem>;

pub struct SemanticChecker {
    scope_stack: Vec<Scope>,
}

impl SemanticChecker {
    pub fn new() -> Self {
        SemanticChecker {
            scope_stack: Vec::new(),
        }
    }

    fn lookup_name(&self, name: Symbol) -> Option<EnvItem> {
        None
    }

    pub fn visit_program(&mut self, program: &Program) {
        // compute all "signatures"
        for decl in &program.0 {
            match &decl.kind {
                DeclKind::Trait(_trait) => println!("Found trait {:?}", _trait),
                DeclKind::Struct(_struct) => println!("Found struct {:?}", _struct),
                DeclKind::Extern(_extern) => println!("Found extern {:?}", _extern),
                DeclKind::Func(func) => {
                    println!("Found func {}: {:?}", func.name, EnvItem::from(func))
                }
                DeclKind::Type(_type) => println!("Found type {:?}", _type),
                DeclKind::Use(_use) => println!("Found use {:?}", _use),
            }
        }

        // verify types

        // verify functions
    }
}
