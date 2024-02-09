//! [<img alt="github" src="https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/20jasper/gcg-parser)
//! [<img alt="crates.io" src="https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust" height="20">](https://crates.io/crates/gcg-parser)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/gcg-parser)
//! <br>
//! gcg-parser converts "generic crossword game" (GCG) files into more convenient formats

pub mod error;
use error::{GcgError, Result};

#[derive(Debug, PartialEq)]
pub struct Player {
	pub nickname: String,
	pub full_name: String,
}

impl Player {
	/// The first token is the nickname, the rest are the full name
	pub fn build(text: &str) -> Result<Player> {
		let mut tokens = text.splitn(3, ' ').skip(1);

		let nickname = tokens
			.next()
			.ok_or_else(|| GcgError::MissingToken {
				token: "nickname".to_string(),
				position: 2,
				text: text.to_string(),
			})?
			.to_string();
		let full_name = tokens
			.next()
			.ok_or_else(|| GcgError::MissingToken {
				token: "full_name".to_string(),
				position: 3,
				text: text.to_string(),
			})?
			.to_string();

		Ok(Player {
			nickname,
			full_name,
		})
	}
}

#[cfg(test)]
mod tests {
	use anyhow::Result;

	use super::*;

	#[test]
	fn should_parse_player_name() -> Result<()> {
		let text = "#player1 20jasper Jacob Asper";
		let player = Player::build(text)?;

		assert_eq!(
			player,
			Player {
				nickname: "20jasper".to_string(),
				full_name: "Jacob Asper".to_string(),
			}
		);

		Ok(())
	}

	#[test]
	fn should_return_error_with_field_name_and_position() {
		let text = "#player1 20jasper";
		let error = Player::build(text)
			.unwrap_err()
			.to_string();

		assert!(error.contains("full_name"));
		assert!(error.contains("position 3"));
	}
}
