#![deny(unused_must_use)]
#![deny(missing_docs)]

/*!
# KISS-XML: Keep It Super Simple XML

This Rust library provides an easy-to-use Document Object Model (DOM) for
reading and writing XML files. Unlike many other XML parsers, KISS-XML simply
parses the given XML to a full in-memory DOM, which you can then modify and
serialize back to XML. No schemas or looping required.

This library does not aim to support all XML specifications, only the most
commonly used subset of features.

## What's included:
KISS-XML provides the basics for XML documents, including:
* Parse XML files and strings to a DOM
* XML elements, text, and comments
* DOM is mutable and can be saved as a string and to files
* XML namespaces (with and without prefixes)
* Easy to use

## What's NOT included:
* Schema handling
* CDATA
* Document type declarations (DTDs will be preserved but not interpreted)
* Parsing character encodings other than UTF-8
* Typed XML data (eg integer attribute values)
* Performance optimizations (prioritizing easy-to-use over fast)

If you need any of the above XML features, then this library is too simple for
your needs. Try another XML parsing crate instead.

# Examples

## Parse an XML file and print it to the terminal
To parse an XML file, all you need to do is call the `kiss_xml::parse_filepath(...)` function, and you can convert it to a string with the `to_string()` method or write it to a file with `.write_to_filepath(...)`.

```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	let doc = kiss_xml::parse_filepath("my-file.xml")?;
	println!("{}", doc.to_string());
	Ok(())
}
```

## Parse XML and then search the DOM for specific elements
Parsed XML content will be converted into a Document Object Model (DOM) with a single root element. A DOM is a tree-like data structure made up of XML Element, Text, and Comment nodes. You can explore the DOM element-by-element with the `.elements_by_name(&str)` and `.first_element_by_name(&str)` methods, scan the children of an element with the `.child_*()` methods, or do a recursive search using the `.search(...)` and `.search_*(...)` methods.

For example:
```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	use kiss_xml;
	use kiss_xml::dom::*;
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<config>
	<name>My Settings</name>
	<sound>
		<property name="volume" value="11" />
		<property name="mixer" value="standard" />
	</sound>
<config>
"#;
	// parse XML to a document object model (DOM)
	let dom = kiss_xml::parse_str(xml)?;
	// print all sound properties
	let properties = dom.root_element()
		.first_element_by_name("sound")?
		.elements_by_name("property");
	for prop in properties {
		println!(
			"{} = {}",
			prop.get_attr("name")?,
			prop.get_attr("value")?
		);
	}
	// print children of the root element
	for e in dom.root_element().child_elements() {
		println!("child element <{}>", e.name())
	}
	// print all elements
	for e in dom.root_element().search_elements(|_| true) {
		println!("found element <{}>", e.name())
	}
	Ok(())
}
```

## Create and edit DOM from scratch
To modify the DOM, use the `.*_mut(...)` methods to get mutable references to the elements. You and insert, append, and remove elements (and other kinds of nodes) from the DOM.

For example:
```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	use kiss_xml;
	use kiss_xml::dom::*;
	// make a DOM from scratch
	let mut doc = Document::new(Element::new_from_name("politicians"));
	doc.root_element_mut().append(Element::new_with_text("person", "Hillary Clinton"));
	doc.root_element_mut().insert(0, Element::new_with_text("person", "John Adams"));
	doc.root_element_mut().append(Element::new_with_text("person", "Jimmy John"));
	doc.root_element_mut().append(Element::new_with_text("person", "Nanny No-Name"));
	// remove element by index
	let _removed_element = doc.root_element_mut().remove_element(3)?;
	// remove element(s) by use of a predicate function
	let _num_removed = doc.root_element_mut().remove_elements(|e| e.text()? == "Jimmy John");
	// print first element content
	println!("First politician: {}", doc.root_element().elements_by_name("person").text()?);
	// write to file
	doc.write_to_filepath("politics.xml");
	Ok(())
}
```

## Get and modify text and comments
The XML DOM is made up of Node objects (trait objects implementing trait kiss_xml::dom::Node). The following example shows how to add and remove text and comment nodes in addition to element nodes.

```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	use kiss_xml;
	use kiss_xml::dom::*;
	use std::collections::HashMap;
	let mut doc = kiss_xml::parse_str(
r#"<html>
	<!-- this is a comment ->
	<body>
		Content goes here
	</body>
</html>"#
	)?;
	// read and remove the first comment
	let first_comment = doc.root_element().children()
		.filter(|n| n.is_comment())
		.collect::<Vec<_>>().first()?;
	println!("Comment: {}", first_comment.text()?);
	doc.root_element_mut().remove_all(|n| n.is_comment());
	// replace content of <body> with some HTML
	doc.root_element_mut().first_element_by_name_mut("body").remove_all(|_| true);
	doc.root_element_mut().first_element_by_name_mut("body").append_all(
		&[
			&Element::new_with_text("h1", "Chapter 1"),
			&Comment::new("Note: there is only one chapter"),
			&Element::new_with_children("p", &[
				&Text::new("Once upon a time, there was a little "),
				&Element::new_with_attributes_and_text(
					"a",
					HashMap::from([("href","https://en.wikipedia.org/wiki/Gnome")]),
					"gnome"
				),
				&Text::new(" who lived in a walnut tree...")
			])
		]
	);
	// print the results
	println!("{}", doc.to_string());
}
```

# License
This library is open source, licensed under the MIT License. You may use it
as-is or with modification, without any limitations.

 */

