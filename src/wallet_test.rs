#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_tokens() {
        receive_tokens(100);
        assert_eq!(get_balance(), 100);
    }

    #[test]
    fn test_send_tokens() {
        receive_tokens(200);
        let result = send_tokens("receiver".to_string(), 50);
        assert!(result.is_ok());
        assert_eq!(get_balance(), 150);
    }
}