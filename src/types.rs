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
