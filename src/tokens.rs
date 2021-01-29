use super::*;
use extract::{extract_op, extract_digits_plus};

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("bad operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}

impl Expr {
    pub fn new(s: &str) -> (&str, Self) {
        let (s, lhs) = extract_digits_plus(s);
        let lhs = Number::new(lhs);

        let (s, op) = extract_op(s);
        let op = Op::new(op);

        let (s, rhs) = extract_digits_plus(s);
        let rhs = Number::new(rhs);

        (s, Self {
            lhs,
            rhs,
            op,
        })
    }
}