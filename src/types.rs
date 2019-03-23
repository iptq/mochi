#[derive(Debug)]
pub enum Type {
    Unit,
    Bool,
    Int32,
    Sint32,
    Int64,
    Sint64,
    Record(Vec<(String, Type)>),
}

#[derive(Debug)]
pub enum Constraint {}

#[derive(Debug)]
pub enum Kind {
    Type(Type),
    ParamType(Box<Kind>, Box<Kind>),
}
