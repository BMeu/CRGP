# CRGP

[![Build Status on Codeship](https://app.codeship.com/projects/7d2924a0-f1e4-0134-404a-569aa21b12f1/status?branch=master)](https://app.codeship.com/projects/209508)
[![Build Status on Travis CI](https://travis-ci.org/BMeu/CRGP.svg?branch=master)](https://travis-ci.org/BMeu/CRGP)
[![codecov](https://codecov.io/gh/BMeu/CRGP/branch/master/graph/badge.svg)](https://codecov.io/gh/BMeu/CRGP)

A graph-parallel approach for reconstructing the influences within Retweet cascades.

For details and [benchmarks](https://bitbucket.org/BMeu/crgp/wiki/Benchmarks/Home.md), see the
[Wiki](https://bitbucket.org/BMeu/crgp/wiki/Home).

## Requirements

`CRGP` requires the following programs to be installed _before_ compilation:

* [Rust `1.17+`](https://www.rustup.rs)
* OpenSSL `1.0.1+` (on Linux, you will need the developer packages `openssl-devel` or `libssl-dev`, depending on your
  distribution, and the regular development utilities like `pkg-config`) - see also
  [these notes](https://github.com/sfackler/rust-openssl/blob/0f02a8b61d9d8878eece3264bdc3271e2bd9b86b/README.md#building).

## Usage

Using Rust's package manager [`Cargo`](http://doc.crates.io/guide.html), executing `CRGP` is really simple:

```bash
$ cargo run --release -- [FRIENDS] [RETWEETS] 
```

This will compile `CRGP` with all its dependencies and run the binary. A full list of options is available using the
`-h` flag:

```bash
$ cargo run --release -- -h
```

## Example

This repository includes a data set you can use to test `CRGP`. It consists of two tiny Retweet cascades (each with
three Retweets) on a tiny social graph:

```bash
$ cargo run --release -- data/social_graph data/retweets.json
```

## Author

`CRGP` is developed by [Bastian Meyer](http://www.bastianmeyer.eu/)
<[bastian@bastianmeyer.eu](mailto:bastian@bastianmeyer.eu)> for his master's thesis at the
[Research Group on Web Science](https://websci.informatik.uni-freiburg.de/),
[University of Freiburg, Germany](https://www.uni-freiburg.de), under Prof. Dr. Peter Fischer.

## License

`CRGP` is licensed under either of

 * Apache License, Version 2.0, ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
