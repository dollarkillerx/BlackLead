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


// 数字识别
pub(crate) fn extract_digits_plus(s: &str) -> (&str, &str) {
    task_while(|s| { if s.is_ascii_digit() { true } else { false } }, s)
}

// 操作符识别
pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

// 空格消散
pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    task_while(|s| { if s == ' ' { true } else { false } }, s)
}

// 变量 识别
pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    // 是否以字母开头
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false);

    if input_starts_with_alphabetic {
        task_while(|c| c.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
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
        .unwrap_or_else({
            || s.len()
        });

    (&s[end..], &s[..end])
}

