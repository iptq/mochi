use std::error::Error as StdError;
use std::fmt;

use symbol::Symbol;

use crate::ast;
pub use crate::ast::{Class, LValue, Match, Op, Path, Pattern, Use};

use crate::Position;
use crate::Type;

#[derive(Debug)]
pub enum TypeError {
    None,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unimplemented"),
        }
    }
}

impl StdError for TypeError {}

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
        use ast::DeclKind as aDeclKind;
        Ok(Decl {
            kind: match decl.kind {
                aDeclKind::Class(class) => DeclKind::Class(class),
                aDeclKind::Func(func) => DeclKind::Func(Func::from(func)?),
                aDeclKind::Type(ty) => DeclKind::Type(ty),
                aDeclKind::Use(_use) => DeclKind::Use(_use),
            },
            pos: decl.pos,
        })
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

impl Expr {
    pub fn from(expr: ast::Expr) -> Result<Self, TypeError> {
        use ast::ExprKind as aExprKind;
        let (kind, ty) = match expr.kind {
            aExprKind::BinOp(left, op, right) => (
                ExprKind::BinOp(
                    Box::new(Expr::from(*left)?),
                    op,
                    Box::new(Expr::from(*right)?),
                ),
                Type::gen(),
            ),
            aExprKind::Call(left, right) => (
                ExprKind::Call(Box::new(Expr::from(*left)?), Box::new(Expr::from(*right)?)),
                Type::gen(),
            ),
            aExprKind::IntLiteral(nstr) => (ExprKind::IntLiteral(nstr), Type::gen()),
            aExprKind::Unit => (ExprKind::Unit, Type::Unit),
            _ => (ExprKind::Unit, Type::gen()),
        };
        Ok(Expr {
            kind,
            pos: expr.pos,
            ty,
        })
    }
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

impl Func {
    pub fn from(func: ast::Func) -> Result<Self, TypeError> {
        Err(TypeError::None)
    }
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
