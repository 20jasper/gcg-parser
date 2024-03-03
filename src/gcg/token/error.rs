use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
pub enum Error {
	/// Missing token {token:?} in position {token_index:?}: {text:?}
	MissingToken {
		token: String,
		/// 1 indexed
		token_index: usize,
		text: String,
	},
	/// Invalid token {token:?}: {text:?}
	InvalidToken { token: String, text: String },
}

pub type Result<T> = core::result::Result<T, Error>;
