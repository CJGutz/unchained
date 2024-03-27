#[derive(Debug)]
pub struct Match {
    pub from: usize,
    pub to: usize,
    pub content: String,
}

/// Not supporting graphemes of multiple code points 
/// can lead to a significant performance improvement.
/// This is due to slicing through `char_indices` being O(n) 
const SUPPORT_UTF8: bool = false;

trait FindGrapheme {
    fn find_grapheme(&self, grapheme: &str) -> Option<usize>;
}

impl FindGrapheme for &str {
    fn find_grapheme(&self, grapheme: &str) -> Option<usize> {
        let mut grapheme_chars = grapheme.chars();
        let mut grapheme_char = match grapheme_chars.next() {
            Some(c) => c,
            None => return None,
        };
        let mut index_match: Option<usize> = None;
        for (i, c) in self.char_indices() {
            if c != grapheme_char {
                grapheme_chars = grapheme.chars();
                grapheme_char = match grapheme_chars.next() {
                    Some(c) => c,
                    None => return None,
                };
                index_match = None;
            }
            if c == grapheme_char {
                if index_match.is_none() {
                    index_match = Some(i);
                }
                grapheme_char = match grapheme_chars.next() {
                    Some(c) => c,
                    None => return index_match,
                };
            }
        }
        None
    }
}

pub fn from_slice(s: &str, begin: usize) -> &str {
    slice(s, begin, len(s))
}

pub fn till_slice(s: &str, end: usize) -> &str {
    slice(s, 0, end)
}

pub fn len(s: &str) -> usize {
    s.chars().count()
}

pub fn slice(s: &str, begin: usize, end: usize) -> &str {
    if !SUPPORT_UTF8 {
        return &s[begin..end];
    }

    if end < begin {
        return "";
    }

    s.char_indices()
        .nth(begin)
        .and_then(|(start_pos, _)| {
            if end >= len(s) {
                return Some(&s[start_pos..]);
            }

            s[start_pos..]
                .char_indices()
                .nth(end - begin)
                .map(|(end_pos, _)| &s[start_pos..start_pos + end_pos])
        })
        .unwrap_or("")
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
    let from_index = match content.find_grapheme(from) {
        Some(index) => index,
        None => return None,
    }; 
    let after_from = from_slice(content, from_index+from.len());
    let to_index = match after_from.find_grapheme(to) {
        Some(index) => index,
        None => return None,
    };  
    let content_inside = till_slice(after_from, to_index);

    Some(Match {
        from: from_index,
        to: to_index + from_index + from.len() + to.len() - 1,
        content: content_inside.to_string()
    })
}


fn min<T>(a: T, b: T) -> T where T: PartialOrd {
    if a < b { a } else { b }
}

fn pattern_index_in_full(full: &str, slice_str: &str, pattern: &str, current_index: usize) -> Option<usize> {
    if slice_str.contains(|c| pattern.contains(c)) {
        let pattern_len = pattern.len();
        let start_check = if current_index < 2*pattern_len + 1 { 0 }
        else { current_index - 2*pattern_len + 1 };
        let end_check = min(full.len(), current_index + pattern_len);
        
        let pattern_index = slice(full, start_check, end_check).find_grapheme(pattern);
        return match pattern_index {
            Some(index) => Some(index + start_check),
            None => None,
        };
    }
    None
}

