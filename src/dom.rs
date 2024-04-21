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
use std::slice::{Iter, IterMut};
use http::Uri;
use crate::errors::*;

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
	Returns a list of any and all DTDs for this Document as an iterator
	 */
	pub fn doctype_defs_mut(&mut self) -> IterMut<DTD> {
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
	Returns the text content of the code. For a Comment or Text node, this is just the comment or text string. For an Element, this will return *all* text (including from child elements, recursive scan) as a single string, or `None` if this element has no child text nodes
	 */
	fn text(&self) -> Option<String>;

	/**
	Returns `true` if this Node trait object is an Element struct, otherwise `false`
	 */
	fn is_element(&self) -> bool;

	/**
	Returns `true` if this Node trait object is a Text struct, otherwise `false`
	 */
	fn is_text(&self) -> bool;

	/**
	Returns `true` if this Node trait object is a Comment struct, otherwise `false`
	 */
	fn is_comment(&self) -> bool;

	/**
	Casts this Node to an Element struct (if the Node is not an Element struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_element(&self) -> Result<&Element, TypeCastError> {
		todo!()
	}

	/**
	Casts this Node to a Comment struct (if the Node is not an Comment struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_comment(&self) -> Result<&Comment, TypeCastError> {
		todo!()
	}

	/**
	Casts this Node to a Text struct (if the Node is not a Text struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_text(&self) -> Result<&Text, TypeCastError> {
		todo!()
	}

	/**
	Casts this Node to an Element struct (if the Node is not an Element struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_element_mut(&mut self) -> Result<&mut Element, TypeCastError> {
		todo!()
	}

	/**
	Casts this Node to a Comment struct (if the Node is not an Comment struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_comment_mut(&mut self) -> Result<&mut Comment, TypeCastError> {
		todo!()
	}

	/**
	Casts this Node to a Text struct (if the Node is not a Text struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_text_mut(&mut self) -> Result<&mut Text, TypeCastError> {
		todo!()
	}

	/**
	Casts this struct to a Node trait object
	 */
	fn as_node(&self) -> &dyn Node where Self: Sized {
		self
	}

	/**
	Casts this struct to a Node trait object
	 */
	fn as_node_mut(&mut self) -> &mut dyn Node where Self: Sized {
		self
	}

	/**
	Writes this Node to a string with the provided indent (used to serialize to XML)
	 */
	fn as_string_with_indent(&self, indent: &str) -> String;

	/**
	Writes this Node to a string with the default indent of two spaces per level (used to serialize to XML)
	 */
	fn as_string(&self) -> String {
		self.as_string_with_indent("  ")
	}
}

/// Represents an XML element with a name, text content, attributes, xmlns namespace (with optional prefix), and children.
#[derive(Clone)]
pub struct Element {
	// TODO
}

