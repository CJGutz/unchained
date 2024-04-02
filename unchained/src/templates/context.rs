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
    Branch(Box<HashMap<String, ContextTree>>),
    Slot(Primitive),
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

impl ToString for Primitive {
    fn to_string(&self) -> String {
        match self {
            Primitive::Str(a) => a.to_string(),
            Primitive::Num(a) => a.to_string(),
            Primitive::Bool(a) => a.to_string(),
        }
    }
}

pub fn ctx_vec(parameters: Vec<ContextTree>) -> ContextTree {
    ContextTree::Array(Box::new(parameters))
}

pub fn ctx_map<const N: usize>(array: [(&str, ContextTree); N]) -> ContextTree {
    let mut string_array: Vec<(String, ContextTree)> = Vec::with_capacity(N);
    for (s, c) in array.iter() {
        string_array.push((s.to_string(), c.clone()));
    }
    let map: HashMap<String, ContextTree> = array
        .iter()
        .map(|(s, c)| (s.to_string(), c.clone()))
        .collect();
    ContextTree::Branch(Box::new(map))
}

pub fn ctx_str(str: &str) -> ContextTree {
    ContextTree::Leaf(Primitive::Str(str.to_string()))
}
