use crate::munge::Munger;
use std::io::Write;
use std::net::Ipv6Addr;
use std::str::FromStr;

pub enum IPFormat {
    Compact,
    Exploded,
}

pub enum IPSurround {
    Brackets,
    Empty,
}

pub struct IPMungerConfig {
    pub format: IPFormat,
    pub surround: IPSurround,
}

pub struct IPMunger {
    config: IPMungerConfig,
}

fn maybe_ip(line: &str) -> Option<Ipv6Addr> {
    match Ipv6Addr::from_str(&line) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn explode<T: Write>(ip: &Ipv6Addr, loc: &mut T) {
    match ip.segments() {
        [a, b, c, d, e, f, g, h] => write!(
            loc,
            "{:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}",
            a, b, c, d, e, f, g, h
        )
        .unwrap(),
    };
}

fn compact<T: Write>(ip: &Ipv6Addr, loc: &mut T) {
    write!(loc, "{}", ip).unwrap();
}

impl IPMunger {
    pub fn new(config: IPMungerConfig) -> IPMunger {
        IPMunger { config: config }
    }
}

impl Munger for IPMunger {
    fn possible_match(&self, c: char) -> bool {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' | ':' => true,
            _ => false,
        }
    }
    fn rewriter(&self, s: &str, mut o: &mut dyn Write) {
        match maybe_ip(&s) {
            Some(ip) => {
                match self.config.surround {
                    IPSurround::Brackets => write!(o, "[").unwrap(),
                    IPSurround::Empty => (),
                };
                match self.config.format {
                    IPFormat::Exploded => explode(&ip, &mut o),
                    IPFormat::Compact => compact(&ip, &mut o),
                };
                match self.config.surround {
                    IPSurround::Brackets => write!(o, "]").unwrap(),
                    IPSurround::Empty => (),
                };
            }
            None => self.writethru(s, o),
        };
    }
}

#[cfg(test)]
mod tests {
    //TODO: figure out the typical rust naming and use conventions
    use super::Munger;
    use super::{compact, explode, maybe_ip, IPFormat, IPMunger, IPMungerConfig, IPSurround};
    use std::net::Ipv6Addr;

    #[test]
    fn test_rewriter() {
        let mut buf = Vec::new();
        let m = IPMunger {
            config: IPMungerConfig {
                format: IPFormat::Compact,
                surround: IPSurround::Empty,
            },
        };

        m.rewriter("foo", &mut buf);
        m.rewriter(" ", &mut buf);
        m.rewriter("0000:0000:af77:0000:0000:0000:0000:0001", &mut buf);
        m.rewriter(" ", &mut buf);
        m.rewriter("0:0::0:1", &mut buf);
        assert_eq!("foo 0:0:af77::1 ::1", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn test_wrap_brackets() {
        let mut buf = Vec::new();
        let m = IPMunger {
            config: IPMungerConfig {
                format: IPFormat::Compact,
                surround: IPSurround::Brackets,
            },
        };

        m.rewriter("foo", &mut buf);
        m.rewriter(" ", &mut buf);
        m.rewriter("0000:0000:af77:0000:0000:0000:0000:0001", &mut buf);
        m.rewriter(" ", &mut buf);
        m.rewriter("0:0::0:1", &mut buf);
        assert_eq!("foo [0:0:af77::1] [::1]", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn test_writethru() {
        let mut buf = Vec::new();
        let m = IPMunger {
            config: IPMungerConfig {
                format: IPFormat::Compact,
                surround: IPSurround::Empty,
            },
        };

        m.writethru("foo", &mut buf);
        assert_eq!("foo", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn test_possible_match() {
        let m = IPMunger {
            config: IPMungerConfig {
                format: IPFormat::Compact,
                surround: IPSurround::Empty,
            },
        };

        assert!(m.possible_match('a'));
        assert!(m.possible_match('A'));
        assert!(m.possible_match('f'));
        assert!(m.possible_match('F'));
        assert!(m.possible_match('0'));
        assert!(m.possible_match('9'));
        assert!(m.possible_match(':'));

        assert!(!m.possible_match('g'));
        assert!(!m.possible_match('G'));
        assert!(!m.possible_match('_'));
        assert!(!m.possible_match('!'));
    }

    #[test]
    fn test_maybe_ip() {
        // positive cases
        assert!(maybe_ip("1::1").is_some());
        assert!(maybe_ip("::1").is_some());
        assert!(maybe_ip("a:a:a:a:a:a:a:a").is_some());
        assert!(maybe_ip("a:a::a").is_some());
        assert!(maybe_ip("A::1000").is_some());
        assert!(maybe_ip("A:f::1").is_some());

        // negative cases
        assert!(maybe_ip("a:a:a:a:a:a:a:a:a").is_none());
        assert!(maybe_ip("a:a:a:a:a:a:a").is_none());
        assert!(maybe_ip("::g").is_none());
        assert!(maybe_ip("::10001").is_none());
        assert!(maybe_ip("a:z::a").is_none());
    }

    #[test]
    fn test_explode() {
        let mut buf = Vec::new();
        explode(&Ipv6Addr::new(0, 0, 0xaf77, 0, 0, 0, 0, 1), &mut buf);
        assert_eq!(
            "0000:0000:af77:0000:0000:0000:0000:0001",
            String::from_utf8(buf).unwrap()
        )
    }

    #[test]
    fn test_compact() {
        let mut buf = Vec::new();
        compact(&Ipv6Addr::new(0, 0, 0xaf77, 0, 0, 0, 0, 1), &mut buf);
        assert_eq!("0:0:af77::1", String::from_utf8(buf).unwrap())
    }
}
