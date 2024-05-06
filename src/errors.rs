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
	/// This error indicates an attempt to use an attribute that is not valid
	InvalidAttributeName(InvalidAttributeName),
	/// This error indicates an attempt to create an element with a name that is not valid
	InvalidElementName(InvalidElementName),
	/// Error indicating an attempt to do something that is valid XML, but not supported by KISS-XML
	NotSupportedError(NotSupportedError),
	/// An I/O error when writing or reading a file
	IOError(std::io::Error),
}

impl From<std::io::Error> for KissXmlError {
	fn from(e: std::io::Error) -> Self {KissXmlError::IOError(e)}
}

impl Display for KissXmlError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			KissXmlError::ParsingError(e) => write!(f, "{}", e),
			KissXmlError::TypeCastError(e) => write!(f, "{}", e),
			KissXmlError::DoesNotExistError(e) => write!(f, "{}", e),
			KissXmlError::IndexOutOfBounds(e) => write!(f, "{}", e),
			KissXmlError::InvalidAttributeName(e) => write!(f, "{}", e),
			KissXmlError::InvalidElementName(e) => write!(f, "{}", e),
			KissXmlError::NotSupportedError(e) => write!(f, "{}", e),
			KissXmlError::IOError(e) => write!(f, "{}", e),
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

impl From<ParsingError> for KissXmlError {
	fn from(e: ParsingError) -> Self {KissXmlError::ParsingError(e)}
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

impl From<TypeCastError> for KissXmlError {
	fn from(e: TypeCastError) -> Self {KissXmlError::TypeCastError(e)}
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

impl From<DoesNotExistError> for KissXmlError {
	fn from(e: DoesNotExistError) -> Self {KissXmlError::DoesNotExistError(e)}
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

impl From<IndexOutOfBounds> for KissXmlError {
	fn from(e: IndexOutOfBounds) -> Self {KissXmlError::IndexOutOfBounds(e)}
}

impl Display for IndexOutOfBounds {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.print(f)
	}
}

impl std::error::Error for IndexOutOfBounds{}

/// Error indicating an attempt to add an attribute with an invalid name (eg contains a space)
#[derive(Clone, Debug)]
pub struct InvalidAttributeName {
	/// The error message.
	pub msg: String
}

impl InvalidAttributeName{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}

impl From<InvalidAttributeName> for KissXmlError {
	fn from(e: InvalidAttributeName) -> Self {KissXmlError::InvalidAttributeName(e)}
}

impl Display for InvalidAttributeName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "InvalidAttributeName: {}", self.msg)
	}
}

impl std::error::Error for InvalidAttributeName{}

/// Error indicating an attempt to create an element with an invalid name (eg contains a space)
#[derive(Clone, Debug)]
pub struct InvalidElementName {
	/// The error message.
	pub msg: String
}

impl InvalidElementName{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}

impl From<InvalidElementName> for KissXmlError {
	fn from(e: InvalidElementName) -> Self {KissXmlError::InvalidElementName(e)}
}

impl Display for InvalidElementName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "InvalidElementName: {}", self.msg)
	}
}

impl std::error::Error for InvalidElementName{}


/// Error indicating an attempt to do something that is valid XML, but not supprted by KISS-XML
#[derive(Clone, Debug)]
pub struct NotSupportedError {
	/// The error message.
	pub msg: String
}

impl NotSupportedError{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
	/// Formats and prints the error message
	fn print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.msg)
	}
}

impl From<NotSupportedError> for KissXmlError {
	fn from(e: NotSupportedError) -> Self {KissXmlError::NotSupportedError(e)}
}

impl Display for NotSupportedError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "NotSupportedError: {}", self.msg)
	}
}

impl std::error::Error for NotSupportedError{}


