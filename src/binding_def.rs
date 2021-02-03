use crate::tokens::Expr;
use crate::extract::extract_whitespace;

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    name: String,
    val: Expr,
}

impl BindingDef {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, _) = extract_whitespace(s);
        let s = if s.starts_with("let")
        {
            &s[3..]
        } else {
            panic!("expected let")
        };

        let (s, _) = extract_whitespace(s);

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Number, Op};

    #[test]
    fn parse_binding_def() {
        assert_eq!(BindingDef::new("let a = 10 / 2"), (
            "",
            BindingDef {
                name: "a".to_string(),
                val: Expr {
                    lhs: Number(10),
                    rhs: Number(2),
                    op: Op::Div,
                },
            }
        ))
    }
}