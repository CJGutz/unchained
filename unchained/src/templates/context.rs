use std::collections::HashMap;


#[derive(Debug, Clone)]
pub enum Primitive {
    Str(String),
    Num(isize),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub enum ContextTree {
    Leaf(Primitive),
    Array(Box<Vec<ContextTree>>),
    Branch(Box<HashMap<String, ContextTree>>)
}

pub type ContextMap = HashMap<String, ContextTree>;

impl Default for ContextTree {
    fn default() -> Self {
        Self::Branch(Box::new(HashMap::new()))
    }
}

impl ContextTree {
    /// Retrieves a context element from the context tree
    /// If the type is not a branch / hashmap, it returns None
    pub fn get_from_branch(self: &Self, key: &str) -> Option<&Self> {
        match self {
            Self::Branch(map) => map.get(key),
            _ => None,
        }
    }
}

