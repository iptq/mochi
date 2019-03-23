use symbol::Symbol;

#[derive(Debug)]
pub enum Type {
    Unit,
    Bool,
    Int32,
    Sint32,
    Int64,
    Sint64,
    Record(Vec<(Symbol, Type)>),
    Var(Symbol),

    #[doc(hidden)]
    N(Symbol),
}

impl Type {
    pub fn gen() -> Type {
        Type::N(Symbol::gensym())
    }
}

#[derive(Debug)]
pub enum Constraint {}

#[derive(Debug)]
pub enum Kind {
    Type(Type),
    ParamType(Box<Kind>, Box<Kind>),
}
