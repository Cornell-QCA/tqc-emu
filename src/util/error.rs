use std::fmt;

// Main Error enum for the TQC.
#[derive(Debug)]
pub enum Error {
	BraidingError(String),
	FusionError(String),
	Other(String),
}

impl std::error::Error for Error{}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::BraidingError(msg) => write!(f, "Braiding Error: {}", msg),
			Error::FusionError(msg) => write!(f, "Fusion Error: {}", msg),
			Error::Other(msg) => write!(f, "Other Error: {}", msg),
		}
	}
}
