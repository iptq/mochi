use std::collections::{BTreeSet, HashMap};
use std::fmt;

use symbol::Symbol;

use crate::ast::*;
use crate::Type;

#[derive(Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Constraint(pub Type, pub Type);

impl PartialEq for Constraint {
    fn eq(&self, other: &Constraint) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

#[derive(Debug)]
pub enum TypeError {
    NotFound(Symbol),
    CantUnify(Type, Type),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type error")
    }
}

pub fn unify(
    constraints: BTreeSet<Constraint>,
    environment: &HashMap<Symbol, Type>,
) -> Result<HashMap<Symbol, Type>, TypeError> {
    let mut constraints = constraints.into_iter().collect::<Vec<_>>();
    let mut substitutions = HashMap::new();
    while let Some(Constraint(left, right)) = constraints.pop() {
        match &(left, right) {
            (left, right) if left == right => (),
            (Type::Var(sym), ty) | (ty, Type::Var(sym)) => match environment.get(sym) {
                Some(t) => constraints.push(Constraint(t.clone(), ty.clone())),
                None => return Err(TypeError::NotFound(sym.clone())),
            },
            (Type::N(sym), ty) | (ty, Type::N(sym)) => {
                substitutions.insert(sym.clone(), ty.clone());
            }
            (Type::Func(left1, right1), Type::Func(left2, right2)) => {
                constraints.push(Constraint(*left1.clone(), *left2.clone()));
                constraints.push(Constraint(*right1.clone(), *right2.clone()));
            }
            (left, right) => return Err(TypeError::CantUnify(left.clone(), right.clone())),
        }
    }
    Ok(substitutions)
}

pub trait TypeCheck {
    fn constraints(&self) -> BTreeSet<Constraint>;
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>);
}

impl TypeCheck for Program {
    fn constraints(&self) -> BTreeSet<Constraint> {
        self.0.iter().fold(BTreeSet::new(), |a, b| {
            a.union(&b.constraints()).cloned().collect()
        })
    }
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>) {}
}

impl TypeCheck for Decl {
    fn constraints(&self) -> BTreeSet<Constraint> {
        match &self.kind {
            DeclKind::Func(func) => func.constraints(),
            _ => BTreeSet::new(),
        }
    }
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>) {}
}

impl TypeCheck for Expr {
    fn constraints(&self) -> BTreeSet<Constraint> {
        match &self.kind {
            ExprKind::Call(func, arg) => {
                let mut constraints = vec![Constraint(
                    self.ty.clone(),
                    Type::Func(Box::new(func.ty.clone()), Box::new(arg.ty.clone())),
                )];
                constraints.extend(func.constraints());
                constraints.extend(arg.constraints());
                constraints.into_iter().collect()
            }
            _ => BTreeSet::new(),
        }
    }
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>) {}
}

impl TypeCheck for Func {
    fn constraints(&self) -> BTreeSet<Constraint> {
        let mut constraints = Vec::new();
        if self.args.len() == 0 {
            constraints.push(Constraint(
                self.ty.clone(),
                Type::Func(Box::new(Type::Unit), Box::new(self.returns.clone())),
            ));
        }
        for stmt in self.body.iter() {
            constraints.extend(stmt.constraints());
        }
        constraints.into_iter().collect()
    }
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>) {}
}

impl TypeCheck for Stmt {
    fn constraints(&self) -> BTreeSet<Constraint> {
        match &self.kind {
            StmtKind::Expr(expr) => expr.constraints(),
            _ => BTreeSet::new(),
        }
    }
    fn apply_subst(&mut self, constraints: BTreeSet<Constraint>) {}
}
