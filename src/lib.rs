//! [<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/20jasper/gcg-parser)
//! [<img alt="crates.io" src="https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/gcg-parser)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/gcg-parser)
//! <br>
//! gcg-parser converts "generic crossword game" (GCG) files into more convenient formats
//!
//! GCG files are used as a standard for a variety of crossword games, most notably Scrabble
//!
//! ## Specification
//!
//! This parser abides by the [Poslfit GCG specification](https://www.poslfit.com/scrabble/gcg/)

pub mod error;
mod gcg;

pub use gcg::{events, Gcg, Player};
