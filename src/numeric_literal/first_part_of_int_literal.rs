use numeric_literal::maybe_float_literal::maybe_float_literal;
use numeric_literal::start_from_dot::start_from_dot;

named!(pub first_part_of_int_literal<&[u8]>,
    alt!(
        maybe_float_literal |
        start_from_dot
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{ErrorKind};

    #[test]
    fn test_first_part_of_int_literal_success_0() {
        let res = first_part_of_int_literal(".0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b".0"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_1() {
        let res = first_part_of_int_literal(".012".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b".012"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_2() {
        let res = first_part_of_int_literal(".012ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b".012"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_3() {
        let res = first_part_of_int_literal(".42ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b".42"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_4() {
        let res = first_part_of_int_literal("0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"0"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_5() {
        let res = first_part_of_int_literal("012".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"012"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_6() {
        let res = first_part_of_int_literal("012ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"012"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_7() {
        let res = first_part_of_int_literal("42ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"42"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_8() {
        let res = first_part_of_int_literal("42.0ab".as_bytes()).unwrap();
        assert_eq!(res, (&b"ab"[..], &b"42.0"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_9() {
        let res = first_part_of_int_literal("42.0".as_bytes()).unwrap();
        assert_eq!(res, (&b""[..], &b"42.0"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_10() {
        let res = first_part_of_int_literal("42.".as_bytes()).unwrap();
        assert_eq!(res, (&b"."[..], &b"42"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_success_11() {
        let res = first_part_of_int_literal("42.a".as_bytes()).unwrap();
        assert_eq!(res, (&b".a"[..], &b"42"[..]));
    }

    #[test]
    fn test_first_part_of_int_literal_error_0() {
        let res = first_part_of_int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_1() {
        let res = first_part_of_int_literal(".a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_2() {
        let res = first_part_of_int_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_3() {
        let res = first_part_of_int_literal(".".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_4() {
        let res = first_part_of_int_literal("".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }

    #[test]
    fn test_first_part_of_int_literal_error_5() {
        let res = first_part_of_int_literal("a".as_bytes()).unwrap_err();
        assert_eq!(res, ErrorKind::Alt);
    }
}