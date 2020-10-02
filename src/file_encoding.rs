fn hash_file(file: &[u8]) -> String {
    "a".to_owned()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_same_output() {
        assert_eq!(
            hash_file(&[0, 7, 8, 2, 6, 4, 59, 38]),
            hash_file(&[0, 7, 8, 2, 6, 4, 59, 38])
        );
    }

    #[test]
    fn test_const_len() {
        assert_eq!(
            hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48]).len(),
            hash_file(&[0, 7, 8, 2, 6, 4, 59, 38]).len()
        );
    }
    #[test]
    fn test_allowed_chars() {
        assert!(hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48])
            .as_str()
            .chars()
            .into_iter()
            .fold(true, |acc, x| acc && (x.is_ascii() && x.is_alphanumeric())));
    }
    #[test]
    fn test_length() {
        let len = hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48]).len();
        assert!(len <= 6);
    }
}
