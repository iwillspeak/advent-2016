//! IPV7 Parsing

pub struct Ip<'a> {
    parts: Vec<IpPart<'a>>,
}

#[derive(Debug,PartialEq)]
pub enum IpPart<'a> {
    Standard(&'a str),
    Hypernet(&'a str),
}

#[derive(PartialEq)]
enum ParseState {
    Start,
    Standard,
    Hypernet,
}

/// Check for an ABBA in a given string slice
fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|chars| chars[0] == chars[3] && chars[1] == chars[2] && chars[0] != chars[1])
}

fn is_aba(s: &[u8]) -> bool {
    s[0] == s[2] && s[1] != s[0]
}

fn to_bab(s: &[u8]) -> Vec<u8> {
    let mut bab = Vec::new();
    bab.push(s[1]);
    bab.push(s[0]);
    bab.push(s[1]);
    bab
}

/// Parse a IP Parts from a string.
fn ip_parts_from_str<'a>(s: &'a str) -> Vec<IpPart<'a>> {
    use self::ParseState::*;
    let mut parts = Vec::new();
    let mut state = Start;
    let mut start = 0;
    for (pos, ch) in s.char_indices() {
        state = match state {
            Start => {
                match ch {
                    '[' => {
                        start = pos + 1;
                        Hypernet
                    }
                    'a'...'z' => {
                        start = pos;
                        Standard
                    }
                    _ => panic!("invalid character {} at {}", ch, pos),
                }
            }
            Standard => {
                match ch {
                    'a'...'z' => Standard,
                    '[' => {
                        parts.push(IpPart::Standard(&s[start..pos]));
                        start = pos + 1;
                        Hypernet
                    }
                    _ => panic!("invalid character {} at {}", ch, pos),
                }
            }
            Hypernet => {
                match ch {
                    'a'...'z' => {
                        if start == 0 {
                            start = pos
                        }
                        Hypernet
                    }
                    ']' => {
                        parts.push(IpPart::Hypernet(&s[start..pos]));
                        Start
                    }
                    _ => panic!("invalid character {} at {}", ch, pos),
                }
            }
        };
    }
    match state {
        Standard => {
            parts.push(IpPart::Standard(&s[start..]));
        }
        Start => (),
        Hypernet => panic!("Unterminated hypernet block starting at {}", start),
    };
    parts
}

impl<'a> From<&'a str> for Ip<'a> {
    /// Parse an IP Packet from a string.
    fn from(s: &'a str) -> Self {
        Ip { parts: ip_parts_from_str(s) }
    }
}

impl<'a> Ip<'a> {
    /// Check for TLS Support
    pub fn supports_tls(&self) -> bool {
        // If a hypernet part has an ABBA block this IP doesn't
        // support TLS
        if self.parts
            .iter()
            .filter_map(|part| {
                match part {
                    &IpPart::Standard(_) => None,
                    &IpPart::Hypernet(hyp) => Some(hyp),
                }
            })
            .any(|part| has_abba(part)) {
            return false;
        }

        // otherwise return true if any standard part has an ABBA
        // block.
        self.parts
            .iter()
            .filter_map(|part| {
                match part {
                    &IpPart::Standard(std) => Some(std),
                    &IpPart::Hypernet(_) => None,
                }
            })
            .any(|part| has_abba(part))
    }

    pub fn supports_ssl(&self) -> bool {
        let mut abas: Vec<&[u8]> = Vec::new();
        let mut babs: Vec<&[u8]> = Vec::new();
        for part in self.parts.iter() {
            match part {
                &IpPart::Standard(std) => {
                    for s in std.as_bytes().windows(3).filter(|window| is_aba(window)) {
                        if babs.contains(&&(to_bab(s)[..])) {
                            return true;
                        }
                        abas.push(s);
                    }
                }
                &IpPart::Hypernet(hyper) => {
                    for s in hyper.as_bytes().windows(3).filter(|window| is_aba(window)) {
                        if abas.contains(&&(to_bab(s)[..])) {
                            return true;
                        }
                        babs.push(s);
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {

    use super::*;

    macro_rules! check_parse {
        ($ip:expr, $parts:expr) => {
            let ip = Ip::from($ip);
            assert_eq!($parts, ip.parts);
        }
    }

    #[test]
    fn parse_addresses() {
        check_parse!("abba[mnop]qrst",
                     vec![
                         IpPart::Standard("abba"),
                         IpPart::Hypernet("mnop"),
                         IpPart::Standard("qrst"),
                     ]);
        check_parse!("abcd[bddb]xyyx",
                     vec![
                         IpPart::Standard("abcd"),
                         IpPart::Hypernet("bddb"),
                         IpPart::Standard("xyyx"),
                     ]);
        check_parse!("aaaa[qwer]tyui",
                     vec![
                         IpPart::Standard("aaaa"),
                         IpPart::Hypernet("qwer"),
                         IpPart::Standard("tyui"),
                     ]);
        check_parse!("ioxxoj[asdfgh]zxcvbn",
                     vec![
                         IpPart::Standard("ioxxoj"),
                         IpPart::Hypernet("asdfgh"),
                         IpPart::Standard("zxcvbn"),
                     ]);
    }

    #[test]
    fn example_tls_support() {
        assert_eq!(true, Ip::from("ioxxoj[asdfgh]zxcvbn").supports_tls());
        assert_eq!(false, Ip::from("aaaa[qwer]tyui").supports_tls());
        assert_eq!(false, Ip::from("abcd[bddb]xyyx").supports_tls());
        assert_eq!(true, Ip::from("abba[mnop]qrst").supports_tls());
    }

    #[test]
    fn example_ssl_support() {
        assert_eq!(true, Ip::from("aba[bab]xyz").supports_ssl());
        assert_eq!(false, Ip::from("xyx[xyx]xyx").supports_ssl());
        assert_eq!(true, Ip::from("aaa[kek]eke").supports_ssl());
        assert_eq!(true, Ip::from("zazbz[bzb]cdb").supports_ssl());
    }
}
