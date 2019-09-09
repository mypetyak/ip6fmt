use std::io;

use ip6fmt::munge::ip::{IPFormat, IPMunger, IPMungerConfig};
use ip6fmt::stream::replace;

fn main() {
    let input = io::stdin();
    let mut stdin = input.lock();
    let output = io::stdout();
    let mut stdout = output.lock();

    let ip_munger = IPMunger::new(IPMungerConfig {
        format: IPFormat::Exploded,
    });

    replace(&mut stdin, &mut stdout, &ip_munger);
}
