
use std::collections::HashSet;

fn all_unique_chars(s: &str) -> bool {
    let mut seen = HashSet::new();

    for c in s.chars() {
        if seen.contains(&c) {
            return false;
        }
        else {
            seen.insert(c.clone());
        }
    }
    return true;
}

#[test]
fn test_all_unique_chars() {
    assert!(all_unique_chars("abcdefg"));
    assert!(all_unique_chars("aaa"))
}

