use crate::munge::Munger;
use std::io::Write;
use std::net::Ipv6Addr;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    //TODO: figure out the typical rust naming and use conventions
    use super::{compact, explode, maybe_ip};
    use std::net::Ipv6Addr;

    #[test]
    fn maybe_ip_positive() {
        assert!(maybe_ip("1::1").is_some());
        assert!(maybe_ip("::1").is_some());
        assert!(maybe_ip("a:a:a:a:a:a:a:a").is_some());
        assert!(maybe_ip("a:a::a").is_some());
        assert!(maybe_ip("A::1").is_some());
    }
    #[test]
    fn maybe_ip_negative() {
        assert!(maybe_ip("a:a:a:a:a:a:a:a:a").is_none());
        assert!(maybe_ip("a:a:a:a:a:a:a").is_none());
        assert!(maybe_ip("::g").is_none());
        assert!(maybe_ip("a:z::a").is_none());
    }
    #[test]
    fn explode_ip() {
        let mut buf = Vec::new();
        explode(&Ipv6Addr::new(0, 0, 0xaf77, 0, 0, 0, 0, 1), &mut buf);
        assert_eq!(
            "0000:0000:af77:0000:0000:0000:0000:0001",
            String::from_utf8(buf).unwrap()
        )
    }
    #[test]
    fn compact_ip() {
        let mut buf = Vec::new();
        compact(&Ipv6Addr::new(0, 0, 0xaf77, 0, 0, 0, 0, 1), &mut buf);
        assert_eq!("0:0:af77::1", String::from_utf8(buf).unwrap())
    }
}

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
            '0'..='9' | 'a'..='f' | ':' => true,
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
