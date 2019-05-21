use std::collections::HashSet;

use symbol::Symbol;

use crate::env::Environment;
use crate::ast::{Decl as AstDecl, Expr as AstExpr, Stmt as AstStmt, Type};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Constraint(pub Type, pub Type);

// get constraints

pub fn get_constraints_decl(type_env: &mut Environment<Symbol, Type>, constraints: &mut HashSet<Constraint>, decl: &AstDecl) {
    match decl {
        AstDecl::Extern(_, _, _) => (),
        AstDecl::Func(func) => {
            for stmt in &func.body {
                get_constraints_stmt(type_env, constraints, stmt);
            }
        }
    }
}

pub fn get_constraints_stmt(type_env: &mut Environment<Symbol, Type>, constraints: &mut HashSet<Constraint>, stmt: &AstStmt) {
    match stmt {
        AstStmt::Return(expr) | AstStmt::Expr(expr) => get_constraints_expr(type_env, constraints, expr),
    }
}

pub fn get_constraints_expr(type_env: &mut Environment<Symbol, Type>, constraints: &mut HashSet<Constraint>, expr: &AstExpr) {
    match expr {
        AstExpr::Int(_) => (),
        AstExpr::Call(_, _) => {
            // TODO: finish this
        }
    }
}
