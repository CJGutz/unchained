use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub enum Primitive {
    Str(String),
    Num(isize),
    Bool(bool),
}

/// Enum for context information sent to the templates
/// Use for primitives (str/num/bool), arrays, and maps
///
/// `Slot` contains identifier for where component children are inserted.
/// See [component operation](crate::templates::operations::get_template_operation)
///
/// [ContextTree] implements the [From] trait for [isize], [&str], [String], [bool],
/// arrays, and maps (from arrays of tuples or [HashMap]s)
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

// ==================================
// From implementation for ContextTree
// ==================================

impl From<isize> for ContextTree {
    fn from(value: isize) -> Self {
        ContextTree::Leaf(Primitive::Num(value))
    }
}

impl From<String> for ContextTree {
    fn from(value: String) -> Self {
        ContextTree::Leaf(Primitive::Str(value))
    }
}

impl From<&str> for ContextTree {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<bool> for ContextTree {
    fn from(value: bool) -> Self {
        ContextTree::Leaf(Primitive::Bool(value))
    }
}

impl<T: Into<ContextTree>> From<Vec<T>> for ContextTree {
    fn from(value: Vec<T>) -> Self {
        ContextTree::Array(Box::new(value.into_iter().map(|v| v.into()).collect()))
    }
}

impl<V: Into<ContextTree>> From<HashMap<String, V>> for ContextTree {
    fn from(value: HashMap<String, V>) -> Self {
        ContextTree::Branch(Box::new(
            value.into_iter().map(|(k, v)| (k, v.into())).collect(),
        ))
    }
}

impl<V: Into<ContextTree>, const N: usize> From<[V; N]> for ContextTree {
    fn from(value: [V; N]) -> Self {
        ContextTree::Array(Box::new(value.into_iter().map(|v| v.into()).collect()))
    }
}

impl<V: Into<ContextTree>, const N: usize> From<[(&str, V); N]> for ContextTree {
    fn from(value: [(&str, V); N]) -> Self {
        ContextTree::Branch(Box::new(
            value
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        ))
    }
}

// ==================================
// Display implementation for ContextTree
// ==================================

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

/// Convert a vector of context trees to an array context tree
/// __Deprecated__: Use [ContextTree]::from instead
#[deprecated(note = "Use [ContextTree]::from instead")]
pub fn ctx_vec(parameters: Vec<ContextTree>) -> ContextTree {
    ContextTree::Array(Box::new(parameters))
}

/// Convert an array of key-value pairs of context trees to a branch context tree
/// __Deprecated__: Use [ContextTree]::from instead
#[deprecated(note = "Use [ContextTree]::from instead")]
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

/// Convert a string slice to a context tree
/// __Deprecated__: Use [ContextTree]::from instead
#[deprecated(note = "Use [ContextTree]::from instead")]
pub fn ctx_str(str: &str) -> ContextTree {
    ContextTree::Leaf(Primitive::Str(str.to_string()))
}
