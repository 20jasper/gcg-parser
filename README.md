# GCG parser

[<img alt="github" src="https://img.shields.io/badge/github-20jasper/gcg--parser-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/20jasper/gcg-parser)
[<img alt="crates.io" src="https://img.shields.io/crates/v/gcg-parser.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/gcg-parser)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-gcg--parser-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/gcg-parser)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/20jasper/gcg-parser/verify.yaml?branch=main&style=for-the-badge" height="20">](https://github.com/20jasper/gcg-parser/actions?query=branch%3Amain)

Parser and Data Structures for the "Generic Crossword Game" file format

GCG files are used as a standard for a variety of crossword games, most notably Scrabble

## Specification

This parser abides by the [Poslfit GCG specification](https://www.poslfit.com/scrabble/gcg/)

## Development

### Dependencies

| Dependency  | Installation                                                             |
| ----------- | ------------------------------------------------------------------------ |
| Rust        | [Install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) |
| cargo-watch | `cargo install cargo-watch`                                              |
| just        | `cargo install just`                                                     |
| npm         | [Install NodeJS](https://nodejs.org/en/download)                         |

### Commands

run `just --list` to see a list of available commands
