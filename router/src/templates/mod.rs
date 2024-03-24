pub mod render;
pub mod operations;
pub mod text_parse;

#[cfg(test)]
mod tests {
    use crate::templates::text_parse::{find_between, remove_between};
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

}
