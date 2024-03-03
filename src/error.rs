use displaydoc::Display;
use thiserror::Error;

use crate::token;

#[derive(Display, Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum Error {
	/// Error on line {line_index:?}: {source:?}
	InvalidLine {
		/// 1 indexed
		line_index: usize,
		#[source]
		source: token::Error,
	},
	/// Missing required pragma {keyword:?}
	MissingPragma {
		/// indicates type of pragma
		keyword: String,
	},
	/// Unknown pragma on line {line_index:?}: {line:?}
	UnknownPragma {
		line: String,
		// 1 indexed
		line_index: usize,
	},
}

pub type Result<T> = core::result::Result<T, Error>;
