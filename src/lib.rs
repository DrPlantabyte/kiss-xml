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
	let mut doc = Document::new(Element::new_from_name("politicians")?);
	doc.root_element_mut().append(Element::new_with_text("person", "Hillary Clinton")?);
	doc.root_element_mut().insert(0, Element::new_with_text("person", "John Adams")?);
	doc.root_element_mut().append(Element::new_with_text("person", "Jimmy John")?);
	doc.root_element_mut().append(Element::new_with_text("person", "Nanny No-Name")?);
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

use std::cell::OnceCell;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use regex::Regex;
use crate::dom::Element;

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
	escape(text)
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
	let mut buffer: String = text.into();
	for (i, c) in buffer.char_indices() {
		if c == '&' {
			let start = i;
			let slice = &buffer[i..];
			for (j, k) in slice.char_indices() {
				if k == ';' {
					let end = i + j + 1;
					let slice = &slice[..j];
					// note: trailing ; omitted from this slice
					if slice == "&amp" {
						buffer = string_insert(buffer.as_str(), (start, end), "&");
					}
					if slice == "&lt" {
						buffer = string_insert(buffer.as_str(), (start, end), "<");
					}
					if slice == "&gt" {
						buffer = string_insert(buffer.as_str(), (start, end), ">");
					}
					if slice == "&apos" {
						buffer = string_insert(buffer.as_str(), (start, end), "'");
					}
					if slice == "&quot" {
						buffer = string_insert(buffer.as_str(), (start, end), "\"");
					}
					if slice.starts_with("&#") {
						match u32::from_str_radix(&slice[2..], 16) {
							Ok(codepoint) => {
								match char::from_u32(codepoint) {
									Some(unicode) => {
										buffer = string_insert(buffer.as_str(), (start, end), unicode.to_string().as_str());
									},
									None => {/* do nothing */}
								}
							}
							Err(_) => {/* do nothing */}
						}
					}
				}
			}
		}
	}
	buffer
}

