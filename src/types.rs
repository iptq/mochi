use std::collections::BTreeSet;

use symbol::Symbol;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Unit,
    Bool,
    Int32,
    Sint32,
    Int64,
    Sint64,
    Record(Vec<(Symbol, Type)>),
    Func(Box<Type>, Box<Type>),
    Var(Symbol),

    #[doc(hidden)]
    N(Symbol),
}

impl Type {
    pub fn gen() -> Type {
        Type::N(Symbol::gensym())
    }

    pub fn freevars(&self) -> BTreeSet<Symbol> {
        match self {
            Type::Unit | Type::Bool | Type::Int32 | Type::Sint32 | Type::Int64 | Type::Sint64 => {
                BTreeSet::new()
            }
            Type::Record(fields) => fields.iter().fold(BTreeSet::new(), |a, b| {
                a.union(&b.1.freevars()).into_iter().cloned().collect()
            }),
            Type::Func(left, right) => left
                .freevars()
                .into_iter()
                .chain(right.freevars())
                .collect(),
            Type::Var(sym) | Type::N(sym) => vec![sym.clone()].into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub struct TypeFunc(pub Type, pub Type);
