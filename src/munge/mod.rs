use std::io::Write;

// Munger could be anything:
// - finds IP substrings and explodes them
// - finds telephone numbers and converts them to hex
// - finds dollar values and converts to rubles
pub trait Munger<'a> {
    fn possible_match(&self, c: char) -> bool;
    // possibly expensive
    fn rewriter(&mut self, s: &str);
    // fast, use on substrs known not to be a match
    fn writethru(&mut self, s: &str);
    fn output(&'a mut self) -> &'a mut dyn Write;
}

pub mod ip;
