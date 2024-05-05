use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct Match {
    pub from: usize,
    pub to: usize,
    pub content: String,
}

impl Match {
    pub fn push(&mut self, length: usize) {
        self.from += length;
        self.to += length;
    }
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
    let after_from = &content[from_index + from.len()..];
    let to_index = match after_from.find(to) {
        Some(index) => index,
        None => return None,
    };
    let content_inside = &after_from[..to_index];

    Some(Match {
        from: from_index,
        to: to_index + from_index + from.len() + to.len() - 1,
        content: content_inside.to_string(),
    })
}

/// Iterates through iterator and tries to match it with a str slice.
/// Returns true if they have equal characters and how many items were
/// iterated through.
fn chars_equal_for_length(
    iter: &mut dyn Iterator<Item = char>,
    to_match: &mut dyn Iterator<Item = char>,
) -> (bool, usize) {
    let zipped = iter.zip(to_match);
    let mut index = 0;
    for ch in zipped {
        index += ch.0.len_utf8();
        if ch.0 != ch.1 {
            return (false, index);
        }
    }
    (true, index)
}

pub fn between_connected_patterns(
    content: &str,
    opening_pattern: &str,
    closing_pattern: &str,
) -> Option<Match> {
    if opening_pattern == closing_pattern {
        return find_between(content, opening_pattern, closing_pattern);
    }

    let mut open_parens = 0;
    let mut first_index: Option<usize> = None;
    let mut index = 0;
    let opening_first_char = opening_pattern.chars().next();
    let closing_first_char = closing_pattern.chars().next();
    if opening_first_char.is_none() || closing_first_char.is_none() {
        return None;
    }

    let mut chars = content.chars();
    while let Some(ch) = chars.next() {
        match ch {
            _ if ch == opening_first_char.unwrap() => {
                let (matches, index_to_add) = chars_equal_for_length(
                    chars.clone().borrow_mut(),
                    opening_pattern.chars().skip(1).borrow_mut(),
                );
                if index_to_add > 0 {
                    chars.nth(index_to_add - 1);
                }
                index += index_to_add;
                if matches {
                    open_parens += 1;
                    if first_index.is_none() {
                        first_index = Some(index + 1 - opening_pattern.len());
                    }
                }
            }
            _ if ch == closing_first_char.unwrap() && first_index.is_some() => {
                let (matches, index_to_add) = chars_equal_for_length(
                    chars.clone().borrow_mut(),
                    closing_pattern.chars().skip(1).borrow_mut(),
                );
                if index_to_add > 0 {
                    chars.nth(index_to_add - 1);
                }
                index += index_to_add;
                if matches {
                    open_parens -= 1;
                    if open_parens == 0 {
                        let first_index = first_index.unwrap();
                        let m = Some(Match {
                            from: first_index,
                            to: index,
                            content: content[first_index + opening_pattern.len()
                                ..index - closing_pattern.len() + 1]
                                .to_string(),
                        });
                        return m;
                    }
                }
            }
            _ => {}
        }
        index += ch.len_utf8();
    }

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
    let find = find_between(content, from, to)?;

    let mut content = content.to_string();
    content.replace_range(find.from..find.to + 1, "");
    Some((content, find.content))
}

#[cfg(test)]
mod tests {
    use crate::templates::text_parse::{between_connected_patterns, find_between, remove_between};
    #[test]
    fn test_get_between_in_one_line_match_w_equal_patterns() {
        let found = find_between("content that | contains patterns |", "|", "|");
        assert!(found.is_some());
        assert_eq!(found.unwrap().content, " contains patterns ");
    }

    #[test]
    fn test_get_between_in_two_lines_match_w_equal_patterns() {
        let found = find_between(
            "content that | contains patterns | but this |is \n also content to| get",
            "|",
            "|",
        );

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 33);
    }

    #[test]
    fn test_get_between_w_multi_char_pattern() {
        let found = find_between(
            "content that || contains patterns || but this ||is \n also content to|| get",
            "||",
            "||",
        );

        assert!(found.is_some());
        let found = found.unwrap();
        assert_eq!(found.content, " contains patterns ");
        assert_eq!(found.from, 13);
        assert_eq!(found.to, 35);
    }

    #[test]
    fn test_assymmetric_pattern() {
        let found = find_between(
            "content that contains patterns but this {{is \n also content to}} get",
            "{{",
            "}}",
        );

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
        assert_eq!(res.content, " another pattern {* *}  ");
        assert_eq!(res.from, 30);
        assert_eq!(res.to, 57);
    }

    #[test]
    fn test_with_html_content_operation() {
        let content = r#"{* component me.html {
            <div class="my-class">
                <h1>{{ Me }}</h1>
                <p>{* Some text about me *}</p>
            </div>
        } *}"#;
        let res = between_connected_patterns(content, "{*", "*}");
        assert!(res.is_some());
        let res = res.unwrap();
        assert_eq!(res.from, 0);
        assert_eq!(res.to, 170);
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
    fn find_replace_with_several_code_points() {
        let mut content = String::from("åüåå[cåüååontent]");
        let res1 = content.find("[").unwrap();
        let res2 = content.find("]").unwrap();
        content.replace_range(res1 + 1..res2, "bruh");
        assert_eq!(content, "åüåå[bruh]");
    }
}
