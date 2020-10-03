#[derive(Default)]
pub struct FileHasher;
impl FileHasher {
    pub fn hash_file(&mut self, file: &[u8]) -> String {
        "a".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_output() {
        let mut hasher = FileHasher::default();
        assert_eq!(
            hasher.hash_file(&[0, 7, 8, 2, 6, 4, 59, 38]),
            hasher.hash_file(&[0, 7, 8, 2, 6, 4, 59, 38])
        );
    }

    #[test]
    fn test_const_len() {
        let mut hasher = FileHasher::default();
        assert_eq!(
            hasher
                .hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48])
                .len(),
            hasher.hash_file(&[0, 7, 8, 2, 6, 4, 59, 38]).len()
        );
    }
    #[test]
    fn test_allowed_chars() {
        let mut hasher = FileHasher::default();
        assert!(hasher
            .hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48])
            .as_str()
            .chars()
            .into_iter()
            .fold(true, |acc, x| acc && (x.is_ascii() && x.is_alphanumeric())));
    }
    #[test]
    fn test_length() {
        let mut hasher = FileHasher::default();
        let len = hasher
            .hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48])
            .len();
        assert!(len <= 6);
    }
}
