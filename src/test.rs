use super::*;
use tokens::Number;

#[cfg(test)]
mod test {
    use super::*;
    use tokens::Op;

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
}