/// replaces indices (a, b) in given string with a new string and returns the new string
fn string_insert(buffer: &str, indices: (usize, usize), insert: &str) -> String{
	let mut new_str = buffer[..indices.0].to_string();
	new_str.push_str(insert);
	new_str.push_str(&buffer[indices.1..]);
	new_str
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
	let mut decl: Option<dom::Declaration> = None;
	let mut dtds: Vec<dom::DTD> = Vec::new();
	let mut no_comment_warn = 0;
	let mut tag_span: (usize, usize) = (0, 0);
	// parse decl and dtds, break on start of root element
	loop {
		let (tag_start, tag_end) = next_tag(&buffer, tag_span.1);
		if tag_start.is_none() {
			// not XML
			return Err(errors::ParsingError::new(format!("no XML content")).into());
		}
		if tag_end.is_none(){
			let (line, col) = line_and_column(&buffer, tag_start.unwrap());
			return Err(errors::ParsingError::new(format!(
				"invalid XML syntax on line {line}, column {col}: '<' has not matching '>'"
			)).into());
		}
		let tag_start = tag_start.unwrap();
		let tag_end = tag_end.unwrap();
		let text_between = &buffer[tag_span.1..tag_start];
		if real_text(text_between).is_some() {
			let (line, col) = line_and_column(&buffer, tag_span.1);
			return Err(errors::ParsingError::new(format!(
				"invalid XML syntax on line {line}, column {col}: Text outside the root element is not supported"
			)).into());
		}
		let slice = &buffer[tag_start..tag_end];
		if slice.starts_with("<?xml") {
			if tag_span.0 != 0 {
				let (line, col) = line_and_column(&buffer, tag_start.unwrap());
				return Err(errors::ParsingError::new(format!(
					"invalid XML syntax on line {line}, column {col}: <?xml ...?> declaration must at start of XML"
				)).into());
			}
			decl = Some(dom::Declaration::from_str(slice)?);
		} else if slice.starts_with("<!--") {
			// comments outside root element not supported
			if no_comment_warn == 0 {
				eprintln!("WARNING: Encountered comment {slice} outside of root element. Comments outside of the root are not supported and will be ignored.");
			}
			no_comment_warn += 1;
		} else if slice.starts_with("<!DOCTYPE") {
			// DTD
			let dtd = dom::DTD::from_string(slice);
			dtds.push(dtd);
		} else if slice.starts_with("<!"){
			// some other XML mallarky
			eprintln!("WARNING: Ignoring {slice} (not supported outside root element)");
		} else if slice.starts_with("</") {
			// bad XML
			let (line, col) = line_and_column(&buffer, tag_start.unwrap());
			return Err(errors::ParsingError::new(format!(
				"invalid XML syntax on line {line}, column {col}: cannot start with closing tag"
			)).into());
		} else {
			// root element?
			check_element_tag(slice).map_err(move |e| {
				let (line, col) = line_and_column(&buffer, tag_start.unwrap());
				Err(errors::ParsingError::new(format!(
					"invalid XML syntax on line {line}, column {col}"
				)).into())
			})?;
			tag_span = (tag_start, tag_end);
			break;
		}
		tag_span = (tag_start, tag_end);
	}
	// now parse the elements, keeping a stack of parents as the tree is traversed
	let mut root_element: Option<Element> = None;
	let mut e_stack: Vec<&dom::Element> = Vec::new();
	let mut last_span: (usize, usize) = (tag_span.0, tag_span.0);
	loop {
		// get text since last tag
		let text = &buffer[last_span.1 .. tag_span.0];
		// if text is not empty, add text node
		match real_text(text) {
			None => {},
			Some(content) => {
				e_stack.last_mut().unwrap().append(dom::Text::new(content))
			}
		};
		// parse span
		let slice = &buffer[tag_span.0 .. tag_span.1];
		if slice.starts_with("<!--") && slice.ends_with("-->") {
			// comment
			let c = dom::Comment::new(&slice[4 .. slice.len().saturating_sub(3)]);
			e_stack.last_mut().unwrap().append(c)
		} else if slice.starts_with("<!") {
			// CDATA or other unsupported thing
			let (line, col) = line_and_column(&buffer, tag_span.0);
			Err(errors::NotSupportedError::new(format!(
				"error on line {line}, column {col}: kiss-xml does not support '{slice}'"
			)).into())
		} else {
			// element
			// sanity check
			check_element_tag(slice).map_err(move |e| {
				let (line, col) = line_and_column(&buffer, tag_span.0);
				Err(errors::ParsingError::new(format!(
					"invalid XML syntax on line {line}, column {col}"
				)).into())
			})?;
			// is it a closing tag? If so, pop the parent stack
			if slice.starts_with("</") {
				let tagname = strip_tag(slice);
				if e_stack.len() == 0 {
					let (line, col) = line_and_column(&buffer, tag_span.0);
					return Err(errors::ParsingError::new(format!(
						"invalid XML syntax on line {line}, column {col}: cannot start with a closing tag"
					)).into());
				}
				let open_tagname = e_stack.last().unwrap().tag_name();
				if tagname != open_tagname {
					let (line, col) = line_and_column(&buffer, tag_span.0);
					return Err(errors::ParsingError::new(format!(
						"invalid XML syntax on line {line}, column {col}: closing tag {slice} does not match <{open_tagname}>"
					)).into());
				}
				e_stack.pop();
				// check end condition
				if e_stack.is_empty(){
					break;
				}
			} else {
				// add new element to the stack
				let tag_content = strip_tag(slice);
				let components = quote_aware_split(tag_content.as_str());
				if components.len() == 0 {
					let (line, col) = line_and_column(&buffer, tag_span.0);
					return Err(errors::ParsingError::new(format!(
						"invalid XML syntax on line {line}, column {col}: empty tags not supported"
					)).into());
				}
				// parse attributes
				let mut attrs: HashMap<String, String> = HashMap::new();
				for i in 1..components.len() {
					let kv = &components[i];
					if !kv.contains("=") {
						let (line, col) = line_and_column(&buffer, tag_span.0);
						return Err(errors::ParsingError::new(format!(
							"invalid XML syntax on line {line}, column {col}: attributes must be in the form 'key=\"value\"'"
						)).into());
					}
					let (k, mut v) = kv.split_once("=").unwrap();
					// note: v string contains enclosing quotes
					v = &v[1..(v.len()-1)]; // remove quotes
					attrs.insert(k, v);
				}
				// parse name and namespace
				let mut name = components[0].as_str();
				let mut xmlns: Option<String> = None;
				let mut xmlns_prefix: Option<String> = None;
				// check parent for inherited namespaces
				let (inherited_default_namespace, inherited_xmlns_context) = match e_stack.last() {
					None => (None, None),
					Some(parent) => (parent.default_namespace(), parent.get_namespace_context())
				};
				if name.contains(":"){
					let (a, b) = name.split_once(":").unwrap();
					name = b;
					xmlns_prefix = Some(a.to_string());
					// check if the prefix is in attributes or inherited from parent
					let prefix_key = format!("xmlns:{a}");
					xmlns = match attrs.contains_key(&prefix_key){
						true => attrs.get(prefix_key.as_str()).clone(),
						false => match &inherited_xmlns_context{
							None => {
								let (line, col) = line_and_column(&buffer, tag_span.0);
								return Err(errors::ParsingError::new(format!(
									"invalid XML syntax on line {line}, column {col}: XML namespace prefix '{a}' has no defined namespace (missing 'xmlns:{a}=\"...\"')"
								)).into());
							}
							Some(ctx) => {ctx.get(prefix_key.as_str()).clone()}
						}
					};
				}
				let mut new_element = dom::Element::new(
					name, None, Some(attrs), xmlns.map(String::as_str), xmlns_prefix.map(String::as_str), None
				)?;
				new_element.set_namespace_context(inherited_default_namespace, inherited_xmlns_context);
				// push new element to stack if it is not self-closing
				match slice.ends_with("/>") {
					true => {
						// self-closing tag, don't add to stack
					}
					false => e_stack.push(&new_element)
				}
				// append new element to parent
				match e_stack.last_mut() {
					None => {
						// first element, no parent to add to, set root
						root_element = Some(new_element);
						e_stack.push(&new_element);
					},
					Some(parent) => parent.append(new_element)
				}
			}
		}
		// find next tag, repeat
		let next_span = next_tag(&buffer, tag_span.1);
		if next_span.0.is_none() {
			// EoF
			break
		} else if next_span.1.is_none() {
			// broken tag?
			let (line, col) = line_and_column(&buffer, next_span.0.unwrap());
			Err(errors::ParsingError::new(format!(
				"invalid XML syntax on line {line}, column {col}"
			)).into())
		} else {
			tag_span = (next_span.0.unwrap(), next_span.1.unwrap());
		}
	}
	// error check
	if root_element.is_none() {
		return Err(errors::ParsingError::new(format!(
			"invalid XML syntax: no root element"
		)).into())
	}
	// return a DOM document
	Ok(dom::Document::new_with_decl_dtd(
		root_element.expect("logic error"),
		decl,
		match &dtds.is_empty(){
			true => None,
			false => Some(dtds)
		}
	))
}

