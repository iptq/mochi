use std::collections::HashMap;

use symbol::Symbol;

use crate::ast::*;
use crate::Type;

enum EnvironmentItem {}

pub struct SemanticChecker {
    global_names: HashMap<Symbol, EnvironmentItem>,
    scope_stack: Vec<HashMap<Symbol, Type>>,
}

impl SemanticChecker {
    pub fn new() -> Self {
        SemanticChecker {
            global_names: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    pub fn visit_program(&mut self, program: &Program) {
        // compute all "signatures"
        for decl in &program.0 {
            match &decl.kind {
                DeclKind::Class(class) => (),
                DeclKind::Extern(class) => (),
                DeclKind::Func(class) => {}
                DeclKind::Type(class) => (),
                DeclKind::Use(class) => (),
            }
        }

        // verify types

        // verify functions
    }
}
