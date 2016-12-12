//! Decrompression of strings.

/// Decompression state. This represents the current state which the
/// decompression state machine can be in.
enum State {
    Start,
    ReadingLength(usize),
    ReadingCount(usize, usize),
    ReadingBuff(usize, usize, String)
}

/// A decompression repetition window
struct Window(usize, usize);

macro_rules! ival {
    ($ch:expr) => {
        ($ch as u8 - '0' as u8) as usize
    }
}

/// Decompression function. Applies the run-length-encoding codes
/// found within the input and returns the result.
///
/// # Arugments
///  * `input` - The string to decode
///
/// # Returns
///
/// The decompressed string data.
pub fn decompress(input: &str) -> String {
    use self::State::*;
    let mut res = String::with_capacity(input.len());
    let mut state = State::Start;
    for ch in input.chars() {
        state = match state {
            Start => match ch {
                '(' => ReadingLength(0),
                ch => {
                    res.push(ch);
                    Start
                }
            },
            ReadingLength(len) => match ch {
                '0'...'9' => ReadingLength((len * 10) + ival!(ch)),
                'x' => ReadingCount(len, 0),
                _ => panic!("unexpected char {} when reading repeat length", ch),
            },
            ReadingCount(len, count) => match ch {
                '0'...'9' => ReadingCount(len, (count * 10) + ival!(ch)),
                ')' => ReadingBuff(len, count, String::with_capacity(len)),
                _ => panic!("unexpected char {} when reading repeat cont", ch),
            },
            ReadingBuff(len, count, mut buf) => {
                buf.push(ch);
                if len > 1 {
                    ReadingBuff(len - 1, count, buf)
                } else {
                    for _ in 0..count {
                        res.push_str(&buf)
                    }
                    Start
                }
            }
        }
    }
    res
}

/// Get decompressed lenght.
///
/// Given an input string return the length of the string after
/// decompressing it and, optionally, expanding markers in the output
/// of each level of decompression.
pub fn decompressed_length(input: &str) -> usize {
    use self::State::*;
    let mut res = 0;
    let mut windows = Vec::new();
    let mut state = State::Start;
    for ch in input.chars() {
        let mut char_weight = 1;
        let mut new_windows = Vec::new();
        for &mut Window(size, weight) in &mut windows {
            if size > 1 {
                new_windows.push(Window(size - 1, weight));
            }
            char_weight = weight * char_weight;
        }
        windows = new_windows;
        state = match state {
            Start => match ch {
                '(' => ReadingLength(0),
                _ => {
                    res += char_weight;
                    Start
                }
            },
            ReadingLength(len) => match ch {
                '0'...'9' => ReadingLength((len * 10) + ival!(ch)),
                'x' => ReadingCount(len, 0),
                _ => panic!("unexpected char {} when reading repeat length", ch),
            },
            ReadingCount(len, count) => match ch {
                '0'...'9' => ReadingCount(len, (count * 10) + ival!(ch)),
                ')' => {
                    windows.push(Window(len, count));
                    Start
                },
                _ => panic!("unexpected char {} when reading repeat cont", ch),
            },
            _ => panic!("d"),
        }        
    }
    res
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example_decompressions() {
        assert_eq!("ADVENT", decompress("ADVENT"));
        assert_eq!("ABBBBBC", decompress("A(1x5)BC"));
        assert_eq!("XYZXYZXYZ", decompress("(3x3)XYZ"));
        assert_eq!("ABCBCDEFEFG", decompress("A(2x2)BCD(2x2)EFG"));
        assert_eq!("(1x3)A", decompress("(6x1)(1x3)A"));
        assert_eq!("X(3x3)ABC(3x3)ABCY", decompress("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn example_recursive_decompression() {
        assert_eq!(20, decompressed_length("X(8x2)(3x3)ABCY"));
        assert_eq!(9, decompressed_length("(3x3)XYZ"));
        assert_eq!(241920, decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(445, decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));
    }
}
