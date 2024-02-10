use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum GcgError {
	/// Missing token {token:?} in position {token_index:?}: {text:?}
	MissingToken {
		token: String,
		/// 1 indexed
		token_index: usize,
		/// 1 indexed
		line_index: usize,
		text: String,
	},
	/// Missing required pragma {keyword:?}
	MissingPragma {
		/// indicates type of pragma
		keyword: String,
	},
}

pub type Result<T> = core::result::Result<T, GcgError>;
