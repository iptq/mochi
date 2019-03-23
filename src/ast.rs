use symbol::Symbol;

use crate::Position;
use crate::Type;

#[derive(Debug)]
pub struct Class {}

#[derive(Debug)]
pub struct Decl {
    pub kind: DeclKind,
    pub pos: Position,
}

#[derive(Debug)]
pub enum DeclKind {
    Class(Class),
    Func(Func),
    Type(Type),
    Use(Use),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub pos: Position,
}

#[derive(Debug)]
pub enum ExprKind {
    BinOp(Box<Expr>, Op, Box<Expr>),
    IntLiteral(String),
    Match,
    Var(Symbol),
}

#[derive(Debug)]
pub struct Func {}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum StmtKind {
    Let,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Use {}
