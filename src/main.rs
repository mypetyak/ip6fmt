extern crate clap;
use clap::{App, Arg};
use std::io;

use ip6fmt::munge::ip::{IPFormat, IPMunger, IPMungerConfig, IPSurround};
use ip6fmt::stream::replace;

fn main() {
    let args = App::new("ip6fmt")
        .about("Re-format the IPv6 address from stdin.")
        .arg(
            Arg::with_name("b")
                .short("b")
                .help("Surround with brackets"),
        )
        .arg(Arg::with_name("c").short("c").help("Compact IPv6 format"))
        .get_matches();

    let input = io::stdin();
    let mut stdin = input.lock();
    let output = io::stdout();
    let mut stdout = output.lock();

    let ip_munger = IPMunger::new(IPMungerConfig {
        format: match args.is_present("c") {
            true => IPFormat::Compact,
            false => IPFormat::Exploded,
        },
        surround: match args.is_present("b") {
            true => IPSurround::Brackets,
            false => IPSurround::Empty,
        },
    });

    replace(&mut stdin, &mut stdout, &ip_munger);
}
