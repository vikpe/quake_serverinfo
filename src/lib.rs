use std::collections::HashMap;

const DELIMITER: char = '\\';

pub fn to_hashmap(serverinfo: &str) -> HashMap<String, String> {
    let info = clean(serverinfo);
    let mut iter = info.split(DELIMITER).map(|v| v.to_string());
    let mut result = HashMap::new();

    while let Some(key) = iter.next() {
        let value = iter.next().unwrap_or("".to_string());
        result.insert(key, value);
    }

    result
}

fn clean(serverinfo: &str) -> String {
    String::from(serverinfo)
        .trim_end_matches(r#"\n"#)
        .replace(r#"\\"#, &DELIMITER.to_string())
        .replace('"', "")
        .trim()
        .trim_matches(DELIMITER)
        .to_string()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_hashmap() {
        // unbalanced string
        {
            let result = to_hashmap(r#"\maxfps\77\matchtag\"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), Some(&"".to_string()));
        }

        // valid string
        {
            let result = to_hashmap(r#"\maxfps\77\matchtag\kombat\epoch\123456"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), Some(&"kombat".to_string()));
            assert_eq!(result.get("epoch"), Some(&"123456".to_string()));
            assert_eq!(result.get("INVALID_KEY"), None);
        }
    }

    #[test]
    fn test_clean() {
        assert_eq!(clean(r#"\\maxfps\\77\\"#), r#"maxfps\77"#); // double slashes
        assert_eq!(clean(r#"\needpass\1\\\n"#), r#"needpass\1"#); // trailing new lines
        assert_eq!(clean(r#"\max"fps\77""#), r#"maxfps\77"#); // quotes
    }
}
