

const ALPHABET: [char;62] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',];
const MOD: u64 = 56800235584; //62**6

fn num_to_string(n: u64){
	
}

pub fn hash_file(file: &[u8]) -> String {
    blake3::hash(file).to_string();
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
