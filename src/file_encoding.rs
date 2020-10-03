use std::collections::HashMap;

const ALPHABET: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];
const LEN: usize = 5;
pub const MOD: usize = 916132832; //62**5

#[derive(Default)]
pub struct FileHasher {
    hm: HashMap<blake3::Hash, usize>,
    cnt: usize,
}
impl FileHasher {
    fn num_to_string(mut n: usize) -> String {
        let mut res = String::with_capacity(LEN);
        for _ in 0..LEN {
            res.push(ALPHABET[n % ALPHABET.len()]);
            n /= ALPHABET.len();
        }
        res
    }
    pub fn new() -> Self {
        FileHasher::default()
    }
    pub fn with_cnt(cnt: usize) -> Self {
        FileHasher {
            hm: HashMap::new(),
            cnt,
        }
    }
    pub fn get_cnt(&self) -> usize {
        self.cnt
    }
    pub fn hash_file(&mut self, file: &[u8]) -> String {
        let h = blake3::hash(file);
        if let Some(x) = self.hm.get(&h) {
            FileHasher::num_to_string(*x)
        } else {
            let x = self.cnt;
            self.cnt = (self.cnt + 1) % MOD;
            self.hm.insert(h, x);
            FileHasher::num_to_string(x)
        }
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
    fn test_different_inputs() {
        let mut hasher = FileHasher::default();
        assert!(
            hasher.hash_file(&[0, 7, 8, 2, 6, 4, 59, 38, 78, 159, 48])
                != hasher.hash_file(&[0, 7, 8, 2, 6, 4, 59, 38])
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
