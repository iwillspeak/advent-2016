//! Day 3.
//!
//! Counting valid triangles based on the length of each side. A
//! triangle is considered 'valid' if all sides are shorter than the
//! sum of the other two sides.

/// Counts the valid triangles in a list of sides.
fn count_valid_triangles(numbers: Vec<i32>) -> usize {
    numbers.chunks(3)
        .filter(|sides| { is_triangular(sides) })
        .count()
}

/// Get the whitespace-separated numbers from a string.
fn get_numbers(triangles: &str) -> Vec<i32> {
    triangles.split_whitespace()
        .map(|s| { i32::from_str_radix(s, 10).unwrap() })
        .collect()
}

/// Returns true if the given string represents a valid triangle.
pub fn is_triangular(sides: &[i32]) -> bool {
    let total: i32 = sides.iter().sum();

    for i in 0..3 {
        let len = sides[i];
        if (total - len) <= len {
            return false;
        }
    }
    true
}

/// Counts the valid triangles, taking sides from each line.
pub fn count_by_line(triangles: &str) -> usize {
    let numbers = get_numbers(triangles);
    count_valid_triangles(numbers)
}

/// Counts the valid triangles in a given column.
fn count_at_offset<'a, T>(numbers: T, offset: usize) -> usize
    where T: IntoIterator<Item=&'a (usize, i32)>
{
    let numbers = numbers.into_iter()
        .filter_map(|&(i, side)| {
            if (i + offset) % 3 == 0 {
                Some(side)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    count_valid_triangles(numbers)
}

/// Counts the valid triangles, taking sides from each column.
pub fn count_by_column(triangles: &str) -> usize {
    let numbers = get_numbers(triangles)
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    (0..3).map(|i| { count_at_offset(&numbers, i) })
        .sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn is_triangular_valid_triangles() {
        assert!(is_triangular(&[2, 1, 2]));
    }

    #[test]
    fn is_triangular_with_invalid_triangles() {
        assert!(!is_triangular(&[5, 10, 25]));
    }

    #[test]
    fn vertical_example() {
        const TRIANGLES: &'static str  = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        assert_eq!(6, count_by_column(TRIANGLES));
    }
}
