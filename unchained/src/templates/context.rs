use std::{collections::HashMap, fmt::Display};

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
        Self::Branch(Box::default())
    }
}

impl ContextTree {
    /// Retrieves a context element from the context tree
    /// If the type is not a branch / hashmap, it returns None
    pub fn get_from_branch(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Branch(map) => map.get(key),
            _ => None,
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Str(s) => f.write_str(s),
            Primitive::Num(n) => f.write_str(&n.to_string()),
            Primitive::Bool(b) => f.write_str(&b.to_string()),
        }
    }
}

impl Display for ContextTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let children = match self {
            ContextTree::Leaf(p) => p.to_string(),
            ContextTree::Array(a) => format!(
                "[ \n\t{}\n ]",
                a.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",\n\t")
            ),
            ContextTree::Slot(p) => p.to_string(),
            ContextTree::Branch(b) => format!(
                "{{ {} }}",
                b.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        };
        f.write_str(&children)
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
