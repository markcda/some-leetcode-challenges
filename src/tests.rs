#[cfg(test)]
mod tests {
    use crate::tools::parse;
    use crate::tools::parse_mul;

    #[test]
    fn test_parse_single() {
        assert_eq!(parse::<u8>("0"), Ok(0u8));
        assert_eq!(parse::<i32>("1"), Ok(1i32));
        assert!(parse::<u8>("-1").is_err());
        assert!(parse::<i32>("a").is_err());
        assert!(parse::<i32>("-").is_err());
    }
    
    #[test]
    fn test_parse_multiple() {
        assert_eq!(parse_mul::<u8>("7 11 8 6 3 8 9", None), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
        assert_eq!(parse_mul::<u8>("7,11,8,6,3,8,9", Some(",".into())), Ok(vec![7u8, 11u8, 8u8, 6u8, 3u8, 8u8, 9u8]));
    }
}
