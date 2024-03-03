use crate::error::{GcgError, Result};

#[derive(Debug, PartialEq)]
pub struct Player {
	pub nickname: String,
	pub full_name: String,
}

impl Player {
	/// The player pragma indicates the nickname and full name of a player.
	///
	/// # Examples
	/// ```
	/// # use gcg_parser::Player;
	/// # use anyhow::Ok;
	/// let text = "#player1 xXFerrisXx Ferris The Crab";
	/// let player = Player::build(text)?;
	///
	/// assert_eq!(
	///     player,
	///     Player {
	///         nickname: "xXFerrisXx".to_string(),
	///         full_name: "Ferris The Crab".to_string(),
	///     }
	/// );
	/// # Ok(())
	/// ```
	///
	/// # Errors
	///
	/// If the nickname or full name tokens are missing, a
	/// [`MissingToken`](GcgError::MissingToken) error is returned
	pub fn build(text: &str) -> Result<Player> {
		let mut tokens = text.splitn(3, ' ').skip(1);

		let mut get_token = |token: &str, token_index| {
			tokens
				.next()
				.ok_or_else(|| GcgError::MissingToken {
					token: token.to_string(),
					text: text.to_string(),
					token_index,
				})
		};

		let nickname = get_token("nickname", 2)?.to_string();
		let full_name = get_token("full_name", 3)?.to_string();

		Ok(Player {
			nickname,
			full_name,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
