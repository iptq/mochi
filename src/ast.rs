use crate::Type;
use crate::Position;

#[derive(Debug)]
pub enum DeclKind {
    Class(Class),
    Type(Type),
}

#[derive(Debug)]
pub struct Decl {
    pub kind: DeclKind,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Class {
    pub pos: Position,
}

#[derive(Debug)]
pub enum Constraint {
	Is(String, Class),
}

#[derive(Debug)]
pub enum KindKind {
	Type(Type),
	ParamType(Box<Kind>, Box<Kind>),
}

#[derive(Debug)]
pub struct Kind {
	pub kind: KindKind,
	pub pos: Position,
}

#[derive(Debug)]
pub enum ExprKind {
    IntLiteral(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub pos: Position,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