/// removes leading and trailing <> and/or /
fn strip_tag(tag: &str) -> String {
	let mut tag = tag;
	if tag.starts_with("<") {tag = &tag[1..];}
	if tag.starts_with("/") {tag = &tag[1..];}
	if tag.ends_with(">") {tag = &tag[..tag.len().saturating_sub(1)];}
	if tag.ends_with("/") {tag = &tag[..tag.len().saturating_sub(1)];}
	tag.trim().to_string()
}


/// singleton regex matcher
const ELEM_MATCHER_SINGLETON: OnceCell<Regex> = OnceCell::new();
/// checks if a tag has valid syntax for an element (does not parse)
fn check_element_tag(text: &str) -> Result<(), errors::KissXmlError> {
	let singleton = ELEM_MATCHER_SINGLETON;
	let matcher = singleton.get_or_init(||{
		// see https://www.w3.org/TR/REC-xml/#sec-common-syn
		let name_start_char = r#"[:A-Z_a-z\xC0-\xD6\xD8-\xF6\xF8-\x{2FF}\x{370}-\x{37D}\x{37F}-\x{1FFF}\x{200C}-\x{200D}\x{2070}-\x{218F}\x{2C00}-\x{2FEF}\x{3001}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFFD}\x{10000}-\x{EFFFF}]"#;
		let name_char = r#"[A-Z_a-z\xC0-\xD6\xD8-\xF6\xF8-\x{2FF}\x{370}-\x{37D}\x{37F}-\x{1FFF}\x{200C}-\x{200D}\x{2070}-\x{218F}\x{2C00}-\x{2FEF}\x{3001}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFFD}\x{10000}-\x{EFFFF}.\-0-9\xB7\x{0300}-\x{036F}\x{203F}-\x{2040}]"#;
		let pattern = format!(r#"(?ms)<{name_start_char}{name_char}*(:{name_start_char}{name_char}*)?(\s+{name_start_char}{name_char}*=(".*?"|'.*?'))*\s*/?>"#);
		Regex::new(pattern.as_str()).unwrap()
	});
	match matcher.is_match(text){
		true => Ok(()),
		false => Err(errors::ParsingError::new("Invalid XML Element").into())
	}
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
		return (start, quote_aware_find(sub_buffer, "?>", 2).map(|i|i+start_index+2))
	} else if sub_buffer.starts_with("<![CDATA[") {
		// CDATA
		return (start, sub_buffer.find("]]>").map(|i|i+start_index+3));
	} else if sub_buffer.starts_with("<!") {
		// DTD or other XML weirdness, do nested search for closing >
		return (start, nested_quote_aware_find_close(sub_buffer,2).map(|i|i+start_index+1))
	} else {
		// normal element tag (we assume)
		return (start, quote_aware_find(sub_buffer, ">", 1).map(|i|i+start_index+1))
	}
}

