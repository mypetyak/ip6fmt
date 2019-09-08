use std::io::Write;

// Munger could be anything:
// - finds IP substrings and explodes them
// - finds telephone numbers and converts them to hex
// - finds dollar values and converts to rubles
pub trait Munger<'a> {
    fn possible_match(&self, c: char) -> bool;
    // possibly expensive
    fn rewriter(&self, s: &str, o: &mut dyn Write);
    // fast, use on substrs known not to be a match
    fn writethru(&self, s: &str, o: &mut dyn Write) {
        write!(o, "{}", s).unwrap();
    }
}

pub mod ip;
