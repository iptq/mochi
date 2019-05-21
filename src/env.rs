use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::Hash;

pub struct Environment<K, V>(Vec<HashMap<K, V>>);

impl<K: Eq + Hash + Debug, V: Debug> Debug for Environment<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<K: Eq + Hash, V> Environment<K, V> {
    pub fn new() -> Self {
        Environment(vec![HashMap::new()])
    }

    pub fn push_scope(&mut self) {
        self.0.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.0.pop();
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.0
            .last_mut()
            .expect("should not happen")
            .insert(key, value);
    }

    pub fn lookup(&self, key: K) -> Option<&V> {
        for scope in self.0.iter().rev() {
            if let Some(value) = scope.get(&key) {
                return Some(value);
            }
        }
        None
    }
}
