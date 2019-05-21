use symbol::Symbol;

#[derive(Debug)]
pub struct Program(pub Vec<Decl>);

#[derive(Debug)]
pub enum Decl {
    Func(Func),
}

#[derive(Debug)]
pub struct Func {
    pub name: Symbol,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Int(i64),
}
