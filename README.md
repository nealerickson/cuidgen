# cuidgen

> A command-line tool to generate CUIDs. ([cuid](https://github.com/ericelliott/cuid))

## Install

* Install a recent version of Rust using [rustup](https://rustup.rs/) or update
it with:

```
# rustup update
```

* Build `cuidgen` with cargo:

```
# cargo build --release
```

## Usage

```sh
# cuidgen --help

Simple program to generate CUIDs

Usage: cuidgen [OPTIONS]

Options:
  -c, --count <COUNT>    number of CUIDs to generate [default: 1]
  -p, --prefix <PREFIX>  prefix of the CUID
  -s, --slug             generate a slug instead of a full CUID
  -h, --help             Print help information
  -V, --version          Print version information

Examples:
  cuidgen
  cuidgen --count=10
  cuidgen --prefix=id_
  cuidgen --slug
```
