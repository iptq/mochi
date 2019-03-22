pub enum Expr {
    Name(String),
    Temp(),
    ArithOp(ArithOp, Box<Expr>, Box<Expr>),
}

pub type Label = String;

pub enum Stmt {
    Move(Expr, Expr),
    Expr(Expr),
    Jump(Expr),
    CondJump(CompOp, Expr, Expr, Label, Label),
    Seq(Box<Stmt>, Box<Stmt>),
    Label(Label),
}

pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
    LShift,
    RShift,
    ARShift,
    And,
    Or,
    Xor,
}

pub enum CompOp {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    ULt,
    UGt,
    ULte,
    UGte,
}