/// splits by whitespace, respecting quotes
fn quote_aware_split(text: &str) -> Vec<String> {
	let mut builder = String::new();
	let mut vec: Vec<String> = Vec::new();
	let mut in_quote = false;
	let mut quote_char = '\0';
	for (i, c) in text.char_indices() {
		if !in_quote && (c == '\'' || c == '"') {
			// start of quoted text
			in_quote = true;
			quote_char = c;
			builder.push(c);
		} else if in_quote {
			// quoted text
			builder.push(c);
			if c == quote_char {
				// end of quoted text
				in_quote = false;
			}
		} else if c.is_whitespace() {
			// break on whitespace
			if builder.len() > 0 {
				vec.push(builder);
				builder = String::new();
			}
		} else {
			// normal text
			builder.push(c);
		}
	}
	return vec;
}
/// like `String.find()` but skipping quoted content
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

/// like `quote_aware_find()` above, but the pattern is '>' and it skips both quoted content and nested <tags>
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


/// singleton regex matcher
const IS_BLANK_MATCHER_SINGLETON: OnceCell<Regex> = OnceCell::new();
/// singleton regex matcher
const INDENTED_LINE_MATCHER_SINGLETON: OnceCell<Regex> = OnceCell::new();
/// extracts the actual text (accounting for indenting) from a string slice,
/// returning None if it is all whitespace
fn real_text(text: &str) -> Option<String> {
	// check for empty string
	let singleton = IS_BLANK_MATCHER_SINGLETON;
	let matcher = singleton.get_or_init(|| Regex::new(r#"^\s*$"#).unwrap());
	if matcher.is_match(text) {
		return None;
	}
	// extract actual text
	let singleton = INDENTED_LINE_MATCHER_SINGLETON;
	let matcher = singleton.get_or_init(|| Regex::new(r#"\n\s*"#).unwrap());
	let text = matcher.replace(text, "\n");
	Some(text.trim_start().to_string())
}

/// get line and column number for index to use for error reporting
fn line_and_column(text: &String, pos: usize) -> (usize, usize){
	let mut line = 1;
	let mut col = 1;
	for (i, c) in text.char_indices(){
		col += 1;
		if c == '\n' {
			line += 1;
			col = 1;
		}
		if i >= pos {break;}
	}
	(line, col)
}
