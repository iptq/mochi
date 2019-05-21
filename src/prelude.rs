use symbol::Symbol;

use crate::ast::Type;
use crate::env::Environment;

pub fn load_prelude(env: &mut Environment<Symbol, Type>) {
    env.insert(Symbol::from("int"), Type::Int);
}
