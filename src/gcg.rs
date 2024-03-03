pub mod events;
mod player;

use crate::error::{GcgError, Result};
pub use player::Player;

#[derive(Debug, PartialEq)]
pub struct Gcg {
	pub player1: Player,
	pub player2: Player,
	/// A long description of the game. May contain HTML entities
	pub description: Option<String>,
}

impl Gcg {
	pub fn build(text: &str) -> Result<Gcg> {
		let mut player1 = None::<Player>;
		let mut player2 = None::<Player>;
		let mut description = None::<String>;

		for (i, line) in text.lines().enumerate() {
			if line.starts_with("#player1") {
				let player = Player::build(line)?;
				player1 = Some(player);
			} else if line.starts_with("#player2") {
				let player = Player::build(line)?;
				player2 = Some(player);
			} else if line.starts_with("#description") {
				let (_, desc) = line.split_once(' ').unwrap_or_default();

				description = Some(desc.to_string());
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
			description,
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
			gcg.player1,
			Player {
				nickname: "20jasper".to_string(),
				full_name: "Jacob Asper".to_string(),
			},
		);
		assert_eq!(
			gcg.player2,
			Player {
				nickname: "xXFerrisXx".to_string(),
				full_name: "Ferris The Crab".to_string(),
			},
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

	#[test]
	fn should_parse_description() -> Result<()> {
		let text = [
			"#player1 20jasper Jacob Asper",
			"#player2 xXFerrisXx Ferris The Crab",
			"#description 20jasper vs xXFerrisXx",
		]
		.join("\n");

		let gcg = Gcg::build(&text)?;

		assert_eq!(gcg.description, Some("20jasper vs xXFerrisXx".to_string()));

		Ok(())
	}
}