use std::fs;
use std::io::Read;
use std::path::Path;

pub mod errors;
pub mod dom;


/// Escapes a subset of XML reserved characters (&, <, and >) in a text string
/// into XML-compatible text, eg replacing "&" with "&amp;" and "<" with "&lt;"
pub fn text_escape(text: impl Into<String>) -> String {
	let buffer: String = text.into();
	buffer.replace("&", "&amp;")
		.replace("<", "&lt;")
		.replace(">", "&gt;")
}

/// Escapes a subset of XML reserved characters (&, ', and ") in an attribute
/// into XML-compatible text, eg replacing "&" with "&amp;" and "'" with "&apos;"
pub fn attribute_escape(text: impl Into<String>) -> String {
	let buffer: String = text.into();
	buffer.replace("&", "&amp;")
		.replace("'", "&apos;")
		.replace("\"", "&quot;")
}

/// Escapes all special characters (&, <, >, ', and ") in a string into an
/// XML-compatible string, eg replacing "&" with "&amp;" and "<" with "&lt;"
pub fn escape(text: impl Into<String>) -> String {
	let buffer: String = text.into();
	buffer.replace("&", "&amp;")
		.replace("<", "&lt;")
		.replace(">", "&gt;")
		.replace("'", "&apos;")
		.replace("\"", "&quot;")
}

/// Reverses any escaped characters (&, <, >, ', and ") in XML-compatible text
/// to regenerate the original text, eg replacing "&amp;" with "&" and "&lt;"
/// with "<"
pub fn unescape(text: impl Into<String>) -> String {
	let buffer: String = text.into();
	buffer.replace("&amp;", "&")
		.replace("&lt;", "<")
		.replace("&gt;", ">")
		.replace("&apos;", "'")
		.replace("&quot;", "\"")
}

/** Reads the file from the given filepath and parses it as an XML document
*/
pub fn parse_filepath(path: impl AsRef<Path>) -> Result<dom::Document, errors::KissXmlError> {
	let path_ref = path.as_ref();
	let content = fs::read_to_string(path_ref)?;
	parse_str(content)
}

/** Reads the XML content from the given stream reader and parses it as an
XML document. Note that this function will read to EOF before returning.
 */
pub fn parse_stream(mut reader: impl Read) -> Result<dom::Document, errors::KissXmlError> {
	let mut buffer = String::new();
	reader.read_to_string(&mut buffer)?;
	parse_str(buffer)
}


/** Reads the XML content from the UTF-8 encoded text string and parses it as an XML document
 */
pub fn parse_str(xml_string: impl Into<String>) -> Result<dom::Document, errors::KissXmlError> {
	let buffer = xml_string.into();
	// first, look for XML declaration (if there is one)
	if buffer.starts_with("<?xml") {
		let tag
	}
	todo!()
}

/// finds next <> enclosed thing (or None if EoF is reached)
fn next_tag(buffer: &String, from: usize) -> (Option<usize>, Option<usize>) {
	let mut i = from;
	let mut start: Option<usize> = None;
	let mut end: Option<usize> = None;
	while start.is_none() && i < buffer.len() {
		if buffer[i] == '<' {
			start = Some(i);
		}
		i += 1;
	}
	if start.is_none() {
		return (None, None);
	}
	let start_index = start.unwrap();
	// the rules differ depending on the kind of tag
	let sub_buffer = &buffer[start_index..];
	if sub_buffer.starts_with("<!--") {
		// comment
		return (start, sub_buffer.find("-->").map(|i|i+start_index));
	} else if sub_buffer.starts_with("<?") {
		// declaration, look for ?> but handle quoting
		return (start, quote_aware_find(sub_buffer, "?>", 2).map(|i|i+start_index))
	} else if sub_buffer.starts_with("<![CDATA[") {
		// CDATA
		return (start, sub_buffer.find("]]>"));
	} else if sub_buffer.starts_with("<!") {
		// DTD or other XML weirdness, do nested search for closing >
		return (start, nested_quote_aware_find_close(sub_buffer,2).map(|i|i+start_index))
	} else {
		// normal element (we assume)
		todo!()
	}
	todo!()
}

fn quote_aware_find(text: &str, pattern: &str, from: usize) -> Option<usize> {
	let mut in_quote = false;
	let mut quote_char = '\0';
	for (i, c) in text[from..].char_indices() {
		if in_quote {
			if c == quote_char { // end of quoted field
				in_quote = false;
			}
		} else {
			if c == '"' { // start of double-quoted field
				quote_char = '"';
				in_quote = true;
			} else if c == '\'' { // start of single-quoted field
				quote_char = '\'';
				in_quote = true;
			} else if text[(from + i)..].starts_with(pattern) {
				return Some(from+i);
			}
		}
	}
	None
}


fn nested_quote_aware_find_close(text: &str, from: usize) -> Option<usize> {
	let mut depth: i32 = 0;
	let mut in_quote = false;
	let mut quote_char = '\0';
	for (i, c) in text[from..].char_indices() {
		if in_quote {
			if c == quote_char { // end of quoted field
				in_quote = false;
			}
		} else {
			if c == '"' { // start of double-quoted field
				quote_char = '"';
				in_quote = true;
			} else if c == '\'' { // start of single-quoted field
				quote_char = '\'';
				in_quote = true;
			} else if c == '<' {
				depth += 1;
			} else if c == '>' {
				if depth == 0 {
					return Some(from+i)
				}
				depth -= 1;
			}
		}
	}
	None
}

