use super::*;

// 1 + 1 => 1,+1
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    (&s[1..], &s[0..1])
}

// 10 + 12 => 10,+12
pub(crate) fn extract_digits_pro(s: &str) -> (&str, &str) {
    let mut digits_end = 0;

    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            digits_end = idx + 1;
        } else {
            break;
        }
    }

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}


pub(crate) fn extract_digits_plus(s: &str) -> (&str, &str) {
    let digits_end = s
        .char_indices()
        .find_map(|(idx, c)|
            if c.is_ascii_digit() { None } else { Some(idx) }
        )
        .unwrap_or_else(|| s.len());

    // println!("s: {}, idx: {}  len: {}", s, digits_end,s.len());
    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    let whitespace_end = s
        .char_indices()
        .find_map(|(idx, c)| {
            println!("idx: {} c : {}", idx, c);
            if c == ' ' { None } else { Some(idx) }
        })
        .unwrap_or_else(|| s.len());

    (&s[whitespace_end..], &s[..whitespace_end])
}

pub(crate) fn task_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end = s
        .char_indices()
        .find_map(|(idx, r)| {
            if !accept(r) {
                Some(idx)
            } else {
                None
            }
        })
        .unwrap_or_else(|| s.len());

    (&s[end..], &s[..end])
}