impl Element {
	/**
	Creates a new Element
	# Args:
	* *name*: Element name for this XML element (ie "body" for `<body>some text</body>`)
	* *text*: Optional text content for this element (ie "some text" for `<body>some text</body>`)
	* *attributes*: optional HashMap of attributes
	* *xmlns*: optional namespace for this element
	* *xmlns_prefix*: optional namespace prefix (if `xmlns` is not `None` but `xmlns_prefix` is `None`, then this element will set it's xmlns as the default xlmns for it and its children)
	* *children*: optional list of child nodes to add to this element
	 */
	pub fn new<TEXT1: Into<String>, TEXT2: Into<String>>(name: &str, text: Option<&str>, attributes: Option<HashMap<TEXT1, TEXT2>>, xmlns: Option<&str>, xmlns_prefix: Option<&str>, children: Option<&[&dyn Node]>) -> Self {todo!()}
	/// Creates a new Element with the specified name and not attributes or content.
	pub fn new_from_name(name: &str) -> Self {
		todo!()
	}
	/** Creates a new Element with the specified name and attributes.
	# Example
	```rust
	fn main() {
		use kiss_xml::dom::*;
		use std::collections::HashMap;
		let e = Element::new_with_attributes("b", HashMap::from(&[
			("style", "color: blue")
		]));
		println!("{}", e) // prints `<b style="color: blue"/>`
	}
	```
	 */
	pub fn new_with_attributes(name: &str, attributes: HashMap<impl Into<String>, impl Into<String>>) -> Self {
		todo!()
	}
	/// Creates a new Element with the specified name and text content
	pub fn new_with_text(name: &str, text: &str) -> Self {
		todo!()
	}
	/** Creates a new Element with the specified name, attributes, and text.
	# Example
	```rust
	fn main() {
		use kiss_xml::dom::*;
		use std::collections::HashMap;
		let e = Element::new_with_attributes_and_text(
			"b",
			HashMap::from(&[
				("style", "color: blue")
			]),
			"goose"
		);
		println!("{}", e) // prints `<b style="color: blue">goose</b>`
	}
	```
	 */
	pub fn new_with_attributes_and_text(name: &str, attributes: HashMap<impl Into<String>, impl Into<String>>, text: &str) -> Self {
		todo!()
	}
	/**
	Creates a new Element with the specified name, attributes, and children.
	# Example
	```rust
	fn main() {
		use kiss_xml::dom::*;
		use std::collections::HashMap;
		let e = Element::new_with_attributes_and_children(
			"contact",
			HashMap::from(&[
				("id", "123")
			]),
			&[&Element::new_with_text("name", "Billy Bob")]
		);
		println!("{}", e)
		/* prints:
			<contact id="123">
				<name>Billy Bob</name>
			</contact>
		*/
	}
	```
	 */
	pub fn new_with_attributes_and_children(name: &str, attributes: HashMap<impl Into<String>, impl Into<String>>, children: &[&dyn Node]) -> Self {todo!()}

	/**
	Creates a new Element with the specified name and children.
	# Example
	```rust
	fn main() {
		use kiss_xml::dom::*;
		use std::collections::HashMap;
		let e = Element::new_with_children(
			"contact",
			&[&Element::new_with_text("name", "Billy Bob")]
		);
		println!("{}", e)
		/* prints:
			<contact>
				<name>Billy Bob</name>
			</contact>
		*/
	}
	```
	 */
	pub fn new_with_children(name: &str, children: &[&dyn Node]) -> Self {todo!()}
	/** Returns the tag name of this element (eg "book" for element `<book />`) */
	pub fn name(&self) -> String {
		todo!()
	}
	/**
	Returns the namespace of this element, or `None` if it does not have a namespace. If this element has a namespace but `namespace_prefix()` returns `None`, then the namespace is a default namespace (no prefix, can be inherited by children).
	 */
	pub fn namespace(&self) -> Option<Uri> {
		todo!()
	}
	/**
	Returns the default namespace of this element, or `None` if it does not have a default namespace. Default namespaces do not use prefixes and are inherited by the element's children.
	 */
	pub fn default_namespace(&self) -> Option<Uri> {
		todo!()
	}
	/**
	Returns the prefix of this element's namespace, if it has a prefixed namespace. If this element has a namespace but `namespace_prefix()` returns `None`, then the namespace is a default namespace (no prefix, can be inherited by children).
	 */
	pub fn namespace_prefix(&self) -> Option<String> {
		todo!()
	}

	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace. This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no XML namespace associated with them, pass `None` as the argument to this function.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use http::Uri;
		use std::str::FromStr;
		use kiss_xml;
		use kiss_xml::dom::*;
		let doc = kiss_xml::parse_str(r#"<?xml version="1.0" encoding="UTF-8"?>
	<root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
		<width>200</width>
		<height>150</height>
		<depth>50</depth>
		<img:width>200</img:width>
		<img:height>150</img:height>
		<dim:width>200</dim:width>
	</root>"#)?;
		for e in doc.root_element().elements_by_namespace(Some(&Uri::from_str("internal://ns/a")?)){
			println!("img element <{}> contains '{}'", e.name(), e.text())
		}
		/* Prints:
		img element <width> contains '200'
		img element <height> contains '150'
		*/
		Ok(())
	}
	```
	 */
	pub fn elements_by_namespace(&self, namespace: Option<&Uri>) -> Iter<Element>{
		todo!()
	}
	/** Returns a list (as an iterator) of all child elements that belong to the given XML namespace. This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_mut(...)` instead.

	To get a list of elements that have no XML namespace associated with them, pass `None` as the argument to this function.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use http::Uri;
		use std::str::FromStr;
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut doc = kiss_xml::parse_str(r#"<?xml version="1.0" encoding="UTF-8"?>
	<root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
		<width>200</width>
		<height>150</height>
		<depth>50</depth>
		<img:width>200</img:width>
		<img:height>150</img:height>
		<dim:width>200</dim:width>
	</root>"#)?;
		for e in doc.root_element().elements_by_namespace_mut(Some(&Uri::from_str("internal://ns/a")?)){
			e.set_text("0");
		}
		for e in doc.root_element().elements_by_namespace(Some(&Uri::from_str("internal://ns/a")?)){
			println!("img element <{}> contains '{}'", e.name(), e.text())
		}
		/* Prints:
		img element <width> contains '0'
		img element <height> contains '0'
		*/
		Ok(())
	}
	```
	 */
	pub fn elements_by_namespace_mut(&mut self, namespace: Option<&Uri>) -> IterMut<Element>{
		todo!()
	}
	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace according to the namespace's prefix (eg `<svg:g xmlns:svg="http://www.w3.org/2000/svg">`). This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no xmlns prefix associated with them, pass `None` as the argument to this function (this will still return elements with a default namespace as well as elements with no namespace).
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use http::Uri;
		use std::str::FromStr;
		use kiss_xml;
		use kiss_xml::dom::*;
		let doc = kiss_xml::parse_str(r#"<?xml version="1.0" encoding="UTF-8"?>
	<root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
		<width>200</width>
		<height>150</height>
		<depth>50</depth>
		<img:width>200</img:width>
		<img:height>150</img:height>
		<dim:width>200</dim:width>
	</root>"#)?;
		for e in doc.root_element().elements_by_namespace_prefix(Some("img")){
			println!("img element <{}> contains '{}'", e.name(), e.text())
		}
		/* Prints:
		img element <width> contains '200'
		img element <height> contains '150'
		*/
		Ok(())
	}
	 */
	pub fn elements_by_namespace_prefix(&self, prefix: Option<&str>) -> Iter<Element>{
		todo!()
	}
	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace according to the namespace's prefix (eg `<svg:g xmlns:svg="http://www.w3.org/2000/svg">`). This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no xmlns prefix associated with them, pass `None` as the argument to this function (this will still return elements with a default namespace as well as elements with no namespace).
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use http::Uri;
		use std::str::FromStr;
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut doc = kiss_xml::parse_str(r#"<?xml version="1.0" encoding="UTF-8"?>
	<root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
		<width>200</width>
		<height>150</height>
		<depth>50</depth>
		<img:width>200</img:width>
		<img:height>150</img:height>
		<dim:width>200</dim:width>
	</root>"#)?;
		for e in doc.root_element().elements_by_namespace_prefix_mut(Some("img")){
			e.set_text("-1")
		}
		for e in doc.root_element().elements_by_namespace_prefix(Some("img")){
			println!("img element <{}> contains '{}'", e.name(), e.text())
		}
		/* Prints:
		img element <width> contains '-1'
		img element <height> contains '-1'
		*/
		Ok(())
	}
	 */
	pub fn elements_by_namespace_prefix_mut(&mut self, prefix: Option<&str>) -> IterMut<Element>{
		todo!()
	}
	/** Gets any and all xmlns prefixes defined in this element */
	fn namespace_prefixes(&self) -> &Option<HashMap<String, Uri>> {todo!()}
	/** Gets any and all xmlns prefixes relevant to this element. This includes both those that are defined by this element as well as those defined by parent elements up the DOM tree*/
	fn set_namespace_context(&mut self, parent_namespace: Option<String>, parent_prefixes: Option<HashMap<String, Uri>>) { todo!()}
	/** Returns a list of al child elements as an iterator */
	pub fn child_elements(&self) -> Iter<Element>{
		todo!()
	}
	/** Returns a list of al child elements as an iterator */
	pub fn child_elements_mut(&mut self) -> IterMut<Element>{
		todo!()
	}
	/** Returns a list of al child nodes (elements, comments, and text components) as an iterator */
	pub fn children(&self) -> Iter<Box<dyn Node>>{
		todo!()
	}
	/** Returns a list of al child nodes (elements, comments, and text components) as an iterator */
	pub fn children_mut(&mut self) -> IterMut<Box<dyn Node>>{
		todo!()
	}

	/** Deletes all child nodes from this element */
	pub fn clear_children(&mut self) {todo!()}
	/** Replaces this element's content (children) with the given text. **This will delete any child elements and comments from this element!** */
	pub fn set_text(&mut self, text: impl Into<String>) {todo!()}
	/**
	Gets the first child element with the given element name. If no such element exists, `None` is returned.

	This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::errors::DoesNotExistError;
		let doc = kiss_xml::parse_str(r#"<body>
		<p>Hello there!</p>
		<p>Good-bye!</p>
	</body>"#)?;
		println!("1st <p>: {}",
			doc.root_element()
			.first_element_by_name("p").ok_or(Err(DoesNotExistError.default()))?
		);
		// prints: "1st <p>: Hello there!"
		Ok(())
	}
	```
	 */
	pub fn first_element_by_name(&self, name: impl Into<String>) -> Option<&Element> {
		todo!()
	}
	/**
	Gets the first child element with the given element name as a mutable reference. If no such element exists, `None` is returned.

	This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::errors::DoesNotExistError;
		let mut doc = kiss_xml::parse_str(r#"<body>
		<p>Hello there!</p>
	</body>"#)?;
		doc.root_element_mut()
			.first_element_by_name_mut("p").ok_or(Err(DoesNotExistError.default()))?
			.set_text("Good bye!");
		Ok(())
	}
	```
	 */
	pub fn first_element_by_name_mut(&mut self, name: impl Into<String>) -> Option<&mut Element> {
		todo!()
	}
	/** Returns a list of all child elements with the given name as an iterator.

		This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_by_name(...)` instead.
	 */
	pub fn elements_by_name(&self, name: impl Into<String>) -> Iter<Element>{
		todo!()
	}
	/** Returns a list of all child elements with the given name as an iterator.

		This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_by_name(...)` instead.
	 */
	pub fn elements_by_name_mut(&mut self, name: impl Into<String>) -> IterMut<Element>{
		todo!()
	}
	/** Gets the attributes for this element as a `HashMap` */
	pub fn attributes(&self) -> &HashMap<String, String> {
		todo!()
	}
	/** Gets the attributes for this element as a `HashMap` */
	pub fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
		todo!()
	}
	/** Gets the value of an attribute for this Element by name. If there is no such attribute, `None` is returned */
	pub fn get_attr(&self, attr_name: impl Into<String>) -> Option<String> {
		todo!()
	}
	/** Sets the value of an attribute for this Element by name. */
	pub fn set_attr(&mut self, attr_name: impl Into<String>, value: impl Into<String>) {
		todo!()
	}
	/** Deletes an attribute from this element */
	pub fn remove_attr(&mut self, attr_name: impl Into<String>) -> Option<String> {
		todo!()
	}
	/** Deletes all attributes from this element */
	pub fn clear_attributes(&mut self) {todo!()}
	/**
	Performs a recursive search of all child nodes of this element (and all children of child elements, etc), returning an iterator of all nodes matching the given predicate.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		println!("Fantasy books:");
		for fantasy_book in library.root_element.search(
			|n| n.is_element() && n.as_element()?.get_attr("genre") == Some("fantasy")
		){
			println!("{}", fantasy_book.text());
		}
		Ok(())
	}
	```
	 */
	pub fn search<P>(&self, predicate: P) -> Iter<Box<dyn Node>> where P: FnMut(&dyn Node) -> bool {
		// recursive
		todo!()
	}
	/**
	Performs a recursive search of all child nodes of this element (and all children of child elements, etc), returning an iterator of all nodes matching the given predicate.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		// set count to "0" for all of the fantasy books
		for fantasy_book in library.root_element.search_mut(
			|n| n.is_element() && n.as_element()?.get_attr("genre") == Some("fantasy")
		){
			fantasy_book.as_element()?.set_attr("count", "0")
		}
		Ok(())
	}
	```
	 */
	pub fn search_mut<P>(&mut self, predicate: P) -> IterMut<Box<dyn Node>> where P: FnMut(&mut dyn Node) -> bool {
		// recursive
		todo!()
	}
	/**
	Performs a recursive search of all child elements (and all children of child elements, etc), returning an iterator of all elements matching the given predicate.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		println!("Fantasy books:");
		for fantasy_book in library.root_element.search_elements(
			|e| e.get_attr("genre") == Some("fantasy")
		){
			println!("{}", fantasy_book.text());
		}
		Ok(())
	}
	```
	 */
	pub fn search_elements<P>(&self, predicate: P) -> Iter<Element> where P: FnMut(&Element) -> bool {
		// recursive
		todo!()
	}
	/**
	Performs a recursive search of all child elements (and all children of child elements, etc), returning an iterator of all elements matching the given predicate.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		// set count to "0" for all of the fantasy books
		for fantasy_book in library.root_element.search_elements_mut(
			|e| e.get_attr("genre") == Some("fantasy")
		){
			fantasy_book.set_attr("count", "0")
		}
		Ok(())
	}
	```
	 */
	pub fn search_elements_mut<P>(&mut self, predicate: P) -> IterMut<Element> where P: FnMut(&Element) -> bool {
		// recursive
		todo!()
	}
	/**
	Performs a recursive search of all child elements (and all children of child elements, etc), returning an iterator of all elements with the given name (regardless of namespace).

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		println!("All books:");
		for book in library.root_element.search_elements_by_name(
			"book"
		){
			println!("{}", book.text());
		}
		Ok(())
	}
	```
 	*/
	pub fn search_elements_by_name(&self, name: impl Into<String>) -> Iter<Element>{
		// recursive
		todo!()
	}
	/**
	Performs a recursive search of all child elements (and all children of child elements, etc), returning an iterator of all elements with the given name (regardless of namespace).

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut library = kiss_xml::parse_str(r#"<root>
			<books>
				<asian>
					<book genre="fantasy" count="1">Journey to the West</book>
				</asian>
				<european>
					<book genre="fantasy" count="1">The Lord of the Rings</book>
					<book genre="sci-fi" count="1">The Hitchhiker's Guide to the Galaxy</book>
				</european>
			</books>
		</root>"#)?;
		// set count to "0" for all of the fantasy books
		for book in library.root_element.search_elements_by_name_mut(
			"book"
		){
			if book.get_attr("genre") == Some("fantasy") {
				book.set_attr("count", "0");
			}
		}
		Ok(())
	}
	```
	 */
	pub fn search_elements_by_name_mut(&mut self, name: impl Into<String>) -> IterMut<Element>{
		// recursive
		todo!()
	}
	/** Performs a recursive search of all the text nodes under this element and returns all text nodes that match the given predicate as an iterator */
	pub fn search_text<P>(&self, predicate: P) -> Iter<Text> where P: FnMut(&Text) -> bool {
		// recursive
		todo!()
	}

	/** Performs a recursive search of all the text nodes under this element and returns all text nodes that match the given predicate as an iterator */
	pub fn search_text_mut<P>(&mut self, predicate: P) -> IterMut<Text> where P: FnMut(&Text) -> bool {
		// recursive
		todo!()
	}
	/** Performs a recursive search of all the comments under this element and returns all comment nodes that match the given predicate as an iterator */
	pub fn search_comments<P>(&self, predicate: P) -> Iter<Comment> where P: FnMut(&Comment) -> bool {
		// recursive
		todo!()
	}
	/** Performs a recursive search of all the comments under this element and returns all comment nodes that match the given predicate as an iterator */
	pub fn search_comments_mut<P>(&mut self, predicate: P) -> IterMut<Comment> where P: FnMut(&Comment) -> bool {
		// recursive
		todo!()
	}
	/**
	Appends the given node to the children of this element.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut doc = Document::new(Element::new_from_name("album"));
		doc.root_element_mut().append(Element::new_with_text("song", "I Believe I Can Fly"));
		println!("{}", doc);
		/* prints:
			<?xml version="1.0" encoding="UTF-8"?>
			<album>
				<song>I Believe I Can Fly</song>
			</album>
		*/
		Ok(())
	}
	```
	 */
	pub fn append(&mut self, node: impl Node) {
		todo!()
		// TODO: if this is an element, set the namespace context
	}
	/**
	Appends multiple child nodes to the current element.

	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
		use kiss_xml;
		use kiss_xml::dom::*;
		let mut doc = Document::new(Element::new_from_name("album"));
		doc.root_element_mut().append_all(&[
			&Element::new_with_text("song", "I Believe I Can Fly"),
			&Element::new_with_text("song", "My Heart Will Go On"),
			&Comment::new("album list incomplete"),
		]);
		println!("{}", doc);
		/* prints:
			<?xml version="1.0" encoding="UTF-8"?>
			<album>
				<song>I Believe I Can Fly</song>
				<song>My Heart Will Go On</song>
				<!--album list incomplete-->
			</album>
		*/
		Ok(())
	}
	```
	 */
	pub fn append_all(&mut self, children: &[&dyn Node]) {
		todo!()
		// TODO: if child is an element, set the namespace context
	}
	/**
	Inserts the given node at the given index in this element's list of child nodes (see the `children()` method). If the index is invalid, an error result is returned.
	 */
	pub fn insert(&mut self, index: usize, node: impl Node) -> Result<(), IndexOutOfBounds> {
		todo!()
		// TODO: if this is an element, set the namespace context
	}
	/**
	Removes the given node at the given index in this element's list of child nodes (see the `children()` method). If the index is invalid, an Err result is returned, otherwise the removed node is return as an Ok result.
	 */
	pub fn remove(&mut self, index: usize) -> Result<Box<dyn Node>, IndexOutOfBounds> {
		todo!()
	}
	/** Recursively removes all child nodes matching the given predicate function, returning the number of removed nodes. */
	pub fn remove_all<P>(&mut self, predicate: P) -> usize where P: FnMut(&dyn Node) -> bool {
		// recursive, returns count
		todo!()
	}
	/** Recursively removes all child elements matching the given predicate function, returning the number of removed elements. */
	pub fn remove_all_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// recursive, returns count
		todo!()
	}
	/** Removes the Nth child element from this element, returning it as a result (or an `IndexOutOfBounds` error result if the index is out of range) */
	pub fn remove_element(&mut self, index: usize) -> Result<Element, IndexOutOfBounds> {
		todo!()
	}
	/** Removes all child elements matching the given predicate function, returning the number of removed elements.

	This removal is non-recursive, meaning that it can only remove children of this element, not children-of-children. For a recursive removal, use `remove_all_elements(...)` instead. */
	pub fn remove_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// returns count
		todo!()
	}
	/** Removes all child elements matching the given element name (regardless of namespace), returning the number of removed elements.

	This removal is non-recursive, meaning that it can only remove children of this element, not children-of-children. For a recursive removal, use `remove_all_elements(...)` instead. */
	pub fn remove_elements_by_name(&mut self, name: impl Into<String>) -> usize {
		// returns count
		todo!()
	}

}

