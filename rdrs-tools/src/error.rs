use std::{error::Error as StdError, fmt, panic::Location};

#[derive(Debug)]
pub struct Error {
	pub error:    Box<dyn StdError + Send + Sync>,
	pub location: &'static Location<'static>,
}

impl<E: StdError + Send + Sync + 'static> From<E> for Error {
	#[track_caller]
	#[inline]
	fn from(error: E) -> Self { Self { error: Box::new(error), location: Location::caller() } }
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.error, self.location)
	}
}

pub type Result<T> = std::result::Result<T, Error>;