/// Expects opening to be the same length as closing.
pub fn between_connected_patterns(content: &str, opening: &str, closing: &str) -> Option<Match> {

    if opening == closing {
        return find_between(content, opening, closing);
    }
    let opening_len = opening.len();

    let mut open_patterns = 0;
    let mut i = opening_len;
    let mut open_pattern_index: Option<usize> = None;
    while i <= content.len() {
        let slice_check = slice(content, i-opening_len, i);
        let open_index = pattern_index_in_full(content, slice_check, opening, i);
        let close_index = pattern_index_in_full(content, slice_check, closing, i);

        if open_index.is_some() {
            open_patterns += 1;
            if open_pattern_index.is_none() {
                open_pattern_index = open_index;
            }
        }
        if let Some(close_index) = close_index {
            open_patterns -= 1;
            if open_patterns == 0 && open_pattern_index.is_some() {
                return Some(Match {
                    from: open_pattern_index.unwrap(),
                    to: close_index + opening_len - 1,
                    content: slice(content, open_pattern_index.unwrap()+opening_len, close_index).to_string(),
                });
            }
        }
        i += opening_len*2 - 1;
        if i > content.len() && opening_len > 1 {
            i -= 1; 
        }
    };

    None
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

#[cfg(test)]
mod tests {
    use crate::templates::text_parse::{find_between, remove_between, between_connected_patterns, slice, FindGrapheme};
    #[test]
    fn test_get_between_in_one_line_match_w_equal_patterns() {
        let found = find_between("content that | contains patterns |", "|", "|");
        assert!(found.is_some());
        assert_eq!(found.unwrap().content, " contains patterns ");
    }

    #[test]
    fn test_get_between_in_two_lines_match_w_equal_patterns() {
        let found = find_between("content that | contains patterns | but this |is \n also content to| get", "|", "|");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 33);
    }

    #[test]
    fn test_get_between_w_multi_char_pattern() {
        let found = find_between("content that || contains patterns || but this ||is \n also content to|| get", "||", "||");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 35);
    }

    #[test]
    fn test_assymmetric_pattern() { 
        let found = find_between("content that contains patterns but this {{is \n also content to}} get", "{{", "}}");

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, "is \n also content to");
        assert_eq!(found.from, 40);
        assert_eq!(found.to, 63);
    }

    #[test]
    fn test_no_pattern_found() {
        let found = find_between("content with no pattern", "|", "}");

        assert!(found.is_none());
    }

    #[test]
    fn test_no_to_pattern_found() {
        let found = find_between("content with | no pattern", "|", "}");

        assert!(found.is_none());
    }

    #[test]
    fn test_remove_single_asymmetric_pattern() {
        let res = remove_between("A string with [a pattern] found", "[", "]");
        assert!(res.is_some());
        let (changed_content, inside_pattern) = res.unwrap();
        assert_eq!(changed_content, "A string with  found".to_string());
        assert_eq!(inside_pattern, "a pattern".to_string());
    }

    #[test]
    fn test_singular_opening_pattern() {
        let content = "content { with a pattern and { another pattern }  }";
        let res = between_connected_patterns(content, "{", "}");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, " with a pattern and { another pattern }  ");
        assert_eq!(res.from, 8);
        assert_eq!(res.to, 50);
    }

    #[test]
    fn test_multi_char_opening_pattern() {
        let content = "content {* with a pattern and {* another pattern *}  *}";
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, " with a pattern and {* another pattern *}  ");
        assert_eq!(res.from, 8);
        assert_eq!(res.to, 54);
    }

    #[test]
    fn test_no_closing_pattern() {
        let content = "content {* with a pattern and {* another pattern *}";
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_none());
    }

    #[test]
    fn test_closing_bracket_first() {
        let content = "content *} with a pattern and {* another pattern";
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_none());
    }

    #[test]
    fn test_empty_between_opening_and_closing() {
        let content = "{**}";
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, "");
        assert_eq!(res.from, 0);
        assert_eq!(res.to, 3);
    }

    #[test]
    fn test_closing_bracket_first_before_valid_patterns() {
        let content = "content *} with a pattern and {* another pattern {* *}  *}";
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, " another pattern {* ");
        assert_eq!(res.from, 30);
        assert_eq!(res.to, 53);
    }

    #[test]
    fn test_equal_open_and_closing_pattern() {
        let content = "content | with a pattern and | another pattern |";
        let res = between_connected_patterns(content, "|", "|");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, " with a pattern and ");
        assert_eq!(res.from, 8);
        assert_eq!(res.to, 29);
    }

    #[test]
    fn test_equal_open_and_closing__() {
        let content = "content | with a pattern and | another pattern |";
        let res = between_connected_patterns(content, "|", "|");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.content, " with a pattern and ");
        assert_eq!(res.from, 8);
        assert_eq!(res.to, 29);
    }

    #[test]
    fn slice_aa() {
        let content = "cåüååontent";
        let res = slice(content, 0, 4);
        assert_eq!(res, "cåüå");
    }

    #[test]
    fn find_grapheme_through_utf8() {
        let content = "cåüååontent";
        let res = content.find_grapheme("å");
        assert_eq!(res, Some(1));
    }
}