impl Node for Element{

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

	fn as_string_with_indent(&self, indent: &str) -> String {
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

/// Represents a string of text in the XML DOM
#[derive(Clone)]
pub struct Text {
	/// The content of this Text node
	pub content: String
}

impl Text {
	/** Construct a new Text node from the provided string-like object */
	pub fn new(text: impl Into<String>) -> Self {
		todo!()
	}
}

impl From<&str> for Text {
	fn from(value: &str) -> Self {
		Text::new(value)
	}
}

impl From<String> for Text {
	fn from(value: String) -> Self {
		Text::new(value)
	}
}

impl Node for Text{

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

	fn as_string_with_indent(&self, indent: &str) -> String {
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

/// Represents an XML comment
#[derive(Clone)]
pub struct Comment{
	/// The text of the comment
	pub content: String
}

impl Comment {
	/// Constructs a new Comment node from the given string-like object
	pub fn new(comment: impl Into<String>) -> Self {
		todo!()
	}
}

impl Node for Comment{

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

	fn as_string_with_indent(&self, indent: &str) -> String {
		todo!()
	}
}

impl From<&str> for Comment {
	fn from(value: &str) -> Self {
		Comment::new(value)
	}
}

impl From<String> for Comment {
	fn from(value: String) -> Self {
		Comment::new(value)
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

/** An XML document declaration, ie `<?xml version="1.0" encoding="UTF-8"?>`

`kiss_xml` does not interpret XML document declarations and does not require XML documents to have one. The declaration will simply be copied verbatum. */
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Declaration {
	// TODO
}

impl Declaration {
	/// Creates a new Declaration from the given string (eg `<?xml version="1.0" encoding="UTF-8"?>`)
	pub fn from_str(decl: &str) -> Self {
		todo!()
	}
	/// Serializes this Declaration as an XML declaration element string (eg ``<?xml version="1.0" encoding="UTF-8"?>`)
	pub fn to_string(&self) -> String {todo!()}
	/// Creates a new standard Declaration (UTF-8 encoded XML version 1)
	pub fn new() -> Self {
		Self::default()
	}
}

impl Default for Declaration {
	fn default() -> Self {
		Declaration::from_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#)
	}
}

impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

/**
An XML document type declaration (DTD) defines custom behavior for XML documents, but `kiss_xml` does not support DTDs beyond copying them verbatum.
*/
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct DTD {
	// TODO
}

impl std::fmt::Display for DTD {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl DTD {
	/// Creates a new DTD from the given string (eg `<!DOCTYPE note []>)
	pub fn from_string(text: impl Into<String>) -> DTD {todo!()}
	/// Serializes this Declaration as an XML DTD string
	pub fn to_string(&self) -> String {todo!()}
}
