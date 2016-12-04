//! Day 4
//!
//! Identifiying room numbers

extern crate onig;

use std::collections::HashMap;

use self::onig::*;

/// Room id, with checksum.
pub struct Room<'a> {
    pub name: String,
    pub sector: u32,
    pub checksum: &'a str
}

impl<'a> From<&'a str> for Room<'a> {
    /// Parse a room from a string.
    fn from(s: &'a str) -> Room<'a> {
        let pattern = Regex::new("([a-z\\-]+)([0-9]+)\\[([a-z]+)\\]").unwrap();
        let captures = pattern.captures(s).expect("invalid room format");
        Room {
            name: captures.at(1).unwrap().replace("-", ""),
            sector: captures.at(2).unwrap().parse::<u32>().unwrap(),
            checksum: captures.at(3).unwrap(),
        }
    }
}

impl<'a> Room<'a> {
    /// Test if a room's checksum is valid.
    pub fn is_valid(&self) -> bool {
        let mut char_counts = self.name.chars()
            .fold(HashMap::new(), |mut counts, c| {
                {
                    let counter = counts.entry(c).or_insert(0);
                    *counter += 1;
                }
                counts
            })
            .into_iter()
            .collect::<Vec<_>>();
        char_counts.sort_by(|a, b| {
            use std::cmp::Ordering;
            let ordering = b.1.cmp(&a.1);
            match ordering {
                Ordering::Equal => a.0.cmp(&b.0),
                other => other
            }
        });
        let checksum = char_counts.iter().map(|count| { count.0 }).take(5).collect::<String>();
        self.checksum == checksum
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_room_string() {
        {
            let r = Room::from("aaaaa-bbb-z-y-x-123[abxyz]");
            assert_eq!("aaaaabbbzyx", r.name);
            assert_eq!(123, r.sector);
            assert_eq!("abxyz", r.checksum);
        }
        {
            let r = Room::from("a-b-c-d-e-f-g-h-987[abcde]");
            assert_eq!("abcdefgh", r.name);
            assert_eq!(987, r.sector);
            assert_eq!("abcde", r.checksum);
        }
    }

    #[test]
    fn is_valid() {
        assert!(Room::from("aaaaa-bbb-z-y-x-123[abxyz]").is_valid());
        assert!(Room::from("a-b-c-d-e-f-g-h-987[abcde]").is_valid());
        assert!(Room::from("not-a-real-room-404[oarel]").is_valid());
    }

    #[test]
    fn invalid_checksums() {
        assert!(!Room::from("totally-real-room-200[decoy]").is_valid());
    }
}
