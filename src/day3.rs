//! Day 3

/// Returns true if the given string represents a valid triangle.
pub fn is_triangular(s: &str) -> bool {
    let sides = s.split_whitespace()
        .map(|s| { i32::from_str_radix(s, 10).unwrap() })
        .collect::<Vec<_>>();

    let total: i32 = sides.iter().sum();

    for i in 0..3 {
        let len = sides[i];
        if (total - len) <= len {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn is_triangular_valid_triangles() {
        assert!(is_triangular("2 1 2"));
    }

    #[test]
    fn is_triangular_with_invalid_triangles() {
        assert!(!is_triangular("5 10 25"));
    }
}
