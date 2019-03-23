use symbol::Symbol;

use crate::ast;
pub use crate::ast::{Class, LValue, Match, Op, Path, Pattern, Use};

use crate::Position;
use crate::Type;

#[derive(Debug)]
pub enum TypeError {
    None,
}

#[derive(Debug)]
pub struct Program(pub Vec<Decl>);

impl Program {
    pub fn from(program: ast::Program) -> Result<Self, TypeError> {
        program
            .0
            .into_iter()
            .map(|decl| Decl::from(decl))
            .collect::<Result<Vec<_>, _>>()
            .map(|decls| Program(decls))
    }
}

#[derive(Debug)]
pub struct Decl {
    pub kind: DeclKind,
    pub pos: Position,
}

impl Decl {
    pub fn from(decl: ast::Decl) -> Result<Self, TypeError> {
        Err(TypeError::None)
    }
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
    pub ty: Type,
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
    pub returns: Type,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct FuncArg(pub Symbol, pub Type);

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub pos: Position,
}

#[derive(Debug)]
pub enum StmtKind {
    Expr(Expr),
    ForLoop(Pattern, Expr, Vec<Stmt>),
    Let(LValue, Expr),
}
