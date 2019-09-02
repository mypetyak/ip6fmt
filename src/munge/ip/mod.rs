use crate::munge::Munger;
use std::io::Write;
use std::net::Ipv6Addr;
use std::str::FromStr;

pub enum IPFormat {
    Compact,
    Exploded,
}

pub struct IPMungerConfig {
    pub format: IPFormat,
}

pub struct IPMunger<'a> {
    pub out: &'a mut dyn Write,
    pub config: IPMungerConfig,
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

impl Munger for IPMunger<'_> {
    fn possible_match(&self, c: char) -> bool {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' | ':' => true,
            _ => false,
        }
    }
    fn rewriter(&mut self, s: &str) {
        match maybe_ip(&s) {
            Some(ip) => match self.config.format {
                IPFormat::Exploded => explode(&ip, &mut self.out),
                IPFormat::Compact => compact(&ip, &mut self.out),
            },
            None => self.writethru(s),
        };
    }
    fn writethru(&mut self, s: &str) {
        write!(&mut self.out, "{}", s).unwrap();
    }
}

#[cfg(test)]
mod tests {
    //TODO: figure out the typical rust naming and use conventions
    use super::Munger;
    use super::{compact, explode, maybe_ip, IPFormat, IPMunger, IPMungerConfig};
    use std::net::Ipv6Addr;

    #[test]
    fn test_rewriter() {
        let mut buf = Vec::new();
        let mut m = IPMunger {
            out: &mut buf,
            config: IPMungerConfig {
                format: IPFormat::Compact,
            },
        };

        m.rewriter("foo");
        m.rewriter(" ");
        m.rewriter("0000:0000:af77:0000:0000:0000:0000:0001");
        m.rewriter(" ");
        m.rewriter("0:0::0:1");
        assert_eq!("foo 0:0:af77::1 ::1", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn test_writethru() {
        let mut buf = Vec::new();
        let mut m = IPMunger {
            out: &mut buf,
            config: IPMungerConfig {
                format: IPFormat::Compact,
            },
        };

        m.writethru("foo");
        assert_eq!("foo", String::from_utf8(buf).unwrap());
    }

    #[test]
    fn test_possible_match() {
        let m = IPMunger {
            out: &mut Vec::new(),
            config: IPMungerConfig {
                format: IPFormat::Compact,
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
