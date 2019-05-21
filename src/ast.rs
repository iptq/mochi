use symbol::Symbol;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Type {
    Func(Vec<Type>, Box<Type>),
    Name(Symbol),
    Var(Symbol),
    Unit,
    Int,
}

impl Type {
    pub fn gen() -> Type {
        Type::Var(Symbol::gensym())
    }
}

#[derive(Debug)]
pub struct Path(Vec<Symbol>);

#[derive(Debug)]
pub enum BinOp {
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    LeftShift,
    RightShift,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnOp {
    LogicalNot,
    BitwiseNot,
}

#[derive(Debug)]
pub struct Program(pub Vec<Decl>);

#[derive(Debug)]
pub enum Decl {
    Extern(Symbol, Vec<Type>, Type),
    Func(Func),
}

impl Decl {
    pub fn get_signature(&self) -> Option<(Symbol, Type)> {
        match self {
            Decl::Extern(name, args, returns) => {
                let name = name.clone();
                let ty = Type::Func(args.to_vec(), Box::new(returns.clone()));
                Some((name, ty))
            }
            Decl::Func(func) => {
                let name = func.name.clone();
                let args = func.args.iter().map(|(_, ty)| ty.clone()).collect();
                let ty = Type::Func(args, Box::new(func.returns.clone()));
                Some((name, ty))
            }
        }
    }
}

#[derive(Debug)]
pub struct Func {
    pub name: Symbol,
    pub args: Vec<(Symbol, Type)>,
    pub body: Vec<Stmt>,
    pub returns: Type,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Return(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Call(Box<Expr>, Vec<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
}
