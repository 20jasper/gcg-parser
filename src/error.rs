use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum GcgError {
	/// Missing token {token:?} in position {position:?}: "{text}"
	MissingToken {
		token: String,
		/// A 1 indexed position of the tokens
		position: usize,
		text: String,
	},
}

pub type Result<T> = core::result::Result<T, GcgError>;
