fn hash(_s: String, m: String) -> (String) {
    m
}

#[cfg(test)]
mod tests {
    #[test]
    fn case_empty() {
        use hash;

        let expect_hmac = "b613679a0814d9ec772f95d778c35fc5ff1697c493715653c6c712144292c5ad".to_string();
        let res_hmac = hash("".to_string(), "".to_string());
        assert_eq!(expect_hmac, res_hmac);
    }

    #[test]
    fn case_message() {
        use hash;

        let expect_hmac = "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8".to_string();
        let res_hmac = hash("key".to_string(), "The quick brown fox jumps over the lazy dog".to_string());
        assert_eq!(expect_hmac, res_hmac);
    }

}
