//! Character frequencies

use std::collections::HashMap;

pub fn recover_plaintext_by<F>(jammed: &str, char_chooser: F) -> String
    where F: Fn(HashMap<char, usize>) -> char
{
    let mut chars = jammed.lines()
        .fold(HashMap::new(), |mut counts, line| {
            for (i, c) in line.chars().enumerate() {
                let char_counts = counts.entry(i).or_insert_with(|| HashMap::new());
                *char_counts.entry(c).or_insert(0) += 1;
            }
            counts
        })
        .into_iter()
        .map(|(index, counts)| (index, char_chooser(counts)))
        .collect::<Vec<_>>();

    chars.sort_by_key(|&(i, _)| i);

    chars.into_iter().map(|(_, c)| c).collect()
}

pub fn recover_uncommon_plaintext(jammed: &str) -> String {
    recover_plaintext_by(jammed, |counts| {
        match counts.into_iter().min_by_key(|&(_, i)| i) {
            Some((ch, _)) => ch,
            None => '?',
        }
    })
}

pub fn recover_common_plaintext(jammed: &str) -> String {
    recover_plaintext_by(jammed, |counts| {
        match counts.into_iter().max_by_key(|&(_, i)| i) {
            Some((ch, _)) => ch,
            None => '?',
        }
    })
}

#[cfg(test)]
mod test {

    const EXAMPLE_INPUT: &'static str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    use super::*;

    #[test]
    fn example_is_decoded_correctly() {
        assert_eq!("easter", recover_common_plaintext(EXAMPLE_INPUT));
    }

    #[test]
    fn example_less_common_decoded() {
        assert_eq!("advent", recover_uncommon_plaintext(EXAMPLE_INPUT));
    }
}
