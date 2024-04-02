#![deny(unused_must_use)]
#![deny(missing_docs)]

use std::path::Path;

mod errors;
mod dom;
use dom::Node;

pub fn mock() -> i32 {42}


/// Escapes a text string into XML-compatible text, eg replacing "&" with "&amp;" and "<" with "&lt;"
pub fn escape(text: impl Into<String>) -> String {
	todo!()
}

/// Reverses any escaped characters in XML-compatible text to regenerate the original test, eg replacing "&amp;" with "&" and "&lt;" with "<"
pub fn unescape(text: impl Into<String>) -> String {
	todo!()
}

pub fn read_from_filepath(path: &Path) -> Result<Node::Element, errors::KissXmlError> {
	todo!()
}