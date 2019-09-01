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

impl IPMunger<'_> {
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
}

impl Munger for IPMunger<'_> {
    fn possible_match(&self, c: char) -> bool {
        match c {
            '0'..='9' | 'a'..='f' | ':' => true,
            _ => false,
        }
    }
    fn rewriter(&mut self, s: &str) {
        match IPMunger::maybe_ip(&s) {
            Some(ip) => match self.config.format {
                IPFormat::Exploded => IPMunger::explode(&ip, &mut self.out),
                IPFormat::Compact => IPMunger::compact(&ip, &mut self.out),
            },
            None => self.writethru(s),
        };
    }

    fn writethru(&mut self, s: &str) {
        write!(&mut self.out, "{}", s).unwrap();
    }
}
