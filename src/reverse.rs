#![allow(unused)]

fn reverse_a(s: &str) -> String {
    s.to_string().chars().rev().collect()
}

fn reverse_b(s: &str) -> String {
    let mut rev_string = vec![];
    let chars = s.chars().collect::<Vec<_>>();

    for n in (0..s.len()).rev() {
        rev_string.push(chars[n])
    }

    rev_string.iter().collect()
}

#[test]
fn test_reverse_a() {
    assert_eq!(reverse_a("Hello"), "olleH")
}

#[test]
fn test_reverse_b() {
    assert_eq!(reverse_b("Hello"), "olleH")
}
