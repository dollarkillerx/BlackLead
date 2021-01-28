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