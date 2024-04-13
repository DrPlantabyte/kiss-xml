#![deny(unused_must_use)]
#![deny(missing_docs)]

/*!
A document object model (DOM) is a tree data structure with three different kinds of nodes: Element, Text, and Comment nodes. Element nodes can have children (a list of child nodes), while Text and Comment nodes cannot. As per the XML specification, a DOM can only have one root element.

# Examples

## Parse an XML file, modify it, then print it to the terminal
```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	use kiss_xml;
	use kiss_xml::dom::Element;
	let mut doc = kiss_xml::parse_filepath("some-file.xml")?;
	doc.root_element_mut().append(Element::new_with_text("note", "note text"));
	println!("{}", doc.to_string_with_indent("\t"));
	Ok(())
}
```

## Create a new DOM from scratch
```rust
fn main() -> Result<(), kiss_xml::errors::KissXmlError> {
	use kiss_xml;
	use kiss_xml::dom::*;
	use chrono::{DateTime, Utc};
	let mut doc = Document::new(
		Element::new_with_children("root", &[
			Comment::new(format!("This XML document was generated on {}", Utc::now().to_rfc3339())),
			Element::new_with_text("motd", "Message of the day is: hello!")
		])
	);
	println!("{}", doc.to_string_with_indent("\t"));
	Ok(())
}
```

*/

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::slice::Iter;
use http::Uri;

/**
A Document represents a DOM plus additional (optional) metadata such as one or more Document Type Declarations (DTD). Use this struct to write a DOM to a string or file.
*/
pub struct Document {
	// TODO
}

impl Document {
	/**
Constructs a new Document with the given root element and default declaration
	 */
	pub fn new(root: Element) -> Self {
		todo!()
	}
	/**
Full constructor with required root element and optional XML declaration and optional list of one or more document type definition (DTD) items.
	 */
	pub fn new_with_decl_dtd(root: Element, declaration: Option<Declaration>, dtd: Option<&[DTD]>) -> Self {
		todo!()
	}
	/**
Returns a list of any and all DTDs for this Document as an iterator
	 */
	pub fn doctype_defs(&self) -> Iter<DTD> {
		todo!()
	}
	/**
Sets the DTDs for this document (a `None` argument will remove all DTDs)
	 */
	pub fn set_doctype_defs(&mut self, dtds: Option<&[DTD]>) {
		todo!()
	}
	/**
Gets the XML declaration for this document, if it has one (while the XML spec requires a declaration at the start of every XML file, it is commonly omitted, especially when the XML is embedded in a stream or file).
	 */
	pub fn declaration(&self) -> Option<&Declaration> {
		todo!()
	}
	/**
Sets the XML declaration for this document (a `None` argument will remove any existing declaration). While the XML spec requires a declaration at the start of every XML file, it is commonly omitted, especially when the XML is embedded in a stream or file.
	 */
	pub fn set_declaration(&mut self, decl: Declaration) {
		todo!()
	}

	/**
Produces the XML text representing this XML DOM using the default indent of two spaces per level
	 */
	pub fn to_string(&self) -> String {
		self.to_string_with_indent("  ")
	}

	/**
	Produces the XML text representing this XML DOM using the provided indent
	 */
	pub fn to_string_with_indent(&self, indent: impl Into<String>) -> String {
		todo!()
	}

	/**
	Writes this document as XML to the given file using the default indent of two spaces per level, returning a result indicating success or error in this write operation
	*/
	pub fn write_to_filepath(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
		self.write_to_filepath_with_indent(path, "  ")
	}

	/**
	Writes this document as XML to the given file using the provided indent, returning a result indicating success or error in this write operation
	 */
	pub fn write_to_filepath_with_indent(&self, path: impl AsRef<Path>, indent: impl Into<String>) -> std::io::Result<()> {
		todo!()
	}

	/**
	Writes this document as XML to the given file using the default indent of two spaces per level, returning a result indicating success or error in this write operation
	 */
	pub fn write_to_file(&self, file: &File, indent: impl Into<String>) -> std::io::Result<()> {
		self.write_to_file_with_indent(file, "  ")
	}

	/**
	Writes this document as XML to the given file using the default indent of two spaces per level, returning a result indicating success or error in this write operation
	 */
	pub fn write_to_file_with_indent(&self, file: &File, indent: impl Into<String>) -> std::io::Result<()> {
		todo!()
	}

	/**
	Returns the root element of this DOM as an immutable reference
	 */
	pub fn root_element(&self) -> &Element {
		todo!()
	}

	/**
	Returns the root element of this DOM as a mutable reference.
	  */
	pub fn root_element_mut(&mut self) -> &mut Element {
		todo!()
	}
}

impl std::fmt::Display for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl PartialEq<Self> for Document {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

/**
A node in the DOM tree. Elements, Comments, and Text are all types of nodes, but only Elements can be branch nodes with children of their own.
 */
pub trait Node: dyn_clone::DynClone + std::fmt::Debug + std::fmt::Display {

	/**
If this node is an Element, this returns the tag name of the element (eg "author" for XML element `<author />`). Otherwise, this returns None.
	 */
	fn name(&self) -> Option<String>;

	fn text(&self) -> Option<String>;

	fn is_element(&self) -> bool;

	fn is_text(&self) -> bool;

