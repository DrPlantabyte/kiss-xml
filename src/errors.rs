#![deny(unused_must_use)]
#![deny(missing_docs)]
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Represents an error that occurs during parsing or processing of XML
#[derive(Debug)]
pub enum KissXmlError {
	ParsingError(ParsingError),
	IOError(std::io::Error),
}

impl Display for KissXmlError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			KissXmlError::ParsingError(e) => write!(f, "ParsingError: {}", e.msg),
			KissXmlError::IOError(e) => Display::fmt(&e, f)
		}
	}
}

impl std::error::Error for KissXmlError{}


/// Represents an error that occurs during parsing with additional information.
#[derive(Clone, Debug)]
pub struct ParsingError {
	/// The error message.
	pub msg: String
}

impl ParsingError{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}