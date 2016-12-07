//! Brute forcing simple passwords

extern crate crypto;

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
    (0..)
        .filter_map(|i| {
            let hash = hash_for_suffix(door, &mut hasher, i);
            if hash.starts_with("00000") {
                Some(hash)
            } else {
                None
            }
        })
        .filter_map(|hash| hash.chars().nth(5))
        .take(8)
        .collect()
}

pub fn get_indexed_password(door: &str) -> String {
    let mut result = vec!['_', '_', '_', '_', '_', '_', '_', '_'];
    let mut hasher = Md5::new();
    let mut remaining = 8;
    for hash in (0..).filter_map(|i| {
        let hash = hash_for_suffix(door, &mut hasher, i);
        if hash.starts_with("00000") {
            Some(hash)
        } else {
            None
        }
    }) {
        let i = hash.chars().nth(5).unwrap() as usize - '0' as usize;
        if i < 8 && result[i] == '_' {
            result[i] = hash.chars().nth(6).unwrap();
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    result.into_iter().collect()
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
        assert_eq!(Some('1'),
                   hash_for_suffix("abc", &mut hasher, 3231929).chars().nth(5));
        assert!(hash_for_suffix("abc", &mut hasher, 5017308).starts_with("000008f82"));
    }

    // #[test]
    // fn example_password_decrypt() {
    //     assert_eq!("18f47a30", get_password("abc"));
    // }

    // #[test]
    // fn example_indexed_password() {
    //     assert_eq!("05ace8e3", get_indexed_password("abc"));
    // }
}
