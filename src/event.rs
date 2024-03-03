use crate::error::{GcgError, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Coordinate {
	Horizontal(u32, char),
	Vertical(char, u32),
}

impl Coordinate {
	pub fn build(x: &str) -> Result<Self> {
		let mut chars = x.chars();
		let first = chars.next().unwrap();
		let second = chars.next().unwrap();

		if first.is_alphabetic() {
			Ok(Coordinate::Vertical(first, second.to_digit(10).unwrap()))
		} else if first.is_ascii_digit() {
			Ok(Coordinate::Horizontal(first.to_digit(10).unwrap(), second))
		} else {
			todo!("error")
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
		let (score, rest) = rest.split_once(' ').unwrap();
		let total_score = rest.trim_end();

		dbg!(rack);

		Ok(Event::RegularPlay {
			nickname: nickname.to_string(),
			rack: rack
				.chars()
				.map(Tile::Regular)
				.collect(),
			coordinate: Coordinate::build(coordinate).unwrap(),
			word_formed: String::new(),
			score: 10,
			total_score: 10,
		})
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EventWithNote {
	event: Event,
	note: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Events {
	events: Vec<EventWithNote>,
}

impl Events {
	pub fn new() -> Self {
		Self { events: vec![] }
	}

	pub fn from_slice(slice: &[EventWithNote]) -> Self {
		Self {
			events: slice.to_vec(),
		}
	}

	pub fn add(&mut self, line: &str) -> Result<()> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::{Ok, Result};

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
