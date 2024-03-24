
#[derive(Debug)]
pub struct Match {
    pub from: usize,
    pub to: usize,
    pub content: String,
}


/// Finds the first match of from and to in the content.
/// Example:
/// ```
/// let found = find_between("content that | contains patterns | but", "|", "|");
///
/// assert!(found.is_some());
/// let found = found.unwrap();
/// assert_eq!(found.content, " contains patterns ");
/// assert_eq!(found.from, 13);
/// assert_eq!(found.to, 33);
/// ```
pub fn find_between(content: &str, from: &str, to: &str) -> Option<Match> {
    let from_index = match content.find(from) {
        Some(index) => index,
        None => return None,
    }; 
    let after_from = &content[(from_index+from.len())..]; 
    let to_index = match after_from.find(to) {
        Some(index) => index,
        None => return None,
    };  
    let content_inside = after_from[..to_index].to_string(); 

    Some(Match {
        from: from_index,
        to: to_index + from_index + from.len() + to.len() - 1,
        content: content_inside
    })
}

/// Removes everything inclusively between the first occurrences
/// of `from` and `to` and returns it exclusive of the patterns.
/// Example:
/// ```
/// let content = String::from("A string with [a pattern] found");
/// let res = remove_between(content, "[", "]");
/// assert!(res.is_some());
/// let (changed_content, inside_pattern) = res.unwrap();
/// assert_eq!(changed_content, "A string with  found".to_string());
/// assert_eq!(inside_pattern, "a pattern".to_string());
/// ```
pub fn remove_between(content: &str, from: &str, to: &str) -> Option<(String, String)> {
    let find = find_between(content, from, to);
    if find.is_none() { return None }
    let find = find.unwrap();

    let mut content = content.to_string();
    content.replace_range(find.from..find.to+1, "");
    return Some((content, find.content));
}
