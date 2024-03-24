use std::collections::HashMap;

#[derive(Debug)]
pub struct Match {
    pub from: usize,
    pub to: usize,
    pub content: String,
}

impl Match {
    fn new(from: usize, to: usize, content: String) -> Match {
        Match {
            from,
            to,
            content,
        }
    }
}


fn get_all_between(content: String, from: &str, to: &str) -> Vec<Match> {
    
    let mut results = Vec::new();
    let mut content = content;
    let from_len = from.len();
    let to_len = to.len();
    let mut stripped_away = 0;
    loop {
        let from_index = match content.find(from) {
            Some(index) => index,
            None => break,
        };
        let after_from = &content[(from_index+from_len)..];
        let to_index = match after_from.find(to) {
            Some(index) => index,
            None => break,
        }; 
        let contained_content = after_from[..to_index].to_string(); 

        let from_index_accum = from_index + stripped_away; 
        stripped_away += from_index_accum + from_len; 

        let last_pattern_index_accum = to_index + to_len - 1 + stripped_away; 
        
        results.push(Match::new(from_index_accum, last_pattern_index_accum, contained_content));
        content = after_from[(to_index + to_len)..].to_string(); 
    }

    return results;
}

pub fn template(path: &str, context: Option<HashMap<String, String>>) -> String {

    let mut content = std::fs::read_to_string(path).expect("Could not read file");
    let matches = get_all_between(content.clone(), "{{", "}}");
    dbg!(&matches);
    if let Some(context) = context {
        matches.iter().for_each(|m| {
            if let Some(to_replace_with) = context.get(&m.content.trim().to_owned()) {
                dbg!(to_replace_with);
                dbg!(&content[m.from..m.to+1]);
                content.replace_range(m.from..m.to+1, to_replace_with);
            }
        });
    }
    return content;
}

#[cfg(test)]
mod tests {
    use crate::render::get_all_between;
    #[test]
    fn test_get_between_in_one_line_match_w_equal_patterns() {
        let content = String::from("content that | contains patterns |");
        let matches = get_all_between(content, "|", "|");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches.first().unwrap().content, " contains patterns ")
    }

    #[test]
    fn test_get_between_in_two_lines_match_w_equal_patterns() {
        let content = String::from("content that | contains patterns | but this |is \n also content to| get");
        let matches = get_all_between(content, "|", "|");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches.first().unwrap().content, " contains patterns ");
        assert_eq!(matches.get(1).unwrap().content, "is \n also content to");
    }

    #[test]
    fn test_get_between_w_multi_char_pattern() {
        let content = String::from("content that || contains patterns || but this ||is \n also content to|| get");
        let matches = get_all_between(content, "||", "||");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches.first().unwrap().content, " contains patterns ");
        assert_eq!(matches.get(1).unwrap().content, "is \n also content to");
    }

    #[test]
    fn test_different_from_and_to_pattern() { let content = String::from("content that {{ contains patterns }} but this {{is \n also content to}} get");
        let matches = get_all_between(content, "{{", "}}");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches.first().unwrap().content, " contains patterns ");
        assert_eq!(matches.get(1).unwrap().content, "is \n also content to");
    }

}
