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
	/// This error indicates an attempt at an invalid conversion from one type
	/// of Node to another (eg trying to cast an Element to a Comment)
	TypeCastError(TypeCastError),
	/// This error indicates that the user requested something that wasn't there
	DoesNotExistError(DoesNotExistError),
	/// This error indicates that the user requested an invalid index in a collection or slice
	IndexOutOfBounds(IndexOutOfBounds),
	/// An I/O error when writing or reading a file
	IOError(std::io::Error),
	/// An error when parsing an XML namespace URI
	InvalidNamespaceUri(http::uri::InvalidUri),
}

impl Display for KissXmlError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			KissXmlError::ParsingError(e) => write!(f, "{}", e),
			KissXmlError::TypeCastError(e) => write!(f, "{}", e),
			KissXmlError::DoesNotExistError(e) => write!(f, "{}", e),
			KissXmlError::IndexOutOfBounds(e) => write!(f, "{}", e),
			KissXmlError::IOError(e) => Display::fmt(&e, f),
			KissXmlError::InvalidNamespaceUri(e) => Display::fmt(&e, f)
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

/// Error indicating an attempt to convert a Node to the wrong implementing type (eg turning an Element into a Comment)
#[derive(Clone, Debug)]
pub struct TypeCastError {
	/// The error message.
	pub msg: String
}

impl TypeCastError{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}

impl Display for TypeCastError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "TypeCastError: {}", self.msg)
	}
}

impl std::error::Error for TypeCastError{}


/// Error indicating an attempt to convert a Node to the wrong implementing type (eg turning an Element into a Comment)
#[derive(Clone, Debug)]
pub struct DoesNotExistError {
	/// The error message.
	pub msg: String
}

impl DoesNotExistError{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}

impl Display for DoesNotExistError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "DoesNotExistError: {}", self.msg)
	}
}

impl Default for DoesNotExistError {
	fn default() -> Self {
		Self{msg: String::from("requested item not found")}
	}
}

impl std::error::Error for DoesNotExistError{}

/// Error indicating an attempt to index an array or collection with an invalid index
#[derive(Clone, Debug)]
pub struct IndexOutOfBounds {
	/// The error index
	pub index: isize,
	/// optional correct bounds
	pub bounds: Option<(isize, isize)>
}

impl IndexOutOfBounds{
	/// New error with a given index
	pub fn new(index: isize, bounds: Option<(isize, isize)>) -> Self {
		Self{index, bounds}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.bounds {
			Some(b) => write!(f, "Index {} is out of bounds (valid range: {} - {})", &self.index, b.0, b.1),
			None => write!(f, "Index {} is out of bounds", &self.index)
		}
	}
}

impl Display for IndexOutOfBounds {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.print(f)
	}
}

impl std::error::Error for IndexOutOfBounds{}

