use symbol::Symbol;

use crate::ast::Path;

#[derive(Debug)]
pub enum Type {
    Func(Vec<Type>, Box<Type>),
    Name(Symbol),
    Unit,
    Int,
}

#[derive(Debug)]
pub struct Program(pub Vec<Decl>);

#[derive(Debug)]
pub enum Decl {
    Extern(Symbol, Vec<Type>, Type),
    Func(Func),
}

#[derive(Debug)]
pub struct Func {
    pub name: Symbol,
    pub body: Vec<Stmt>,
    pub returns: Type,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Call(Path, Vec<Expr>, Type),
}
