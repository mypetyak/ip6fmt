use std::io;

use ip::munge::ip::{IPFormat, IPMunger, IPMungerConfig};
use ip::stream::replace;

fn main() {
    let input = io::stdin();
    let mut stdin = input.lock();
    let output = io::stdout();
    let mut stdout = output.lock();

    let mut ip_munger = IPMunger::new(IPMungerConfig {
        format: IPFormat::Exploded,
    });

    replace(&mut stdin, &mut stdout, &mut ip_munger);
}
