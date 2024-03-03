use crate::error::{GcgError, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coordinate {
	Horizontal(u32, char),
	Vertical(char, u32),
}

impl Coordinate {
	/// # Examples
	/// ```
	/// # use gcg_parser::events::Coordinate;
	/// # use anyhow::Ok;
	///
	/// let coordinate = Coordinate::build("a1")?;
	/// assert_eq!(coordinate, Coordinate::Vertical('a', 1));
	///
	/// let coordinate = Coordinate::build("8n")?;
	/// assert_eq!(coordinate, Coordinate::Horizontal(8, 'n'));
	///
	/// # Ok(())
	/// ```
	///
	pub fn build(x: &str) -> Result<Self> {
		let mut chars = x.chars();

		match (chars.next(), chars.next()) {
			(Some(first), Some(second)) if first.is_alphabetic() && second.is_ascii_digit() => {
				Ok(Coordinate::Vertical(first, second.to_digit(10).unwrap()))
			}
			(Some(first), Some(second)) if first.is_ascii_digit() && second.is_alphabetic() => {
				Ok(Coordinate::Horizontal(first.to_digit(10).unwrap(), second))
			}
			_ => Err(GcgError::InvalidToken {
				token: "coordinate".to_string(),
				text: x.to_string(),
			}),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
	Regular(char),
	Blank,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Event {
	RegularPlay {
		/// nickname of the player making the play
		nickname: String,
		/// rack from which the tiles were played
		rack: Vec<Tile>,
		coordinate: Coordinate,
		/// word formed by the play. regular tiles are in caps, and blanks are in lowercase
		word_formed: String,
		/// score of the play
		score: i32,
		/// total score of the player after the play
		total_score: i32,
	},
	PassedTurn,
	TileExchange,
	Withdrawal,
	Challenge,
	LastRackPenalty,
	TimePenalty,
}

impl Event {
	pub fn build(line: &str) -> Result<Self> {
		let rest = line.strip_prefix('>').unwrap();
		let (nickname, rest) = rest.split_once(": ").unwrap();
		let (rack, rest) = rest.split_once(' ').unwrap();
		let (coordinate, rest) = rest.split_once(' ').unwrap();
		let (word_formed, rest) = rest.split_once(' ').unwrap();
		let (score, rest) = rest.split_once(' ').unwrap();
		let total_score = rest.trim_end();

		Ok(Event::RegularPlay {
			nickname: nickname.to_string(),
			rack: rack
				.chars()
				.map(Tile::Regular)
				.collect(),
			coordinate: Coordinate::build(coordinate).unwrap(),
			word_formed: word_formed.to_string(),
			score: score.parse().unwrap(),
			total_score: total_score.parse().unwrap(),
		})
	}
}

#[cfg(test)]
mod tests {
	use std::error;

	use super::*;
	use anyhow::{Ok, Result};

	#[test]
	fn should_not_parse_invalid_coords() -> Result<()> {
		let s = "whatIsThis?";
		let coordinates = Coordinate::build(s);

		let error = coordinates.unwrap_err().to_string();

		assert!(error.contains(s));

		Ok(())
	}

	#[test]
	fn should_parse_regular_play() -> Result<()> {
		let s = ">Arie_Sinke: ADHLTTW 8F WHAT +20 20";
		let event = Event::build(s)?;

		let expected = Event::RegularPlay {
			nickname: "Arie_Sinke".to_string(),
			rack: vec![
				Tile::Regular('A'),
				Tile::Regular('D'),
				Tile::Regular('H'),
				Tile::Regular('L'),
				Tile::Regular('T'),
				Tile::Regular('T'),
				Tile::Regular('W'),
			],
			coordinate: Coordinate::Horizontal(8, 'F'),
			word_formed: "WHAT".to_string(),
			score: 20,
			total_score: 20,
		};

		assert_eq!(event, expected);

		Ok(())
	}
}
