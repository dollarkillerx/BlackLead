use super::*;
use tokens::Number;

#[cfg(test)]
mod test {
    use super::*;
    use tokens::{Op, Expr};
    use extract::{extract_digits_plus, extract_digits_pro, extract_op, extract_whitespace};

    #[test]
    fn hello_world() {
        println!("Hello World");
    }

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123))
    }

    #[test]
    fn parse_op() {
        assert_eq!(Op::new("+"), Op::Add);
        assert_eq!(Op::new("-"), Op::Sub);
        assert_eq!(Op::new("*"), Op::Mul);
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract::extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract::extract_digits_pro("10-20"), ("-20", "10"))
    }

    #[test]
    fn do_not_extract_anything_from_empty_input() {
        assert_eq!(extract_digits_pro(""), ("", ""));
        assert_eq!(extract_digits_pro("100"), ("", "100"));

        assert_eq!(extract_digits_plus(""), ("", ""));
        assert_eq!(extract_digits_plus("100"), ("", "100"));

        let (s, lhs) = extract_digits_plus("1+2");
        println!("{:?}", lhs);
        let (s, lhs) = extract_digits_plus(s);
        println!("{:?} {}", lhs, s);
    }

    #[test]
    fn extract_op_test() {
        assert_eq!(extract_op("+2"), ("2", "+"));
        assert_eq!(extract_op("-20"), ("20", "-"));
        assert_eq!(extract_op("*3"), ("3", "*"));
        assert_eq!(extract_op("/9"), ("9", "/"));
    }

    #[test]
    fn parse_one_plus_two() {
        // assert_eq!(
        //     Expr::new("1+2"),
        //     Expr {
        //         lhs: Number(1),
        //         rhs: Number(2),
        //         op: Op::Add,
        //     },
        // )

        assert_eq!(
            Expr::new("   1   +   2   "),
            (
                "",
                Expr {
                    lhs: Number(1),
                    rhs: Number(2),
                    op: Op::Add,
                },
            )
        )
    }

    #[test]
    fn extract_spaces() {
        println!("{:?}", extract_whitespace("   1 "));
        println!("{:?}", extract_whitespace("   "));
    }
}