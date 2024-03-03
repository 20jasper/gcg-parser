use crate::token::{Error, Result};

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
			_ => Err(Error::InvalidToken {
				token: "coordinate".to_string(),
				text: x.to_string(),
			}),
		}
	}
}

#[cfg(test)]
mod tests {
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
}
