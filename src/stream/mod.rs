use crate::munge::Munger;
use std::io::BufRead;
use std::io::Write;

pub fn replace<I: BufRead, O: Write>(inbuf: &mut I, outbuf: &mut O, fi: &mut dyn Munger) {
    let mut line = String::new();

    let mut process_line = move |line: &str| {
        // left <--> right will alternate between growing to surround
        // maybe_ip words and the expanses between them
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut inside: bool = false;

        for (idx, c) in line.char_indices() {
            match fi.possible_match(c) {
                true => {
                    if !inside {
                        // we've exited a known non-match substr
                        inside = true;
                        fi.writethru(&line[left..idx], outbuf);
                        left = idx;
                    }
                    right = idx;
                }
                false => {
                    if inside {
                        // we've exited a possible match substr
                        fi.rewriter(&line[left..right + 1], outbuf);
                        left = idx;
                        right = idx;
                    }
                    inside = false;
                }
            }
        }
        fi.rewriter(&line[left..], outbuf);
    };

    loop {
        line.clear();
        match inbuf.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => process_line(&line),
            Err(_) => break,
        };
    }
}
