//! Decrompression of strings.

/// Decompression state. This represents the current state which the
/// decompression state machine can be in.
enum State {
    Start,
    ReadingLength(usize),
    ReadingCount(usize, usize),
    ReadingBuff(usize, usize, String)
}

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
}
