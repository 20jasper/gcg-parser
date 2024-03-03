use crate::error::{GcgError, Result};

mod event;
pub use event::Coordinate;
use event::Event;

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
