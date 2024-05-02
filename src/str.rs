use std::collections::HashMap;

pub const DELIMITER: char = '\\';

pub fn to_hashmap(serverinfo: &str) -> HashMap<String, String> {
    let str = clean(serverinfo);
    let mut iter = str
        .trim_matches(DELIMITER)
        .split(DELIMITER)
        .map(|v| v.to_string());
    let mut result = HashMap::new();

    while let Some(key) = iter.next() {
        if let Some(value) = iter.next() {
            result.insert(key, value);
        }
    }

    result
}

pub fn clean(serverinfo: &str) -> String {
    serverinfo
        .trim()
        .trim_end_matches(r#"\n"#)
        .replace(r#"\\"#, &DELIMITER.to_string())
        .replace('"', "")
        .trim_end_matches(DELIMITER)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_hashmap() {
        // unbalanced string
        {
            let result = to_hashmap(r#"\maxfps\77"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), None);
        }

        // valid string
        {
            let result = to_hashmap(r#"\maxfps\77\matchtag\kombat\epoch\123456"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), Some(&"kombat".to_string()));
            assert_eq!(result.get("epoch"), Some(&"123456".to_string()));

            assert_eq!(result.get("MISSING_KEY"), None);
        }
    }

    #[test]
    fn test_clean() {
        assert_eq!(clean(r#"\\maxfps\\77"#), r#"\maxfps\77"#); // double slashes
        assert_eq!(clean(r#"\maxfps\77\"#), r#"\maxfps\77"#); // trailing slash
        assert_eq!(clean(r#" \needpass\1\\\n "#), r#"\needpass\1"#); // whitespace
        assert_eq!(clean(r#"\max"fps\77""#), r#"\maxfps\77"#); // quotes
    }
}
