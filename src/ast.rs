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
    Call(Box<Expr>, Box<Expr>),
    IntLiteral(String),
    Match(Box<Match>),
    Path(Path),
    Range(Box<Expr>, Box<Expr>),
    StringLiteral(String),
    Tuple(Vec<Expr>),
    Unit,
    Var(Symbol),
}

#[derive(Debug)]
pub struct Func {
    pub name: Symbol,
    pub args: Vec<FuncArg>,
    pub returns: Option<TypeLiteral>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct FuncArg(pub Symbol, pub Option<TypeLiteral>);

#[derive(Debug)]
pub enum LValue {
    Ident(Symbol),
}

#[derive(Debug)]
pub struct Match(pub Expr, pub Vec<MatchArm>);

#[derive(Debug)]
pub struct MatchArm(pub Pattern, pub Expr);

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug)]
pub struct Path(pub Vec<Symbol>);

#[derive(Debug)]
pub enum Pattern {
    IntLiteral(String),
    StringLiteral(String),
    Tuple(Vec<Pattern>),
    Var(Symbol),
    Wildcard,
}

#[derive(Debug)]
pub enum StmtKind {
    Expr(Expr),
    ForLoop(Pattern, Expr, Vec<Stmt>),
    Let(LValue, Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub pos: Position,
}

#[derive(Debug)]
pub struct TypeLiteral {
    pub kind: Type,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Use(pub Vec<Path>);
