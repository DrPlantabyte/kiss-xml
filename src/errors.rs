#![deny(unused_must_use)]
#![deny(missing_docs)]

/*!
The kiss_xml::error module holds an enum of possible error types, each of which
has a corresponding implementation struct.
*/

use std::fmt::{Debug, Display, Formatter};

/// Represents an error that occurs during parsing or processing of XML
#[derive(Debug)]
pub enum KissXmlError {
	/// This error indicates that there was a problem with the XML syntax or logic
	ParsingError(ParsingError),
	/// An I/O error when writing or reading a file
	IOError(std::io::Error),
}

impl Display for KissXmlError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			KissXmlError::ParsingError(e) => write!(f, "{}", e),
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

impl Display for ParsingError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "ParsingError: {}", self.msg)
	}
}

impl std::error::Error for ParsingError{}
