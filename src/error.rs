use std::format;

// Main Error enum for the TQC.
#[derive(Debug)]
pub enum Error {
	BraidingError(String),
	Other(String),
}

impl std::error::Error for Error{}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::BraidingError(msg) => write!(f, "Braiding Error: {}", msg),
			Error::Other(msg) => write!(f, "Other Error: {}", msg),
		}
	}
}
