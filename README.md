# ip6fmt
Inline formatter for IPv6 Addresses.

I work with IPv6 addresses and frequently find myself wanting to compare, sort, or collect them from text files or log output.  This tool munges its `stdin` to identify IPv6 addresses, reformat them, and pass the results to `stdout`.  Use it like a `sed` replacer.

[![Build Status](https://travis-ci.org/mypetyak/ip6fmt.svg?branch=master)](https://travis-ci.org/mypetyak/ip6fmt)

## Installing a Release
Compiled binaries are released as [snaps](https://snapcraft.io/docs/getting-started), which can be installed on most Linux systems via:

Install a stable release with:
```
$ sudo snap install ip6fmt
```

Install a release built from `master` with:

```
$ sudo snap install ip6fmt --edge
```

## Usage
See the help:

```
$ ip6fmt -h
ip6fmt
Re-format the IPv6 address from stdin.

USAGE:
    ip6fmt [FLAGS]

FLAGS:
    -b               Surround with brackets
    -c               Compact IPv6 format
    -h, --help       Prints help information
    -V, --version    Prints version information
```

Some examples:

```
$ cat /tmp/foo
foo bar
::1 baz
fuz 0:af77::4 fah
boo 4:0:0::27

$ ip6fmt </tmp/foo
foo bar
0000:0000:0000:0000:0000:0000:0000:0001 baz
fuz 0000:af77:0000:0000:0000:0000:0000:0004 fah
boo 0004:0000:0000:0000:0000:0000:0000:0027

$ ip6fmt -b </tmp/foo
foo bar
[0000:0000:0000:0000:0000:0000:0000:0001] baz
fuz [0000:af77:0000:0000:0000:0000:0000:0004] fah
boo [0004:0000:0000:0000:0000:0000:0000:0027]

$ ip6fmt -b -c </tmp/foo
foo bar
[::1] baz
fuz [0:af77::4] fah
boo [4::27]
```


## Building
Dependencies: `cargo`, `rust`

To build from source:

```
$ git clone https://github.com/mypetyak/ip6fmt
$ cd ip6fmt
$ cargo build
$ ./target/debug/ip6fmt -h
```

## Running Tests
To run unit tests:

```
$ cargo test --all
```