	fn is_comment(&self) -> bool;

	fn as_element(&self) -> Result<>
}

#[derive(Clone)]
pub struct Element {
	// TODO
}

impl Element {

	pub fn new(name: &str, text: Option<&str>, attributes: Option<HashMap<&str, &str>>, xmlns: Option<&str>, xmlns_prefix: Option<&str>, children: Option(&[&dyn Node])) -> Self {todo!()}

	pub fn new_from_name(name: &str) -> Self {
		todo!()
	}

	pub fn new_with_attributes(name: &str, attributes: HashMap<&str, &str>) -> Self {
		todo!()
	}

	pub fn new_with_text(name: &str, text: &str) -> Self {
		todo!()
	}

	pub fn new_with_attributes_and_text(name: &str, attributes: HashMap<&str, &str>, text: &str) -> Self {
		todo!()
	}

	pub fn new_with_attributes_and_children(name: &str, attributes: HashMap<&str, &str>, children: &[&dyn Node]) -> Self {todo!()}

	pub fn new_with_children(name: &str, children: &[&dyn Node]) -> Self {todo!()}

	pub fn namespace(&self) -> Option<Uri> {
		todo!()
	}

	pub fn namespace_prefix(&self) -> Option<String> {
		todo!()
	}

	pub fn elements_by_namespace(&self, namespace: Option<&Uri>) -> Iter<Element>{
		todo!()
	}

	pub fn elements_by_namespace_prefix(&self, prefix: Option<&str>) -> Iter<Element>{
		todo!()
	}

	fn namespace_prefixes(&self) -> &Option<HashMap<String, String>> {todo!()}

	fn set_namespace_context(&mut self, parent_namespace: Option<String>, parent_prefixes: Option<HashMap<String, String>>) { todo!()}

	pub fn child_elements(&self) -> Iter<Element>{
		todo!()
	}

	pub fn children(&self) -> Iter<Box<dyn Node>>{
		todo!()
	}

	pub fn first_element_by_name(&self, name: impl Into<String>) -> Option<&Element> {
		todo!()
	}

	pub fn first_element_by_name_mut(&mut self, name: impl Into<String>) -> Option<&mut Element> {
		todo!()
	}

	pub fn elements_by_name(&self, name: impl Into<String>) -> Iter<Element>{
		todo!()
	}

	pub fn attributes(&self) -> &HashMap<&str, &str> {
		todo!()
	}

	pub fn get_attr(&self, attr_name: impl Into<String>) -> Option<String> {
		todo!()
	}

	pub fn set_attr(&mut self, attr_name: impl Into<String>, value: impl Into<String>) {
		todo!()
	}

	pub fn remove_attr(&mut self, attr_name: impl Into<String>) -> Option<String> {
		todo!()
	}

	pub fn search<P>(&self, predicate: P) -> Iter<Box<dyn Node>> where P: FnMut(&Box<dyn Node>) -> bool {
		// recursive
		todo!()
	}

	pub fn search_elements<P>(&self, predicate: P) -> Iter<Element> where P: FnMut(&Element) -> bool {
		// recursive
		todo!()
	}

	pub fn search_elements_by_name(&self, name: impl Into<String>) -> Iter<Element>{
		// recursive
		todo!()
	}

	pub fn search_text<P>(&self, predicate: P) -> Iter<Text> where P: FnMut(&Text) -> bool {
		// recursive
		todo!()
	}

	pub fn search_comments<P>(&self, predicate: P) -> Iter<Comment> where P: FnMut(&Comment) -> bool {
		// recursive
		todo!()
	}

	pub fn append(&mut self, node: impl Node) {
		todo!()
		// TODO: if this is an element, set the namespace context
	}

	pub fn append_all(&mut self, children: &[&dyn Node]) {
		todo!()
	}

	pub fn insert(&mut self, index: usize, node: impl Node) {
		todo!()
		// TODO: if this is an element, set the namespace context
	}

	pub fn remove(&mut self, index: usize) -> Option<Box<dyn Node>> {
		todo!()
	}

	pub fn remove_all<P>(&mut self, predicate: P) -> usize where P: FnMut(&dyn Node) -> bool {
		// recursive, returns count
		todo!()
	}

	pub fn remove_all_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// recursive, returns count
		todo!()
	}

	pub fn remove_element(&mut self, index: usize) -> Option<Element> {
		todo!()
	}

	pub fn remove_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// returns count
		todo!()
	}

	pub fn remove_elements_by_name(&mut self, name: impl Into<String>) -> usize {
		// returns count
		todo!()
	}

}

impl Node for Element{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Element {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Element {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}


impl std::fmt::Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone)]
pub struct Text {
	pub content: String
}

impl Text {
	pub fn new(text: impl Into<String>) -> Self {
		todo!()
	}

	pub fn name(&self) -> String {
		todo!()
	}
}

impl Node for Text{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Text {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Text {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Text {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}


impl std::fmt::Display for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone)]
pub struct Comment{
	pub content: String
}

impl Comment {
	pub fn new(comment: impl Into<String>) -> Self {
		todo!()
	}

}

impl Node for Comment{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Comment {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Comment {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Comment {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}

impl std::fmt::Display for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Declaration {
	// TODO
}

impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct DTD {
	// TODO
}

impl std::fmt::Display for DTD {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
