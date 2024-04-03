#![deny(unused_must_use)]
#![deny(missing_docs)]

use std::path::Path;

mod errors;
pub mod dom;
use dom::Node;

pub fn mock() -> i32 {42}


/// Escapes a subset of XML reserved characters (&, <, and >) in a text string
/// into XML-compatible text, eg replacing "&" with "&amp;" and "<" with "&lt;"
pub fn text_escape(text: impl Into<String>) -> String {
	todo!()
}

/// Escapes a subset of XML reserved characters (&, ', and ") in an attribute
/// into XML-compatible text, eg replacing "&" with "&amp;" and "'" with "&apos;"
pub fn attribute_escape(text: impl Into<String>) -> String {
	todo!()
}

/// Escapes all special characters (&, <, >, ', and ") in a string into an
/// XML-compatible string, eg replacing "&" with "&amp;" and "<" with "&lt;"
pub fn escape(text: impl Into<String>) -> String {
	todo!()
}

/// Reverses any escaped characters (&, <, >, ', and ") in XML-compatible text
/// to regenerate the original test, eg replacing "&amp;" with "&" and "&lt;"
/// with "<"
pub fn unescape(text: impl Into<String>) -> String {
	todo!()
}

/// Reads the file from the given filepath and parses it as an XML document
pub fn read_from_filepath(path: &Path) -> Result<dom::Document, errors::KissXmlError> {
	todo!()
}