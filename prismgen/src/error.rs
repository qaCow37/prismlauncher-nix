pub struct Error {
	err: Box<ErrorKind>,
}

impl Error {
	fn with_error_kind(kind: ErrorKind) -> Self {
		Self {
			err: Box::new(
				kind
			)
		}
	}

	pub fn custom(msg: String) -> Self {
		Self::with_error_kind(ErrorKind::Custom(
			msg
		))
	}
}

enum ErrorKind {
	IO       (std::io::Error),
	Reqwest  (reqwest::Error),
	JoinError(tokio::task::JoinError),
	SerdeNix (crate::nix::Error),
	Custom   (String),
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use std::write;
		write!(f, "Error({:?})", self.err)
	}
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(
			&self.err,
			f
		)
	}
}
impl std::error::Error for Error {}

impl std::fmt::Debug for ErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    match self {
			Self::IO(e)        => std::fmt::Debug::fmt(e, f),
			Self::Reqwest(e)   => std::fmt::Debug::fmt(e, f),
			Self::JoinError(e) => std::fmt::Debug::fmt(e, f),
			Self::SerdeNix(e)  => std::fmt::Debug::fmt(e, f),
			Self::Custom(s)    => f.write_str(s),
		}
	}
}
impl std::fmt::Display for ErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	    match self {
			Self::IO(e)        => std::fmt::Display::fmt(e, f),
			Self::Reqwest(e)   => std::fmt::Display::fmt(e, f),
			Self::JoinError(e) => std::fmt::Display::fmt(e, f),
			Self::SerdeNix(e)  => std::fmt::Display::fmt(e, f),
			Self::Custom(s)    => f.write_str(s),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::with_error_kind(ErrorKind::IO(
			value
		))
	}
}
impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Self::with_error_kind(ErrorKind::Reqwest(
			value
		))
	}
}
impl From<tokio::task::JoinError> for Error {
	fn from(value: tokio::task::JoinError) -> Self {
		Self::with_error_kind(ErrorKind::JoinError(
			value
		))
	}
}
impl From<crate::nix::Error> for Error {
	fn from(value: crate::nix::Error) -> Self {
		Self::with_error_kind(ErrorKind::SerdeNix(
			value
		))
	}
}

pub type Result<T> = std::result::Result<T, Error>;
