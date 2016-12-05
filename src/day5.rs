//! Brute forcing simple passwords

extern crate crypto;

use std;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn hash_for_suffix(door: &str, hasher: &mut Md5, suffix: i64) -> String {
    hasher.reset();
    hasher.input_str(door);
    hasher.input_str(&suffix.to_string());
    hasher.result_str()
}

pub fn get_password(door: &str) -> String {
    let mut hasher = Md5::new();
    (0..std::i64::MAX).filter_map(|i| {
        let hash = hash_for_suffix(door, &mut hasher, i);
        if hash.starts_with("00000") {
            println!("found hash {} at {}", hash, i);
            hash.chars().nth(5)
        } else {
            None
        }
    }).take(8).collect::<String>()
}

#[cfg(test)]
mod test {

    use super::*;

    use super::crypto::md5::Md5;

    #[test]
    fn char_at() {
        assert_eq!(Some('2'), "124".chars().nth(1));
    }

    #[test]
    fn hash_for_known_suffixes() {
        let mut hasher = Md5::new();
        assert_eq!(Some('1'), hash_for_suffix("abc", &mut hasher, 3231929).chars().nth(5));
        assert!(hash_for_suffix("abc", &mut hasher, 5017308).starts_with("000008f82"));
    }

    #[test]
    fn example_password_decrypt() {
        assert_eq!("18f47a30", get_password("abc"));
    }
}
