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
    WrongArgs(usize, usize),
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type error")
    }
}

pub fn unify(
    constraints: BTreeSet<Constraint>,
    environment: &mut HashMap<Symbol, Type>,
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
            (Type::Func(args1, returns1), Type::Func(args2, returns2)) => {
                if args1.len() != args2.len() {
                    return Err(TypeError::WrongArgs(args1.len(), args2.len()));
                }
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    constraints.push(Constraint(arg1.clone(), arg2.clone()));
                }
                constraints.push(Constraint(*returns1.clone(), *returns2.clone()));
            }
            (left, right) => return Err(TypeError::CantUnify(left.clone(), right.clone())),
        }
    }
    Ok(substitutions)
}

pub trait TypeCheck {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError>;
    fn apply_subst(&mut self, subst: &HashMap<Symbol, Type>);
}

impl TypeCheck for Program {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError> {
        let constraints = BTreeSet::new();
        for decl in self.0 {
            constraints.extend(decl.constraints()?);
        }
        Ok(constraints)
    }
    fn apply_subst(&mut self, subst: &HashMap<Symbol, Type>) {
        for decl in &mut self.0 {
            decl.apply_subst(subst);
        }
    }
}

impl TypeCheck for Decl {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError> {
        match &self.kind {
            DeclKind::Func(func) => func.constraints(),
            _ => Ok(BTreeSet::new()),
        }
    }
    fn apply_subst(&mut self, subst: &HashMap<Symbol, Type>) {
        match &mut self.kind {
            DeclKind::Func(func) => func.apply_subst(subst),
            _ => (),
        }
    }
}

impl TypeCheck for Expr {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError> {
        match &self.kind {
            ExprKind::Call(func, arg) => {
                let mut constraints = vec![Constraint(
                    func.ty.clone(),
                    Type::Func(func.args.map().collect(), Box::new(arg.ty.clone())),
                )];
                constraints.extend(func.constraints()?);
                constraints.extend(arg.constraints()?);
                Ok(constraints.into_iter().collect())
            }
            _ => BTreeSet::new(),
        }
    }
    fn apply_subst(&mut self, subst: &HashMap<Symbol, Type>) {}
}

impl TypeCheck for Func {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError> {
        let mut constraints = Vec::new();
        if self.args.len() == 0 {
            constraints.push(Constraint(
                self.ty.clone(),
                Type::Func(Vec::new(), Box::new(self.returns.clone())),
            ));
        }
        for stmt in self.body.iter() {
            constraints.extend(stmt.constraints()?);
        }
        Ok(constraints.into_iter().collect())
    }
    fn apply_subst(&mut self, constraints: &HashMap<Symbol, Type>) {}
}

impl TypeCheck for Stmt {
    fn constraints(&self) -> Result<BTreeSet<Constraint>, TypeError> {
        match &self.kind {
            StmtKind::Expr(expr) => expr.constraints(),
            _ => Ok(BTreeSet::new()),
        }
    }
    fn apply_subst(&mut self, subst: &HashMap<Symbol, Type>) {}
}
