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
pub mod player;

use error::{GcgError, Result};
pub use player::Player;

#[derive(Debug, PartialEq)]
pub struct Gcg {
	pub player1: Player,
	pub player2: Player,
}

impl Gcg {
	pub fn build(text: &str) -> Result<Gcg> {
		let mut player1 = None::<Player>;
		let mut player2 = None::<Player>;

		for (i, line) in text.lines().enumerate() {
			if line.starts_with("#player1") {
				let player = Player::build(line, i)?;
				player1 = Some(player);
			} else if line.starts_with("#player2") {
				let player = Player::build(line, i)?;
				player2 = Some(player);
			} else {
				return Err(GcgError::UnknownPragma {
					line: text.to_string(),
					line_index: i.saturating_add(1),
				});
			}
		}

		let gcg = Gcg {
			player1: player1.ok_or_else(|| GcgError::MissingPragma {
				keyword: "player1".to_string(),
			})?,
			player2: player2.ok_or_else(|| GcgError::MissingPragma {
				keyword: "player2".to_string(),
			})?,
		};

		Ok(gcg)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::{Ok, Result};

	#[test]
	fn should_parse_player_names() -> Result<()> {
		let text = [
			"#player1 20jasper Jacob Asper",
			"#player2 xXFerrisXx Ferris The Crab",
		]
		.join("\n");

		let gcg = Gcg::build(&text)?;

		assert_eq!(
			gcg,
			Gcg {
				player1: Player {
					nickname: "20jasper".to_string(),
					full_name: "Jacob Asper".to_string(),
				},
				player2: Player {
					nickname: "xXFerrisXx".to_string(),
					full_name: "Ferris The Crab".to_string(),
				},
			}
		);

		Ok(())
	}

	#[test]
	fn should_error_when_missing_player() {
		let text = ["#player2 20jasper Jacob Asper"].join("\n");

		let error = Gcg::build(&text)
			.unwrap_err()
			.to_string()
			.to_lowercase();

		assert!(error.contains("player1"));
	}

	#[test]
	fn should_error_with_unknown_pragma() {
		let text = ["#whatisthispragma what idk"].join("\n");

		let error = Gcg::build(&text)
			.unwrap_err()
			.to_string()
			.to_lowercase();

		assert!(error.contains("unknown pragma"));
		assert!(error.contains("#whatisthispragma"));
	}
}
