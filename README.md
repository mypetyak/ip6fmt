# ip6fmt
Inline formatter for IPv6 Addresses.

I work with IPv6 addresses and frequently find myself wanting to compare, sort, or collect them from text files or log output.  This tool munges its `stdin` to identify IPv6 addresses, reformat them, and pass the results to `stdout`.  Use it like a `sed` replacer.

[![Build Status](https://travis-ci.org/mypetyak/ip6fmt.svg?branch=master)](https://travis-ci.org/mypetyak/ip6fmt)

## Installing a Release
Compiled binaries are released as [snaps](https://snapcraft.io/docs/getting-started), which can be installed on most Linux systems via:

```
$ snap install ip6fmt
```

## Building
Dependencies: `cargo`, `rust`

To build from source:

```
$ git clone https://github.com/mypetyak/ip6fmt
$ cd ip6fmt

$ echo "foo bar ::1 baz 0:af77::4 fuz" | cargo run
   Compiling ip v0.1.0 (/home/bunn/stuff-artiodactyl/rust/ip6fmt)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
     Running `target/debug/ip`
foo bar 0000:0000:0000:0000:0000:0000:0000:0001 baz 0000:af77:0000:0000:0000:0000:0000:0004 fuz
```

## Running Tests
To run unit tests:

```
$ cargo test --all
```
