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

use std::cell::OnceCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;
use crate::errors::*;

/**
A Document represents a DOM plus additional (optional) metadata such as one or more Document Type Declarations (DTD). Use this struct to write a DOM to a string or file.
*/
pub struct Document {
	/// Optional XML declaration (ie `<?xml version="1.0" encoding="UTF-8"?>`)
	declaration: Option<Declaration>,
	/// Doctype defs, if any
	dtds: Vec<DTD>,
	/// Root element (multi-element XML docs not supported)
	root_element: Element
}

impl Document {
	/**
Constructs a new Document with the given root element and default declaration
	 */
	pub fn new(root: Element) -> Self {
		Document::new_with_decl_dtd(root, Some(Declaration::default()), None)
	}
	/**
Full constructor with required root element and optional XML declaration and optional list of one or more document type definition (DTD) items.
	 */
	pub fn new_with_decl_dtd(root: Element, declaration: Option<Declaration>, dtd: Option<&[DTD]>) -> Self {
		Self{
			declaration: declaration,
			dtds: match dtd{
				None => Vec::with_capacity(1),
				Some(dtds) => Vec::from(dtds)
			},
			root_element: root
		}
	}
	/**
	Returns a list of any and all DTDs for this Document as an iterator
	 */
	pub fn doctype_defs(&self) -> impl Iterator<Item = &DTD> {
		self.dtds.iter()
	}
	/**
	Returns a list of any and all DTDs for this Document as an iterator
	 */
	pub fn doctype_defs_mut(&mut self) -> impl Iterator<Item = &mut DTD> {
		self.dtds.iter_mut()
	}
	/**
Sets the DTDs for this document (a `None` argument will remove all DTDs)
	 */
	pub fn set_doctype_defs(&mut self, dtds: Option<&[DTD]>) {
		match dtds {
			None => self.dtds = Vec::with_capacity(1),
			Some(dlist) => self.dtds = Vec::from(dlist)
		}
	}
	/**
Gets the XML declaration for this document, if it has one (while the XML spec requires a declaration at the start of every XML file, it is commonly omitted, especially when the XML is embedded in a stream or file).
	 */
	pub fn declaration(&self) -> &Option<Declaration> {
		&self.declaration
	}
	/**
Sets the XML declaration for this document (a `None` argument will remove any existing declaration). While the XML spec requires a declaration at the start of every XML file, it is commonly omitted, especially when the XML is embedded in a stream or file.
	 */
	pub fn set_declaration(&mut self, decl: Declaration) {
		self.declaration = Some(decl)
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
		let mut builder = String::new();
		match &self.declaration{
			None => {},
			Some(decl) => {
				builder.push_str(decl.to_string().as_str());
				builder.push_str("\n");
			}
		}
		for dtd in &self.dtds {
			builder.push_str(dtd.to_string().as_str());
			builder.push_str("\n");
		}
		builder.push_str(&self.root_element.to_string_with_indent(indent.into().as_str()));
		builder.push_str("\n");
		return builder;
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
		use std::fs;
		fs::write(path, self.to_string_with_indent(indent))
	}

	/**
	Writes this document as XML to the given file or stream using the default indent of two spaces per level, returning a result indicating success or error in this write operation
	 */
	pub fn write_to_file(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
		self.write_to_file_with_indent(out, "  ")
	}

	/**
	Writes this document as XML to the given file or stream using the default indent of two spaces per level, returning a result indicating success or error in this write operation
	 */
	pub fn write_to_file_with_indent(&self, out: &mut impl std::io::Write, indent: impl Into<String>) -> std::io::Result<()> {
		write!(out, "{}", self.to_string_with_indent(indent))
	}

	/**
	Returns the root element of this DOM as an immutable reference
	 */
	pub fn root_element(&self) -> &Element {
		&self.root_element
	}

	/**
	Returns the root element of this DOM as a mutable reference.
	  */
	pub fn root_element_mut(&mut self) -> &mut Element {
		&mut self.root_element
	}
}

impl std::fmt::Display for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string_with_indent("  "))
	}
}

impl std::fmt::Debug for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string_with_indent("  "))
	}
}

impl PartialEq<Self> for Document {
	fn eq(&self, other: &Self) -> bool {
		self.declaration == other.declaration
		&& self.dtds == other.dtds
		&& self.root_element == other.root_element
	}
}

/**
A node in the DOM tree. Elements, Comments, and Text are all types of nodes, but only Elements can be branch nodes with children of their own.
 */
pub trait Node: dyn_clone::DynClone + std::fmt::Debug + std::fmt::Display + ToString {

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
	fn as_element(&self) -> Result<&Element, TypeCastError>;

	/**
	Casts this Node to a Comment struct (if the Node is not an Comment struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_comment(&self) -> Result<&Comment, TypeCastError>;

	/**
	Casts this Node to a Text struct (if the Node is not a Text struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_text(&self) -> Result<&Text, TypeCastError>;

	/**
	Casts this Node to an Element struct (if the Node is not an Element struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_element_mut(&mut self) -> Result<&mut Element, TypeCastError>;

	/**
	Casts this Node to a Comment struct (if the Node is not an Comment struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_comment_mut(&mut self) -> Result<&mut Comment, TypeCastError>;

	/**
	Casts this Node to a Text struct (if the Node is not a Text struct, then `Err(TypeCastError)` error result is returned).
	 */
	fn as_text_mut(&mut self) -> Result<&mut Text, TypeCastError>;

	/**
	Casts this struct to a Node trait object
	 */
	fn as_node(&self) -> &dyn Node;

	/**
	Casts this struct to a Node trait object
	 */
	fn as_node_mut(&mut self) -> &mut dyn Node;

	/**
	Writes this Node to a string with the provided indent (used to serialize to XML)
	 */
	fn to_string_with_indent(&self, indent: &str) -> String;

	/** Converts this node into a `Box<dyn Node>` for convenient use in collections */
	fn boxed(self) -> Box<dyn Node>;
}

/// clones a given boxed node
pub fn clone_node(node: &Box<dyn Node>) -> Box<dyn Node> {
	if node.is_element() {
		Box::new(node.as_element().expect("logic error").clone())
	} else if node.is_text() {
		Box::new(node.as_text().expect("logic error").clone())
	} else if node.is_comment() {
		Box::new(node.as_comment().expect("logic error").clone())
	} else {
		panic!("logic error: Node is neither of Element, Text, or Comment");
	}
}

/// Returns true if the two nodes are equal, false otherwise
pub fn node_eq(n1: &Box<dyn Node>, n2: &Box<dyn Node>) -> bool {
	if n1.is_element() && n2.is_element() {
		return n1.as_element().expect("logic error") == n2.as_element().expect("logic error");
	} if n1.is_text() && n2.is_text() {
		return n1.as_text().expect("logic error") == n2.as_text().expect("logic error");
	} else if n1.is_comment() && n2.is_comment() {
		return n1.as_comment().expect("logic error") == n2.as_comment().expect("logic error");
	}
	return false;
}

/// Represents an XML element with a name, text content, attributes, xmlns namespace (with optional prefix), and children.
pub struct Element {
	/// Name of this element
	name: String,
	/// All child nodes
	child_nodes: Vec<Box<dyn Node>>,
	/// This element's attributes
	attributes: HashMap<String, String>,
	/// optional xmlns (if xmlns_prefix is None then this is default namespace)
	xmlns: Option<String>,
	/// optional xmlns (if xmlns_prefix is None then the xmlns is default namespace)
	xmlns_prefix: Option<String>,
	/// xmlns definitions for this element, if any
	xmlns_context: Option<HashMap<String, String>>
}

impl Element {
	/**
	Creates a new Element
	# Args:
	* *name*: Element name for this XML element (ie "body" for `<body>some text</body>`)
	* *text*: Optional text content for this element (ie "some text" for `<body>some text</body>`)
	* *attributes*: optional HashMap of attributes
	* *xmlns*: optional namespace for this element. Note that this will override any xmlns definitions in the attributes
	* *xmlns_prefix*: optional namespace prefix. If `xmlns` is not `None` but `xmlns_prefix` is `None`, then this element will set it's xmlns as the default xlmns for it and its children. Note that this will override any xmlns definitions in the attributes
	* *children*: optional list of child nodes to add to this element
	 */
	pub fn new<TEXT1: Into<String>+Clone, TEXT2: Into<String>+Clone>(name: &str, text: Option<&str>, attributes: Option<HashMap<TEXT1, TEXT2>>, xmlns: Option<&str>, xmlns_prefix: Option<&str>, children: Option<Vec<Box<dyn Node>>>) -> Result<Self, KissXmlError> {
		// sanity check
		Element::check_elem_name(name)?;
		// first, convert attributes to <String,String> map
		let mut attrs: HashMap<String, String> = HashMap::new();
		match attributes {
			None => {}
			Some(attr_map) => {
				for (k, v) in attr_map.iter() {
					let n: String = k.clone().into();
					Element::check_attr_name(n.as_str())?;
					attrs.insert(n, v.clone().into());
				}
			}
		}
		// set the XML NS from the attributes and provided args
		let mut elem = Self {
			name: name.to_string(),
			child_nodes: Vec::new(),
			xmlns_context: Element::xmlns_context_from_attributes(&attrs, None, false),
			attributes: attrs,
			xmlns: xmlns.map(|s| s.to_string()),
			xmlns_prefix: xmlns_prefix.map(|s| s.to_string())
		};
		// finally, add children
		// (using the append*(...) functions in case of default namespace inheritance)
		match text {
			None => {}
			Some(t) => elem.append(Text::new(t))
		}
		match children {
			None => {},
			Some(child_vec) => elem.append_all(child_vec)
		};
		return Ok(elem);
	}
	/// Creates a new Element with the specified name and not attributes or content.
	pub fn new_from_name(name: &str) -> Result<Self, KissXmlError> {
		// sanity check
		Element::check_elem_name(name)?;
		Ok(Self {
			name: name.to_string(),
			..Default::default()
		})
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
	pub fn new_with_attributes<TEXT1: Into<String>+Clone, TEXT2: Into<String>+Clone>(name: &str, attributes: HashMap<TEXT1, TEXT2>) -> Result<Self, KissXmlError> {
		Self::new(name, None, Some(attributes), None, None, None)
	}
	/// Creates a new Element with the specified name and text content
	pub fn new_with_text(name: &str, text: &str) -> Result<Self, KissXmlError> {
		Self::new(name, Some(text), Option::<HashMap<String,String>>::None, None, None, None)
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
	pub fn new_with_attributes_and_text<TEXT1: Into<String>+Clone, TEXT2: Into<String>+Clone>(name: &str, attributes: HashMap<TEXT1, TEXT2>, text: &str) -> Result<Self, KissXmlError> {
		Self::new(name, Some(text), Some(attributes), None, None, None)
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
			vec![Element::new_with_text("name", "Billy Bob").boxed()]
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
	pub fn new_with_attributes_and_children<TEXT1: Into<String>+Clone, TEXT2: Into<String>+Clone>(name: &str, attributes: HashMap<TEXT1, TEXT2>, children: Vec<Box<dyn Node>>) -> Result<Self, KissXmlError> {
		Self::new(name, None, Some(attributes), None, None, Some(children))
	}

	/**
	Creates a new Element with the specified name and children.
	# Example
	```rust
	fn main() {
		use kiss_xml::dom::*;
		use std::collections::HashMap;
		let e = Element::new_with_children(
			"contact",
			vec![Element::new_with_text("name", "Billy Bob").boxed()]
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
	pub fn new_with_children(name: &str, children: Vec<Box<dyn Node>>) -> Result<Self, KissXmlError> {
		Self::new(name, None, Option::<HashMap<String,String>>::None, None, None, Some(children))
	}
	/** checks the element's attributes for xmlns definitions
	Note that the default xmlns (if present) is saved as prefix ""
	# Args
	* attrs - kay=value pairs for the element
	* parent_default_xmlns - inherited default namespace (or `None`)
	* include_default - if `true`, the default namespace will be stored in the output hashmap under the key "", if `false` then the default namespace is not included in the output
	 */
	fn xmlns_context_from_attributes(attrs: &HashMap<String, String>, parent_default_xmlns: Option<String>, include_default: bool) -> Option<HashMap<String, String>> {
		// first check for default xlmns
		let mut default_xmlns = parent_default_xmlns;
		if attrs.contains_key("xmlns") {
			default_xmlns = Some(
				attrs.get("xmlns").expect("logic error").to_string()
			);
		}
		// then parse xmlns prefixes
		let mut prefixes: HashMap<String, String> = HashMap::new();
		for (k, v) in attrs.iter() {
			let key = k.as_str();
			if key.starts_with("xmlns:") {
				let split: Vec<&str> = key.splitn(2, ":").collect();
				let prefix = split[1].to_string();
				let ns = v.clone();
				prefixes.insert(prefix, ns);
			}
		}
		// if there are no xmlns defs, return None
		if default_xmlns.is_none() && prefixes.len() == 0 {
			return None;
		} else {
			// add default xmlns as "" in the prefix map
			match default_xmlns {
				None => {}
				Some(def_ns) => {
					if include_default {
						prefixes.insert("".to_string(), def_ns);
					}
				}
			}
			// return prefix map
			return Some(prefixes);
		}
	}
	/** Returns the tag name of this element (eg "book" for element `<book />`) */
	pub fn name(&self) -> String {
		self.name.clone()
	}
	/**
	Returns the namespace of this element, or `None` if it does not have a namespace. If this element has a namespace but `namespace_prefix()` returns `None`, then the namespace is a default namespace (no prefix, can be inherited by children).
	 */
	pub fn namespace(&self) -> Option<String> {
		self.xmlns.clone()
	}
	/**
	Returns the default namespace of this element, or `None` if it does not have a default namespace. Default namespaces do not use prefixes and are inherited by the element's children.
	 */
	pub fn default_namespace(&self) -> Option<String> {
		match self.xmlns_prefix{
			None => self.xmlns.clone(),
			Some(_) => None
		}
	}
	/**
	Returns the prefix of this element's namespace, if it has a prefixed namespace. If this element has a namespace but `namespace_prefix()` returns `None`, then the namespace is a default namespace (no prefix, can be inherited by children).
	 */
	pub fn namespace_prefix(&self) -> Option<String> {
		self.xmlns_prefix.clone()
	}

	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace. This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no XML namespace associated with them, pass `None` as the argument to this function.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
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
		for e in doc.root_element().elements_by_namespace(Some(&String::from_str("internal://ns/a")?)){
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
	pub fn elements_by_namespace(&self, namespace: Option<&str>) -> impl Iterator<Item = &Element>{
		let ns = namespace.map(|s| s.to_string());
		self.child_elements().filter(move |c| c.xmlns == ns)
	}
	/** Returns a list (as an iterator) of all child elements that belong to the given XML namespace. This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_mut(...)` instead.

	To get a list of elements that have no XML namespace associated with them, pass `None` as the argument to this function.
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
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
		for e in doc.root_element().elements_by_namespace_mut(Some("internal://ns/a")){
			e.set_text("0");
		}
		for e in doc.root_element().elements_by_namespace(Some("internal://ns/a")){
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
	pub fn elements_by_namespace_mut(&mut self, namespace: Option<&str>) -> impl Iterator<Item = &mut Element>{
		let ns = namespace.map(|s| s.to_string());
		self.child_elements_mut().filter(move |c| c.xmlns == ns)
	}
	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace according to the namespace's prefix (eg `<svg:g xmlns:svg="http://www.w3.org/2000/svg">`). This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no xmlns prefix associated with them, pass `None` as the argument to this function (this will still return elements with a default namespace as well as elements with no namespace).
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
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
	pub fn elements_by_namespace_prefix(&self, prefix: Option<&str>) ->  impl Iterator<Item = &Element>{
		let pfx = prefix.map(|p| p.to_string());
		self.child_elements().filter(move |c| c.xmlns_prefix == pfx)
	}
	/**
	Returns a list (as an iterator) of all child elements that belong to the given XML namespace according to the namespace's prefix (eg `<svg:g xmlns:svg="http://www.w3.org/2000/svg">`). This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements(...)` instead.

	To get a list of elements that have no xmlns prefix associated with them, pass `None` as the argument to this function (this will still return elements with a default namespace as well as elements with no namespace).
	# Example
	```rust
	fn main() -> Result<(), Box<dyn std::error::Error>> {
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
	pub fn elements_by_namespace_prefix_mut(&mut self, prefix: Option<&str>) ->  impl Iterator<Item = &mut Element>{
		let pfx = prefix.map(|p| p.to_string());
		self.child_elements_mut().filter(move |c| c.xmlns_prefix == pfx)
	}
	/** Gets any and all xmlns prefixes defined in this element (does not include prefix-less default namespace, nor prefixes inherited from a parent element) */
	fn namespace_prefixes(&self) -> Option<HashMap<String, String>> {
		Self::xmlns_context_from_attributes(&self.attributes, None, false)
	}
	/** Gets any and all xmlns prefixes relevant to this element. This includes both those that are defined by this element as well as those defined by parent elements up the DOM tree. Default namespace is stored under the "" key in the hash map. */
	fn get_namespace_context(&self) -> Option<HashMap<String, String>> {self.xmlns_context.clone()}
	/** Sets any and all xmlns prefixes this element should inherit. This must include both those that are defined by this element as well as those defined by parent elements up the DOM tree. */
	fn set_namespace_context(&mut self, parent_default_namespace: Option<String>, parent_prefixes: Option<HashMap<String, String>>) {
		// inherit default namespace unless this element also defines one
		match self.xmlns_prefix {
			None => {
				match self.default_namespace() {
					None => self.xmlns = parent_default_namespace,
					Some(_) => {/* do nothing */}
				}
			}
			Some(_) => {/* do nothing */}
		}
		// add prefixed namespaces (except where already defined locally)
		match parent_prefixes{
			None => {}
			Some(prefixes) => {
				if self.xmlns_context.is_none() {
					self.xmlns_context = Some(HashMap::new())
				}
				let context = &mut self.xmlns_context.clone().expect("logic error");
				for (prefix, ns) in prefixes {
					if ! context.contains_key(prefix.as_str()) {
						context.insert(prefix, ns);
					}
				}
			}
		}
	}
	/** Returns a list of al child elements as an iterator */
	pub fn child_elements(&self) ->  impl Iterator<Item = &Element>{
		self.child_nodes.iter()
			.filter(|n| n.is_element())
			.map(|n| n.as_element().expect("logic error"))
	}
	/** Returns a list of al child elements as an iterator */
	pub fn child_elements_mut(&mut self) ->  impl Iterator<Item = &mut Element>{
		self.child_nodes.iter_mut()
			.filter(|n| n.is_element())
			.map(|n| n.as_element_mut().expect("logic error"))
	}
	/** Returns a list of al child nodes (elements, comments, and text components) as an iterator */
	pub fn children(&self) -> impl Iterator<Item = &Box<dyn Node>>{
		self.child_nodes.iter()
	}
	/** Returns a list of al child nodes (elements, comments, and text components) as an iterator */
	pub fn children_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Node>>{
		self.child_nodes.iter_mut()
	}

	/** Deletes all child nodes from this element */
	pub fn clear_children(&mut self) {self.child_nodes.clear()}
	/** Replaces this element's content (children) with the given text. **This will delete any child elements and comments from this element!** */
	pub fn set_text(&mut self, text: impl Into<String>) {
		self.clear_children();
		self.append(Text::new(text));
	}
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
		let n: String = name.into();
		for e in self.child_elements() {
			if e.name() == n {
				return Some(e);
			}
		}
		None
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
		let n: String = name.into();
		for e in self.child_elements_mut() {
			if e.name() == n {
				return Some(e);
			}
		}
		None
	}
	/** Returns a list of all child elements with the given name as an iterator.

		This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_by_name(...)` instead.
	 */
	pub fn elements_by_name(&self, name: impl Into<String>) ->  impl Iterator<Item = &Element>{
		let n: String = name.into();
		self.child_elements().filter(move |c| c.name == n)
	}
	/** Returns a list of all child elements with the given name as an iterator.

		This search is non-recursive, meaning that it only returns children of this element, not children-of-children. For a recursive search, use `search_elements_by_name(...)` instead.
	 */
	pub fn elements_by_name_mut(&mut self, name: impl Into<String>) ->  impl Iterator<Item = &mut Element>{
		let n: String = name.into();
		self.child_elements_mut().filter(move |c| c.name == n)
	}
	/** Gets the attributes for this element as a `HashMap` */
	pub fn attributes(&self) -> &HashMap<String, String> {
		&self.attributes
	}
	/** Gets the value of an attribute for this Element by name. If there is no such attribute, `None` is returned */
	pub fn get_attr(&self, attr_name: impl Into<String>) -> Option<&String> {
		let n: String = attr_name.into();
		self.attributes.get(&n)
	}
	/** Sets the value of an attribute for this Element by name. */
	pub fn set_attr(&mut self, attr_name: impl Into<String>, value: impl Into<String>) -> Result<(), InvalidAttributeName> {
		let n: String = attr_name.into();
		Element::check_attr_name(n.as_str())?;
		let v: String = value.into();
		self.attributes.insert(n, v);
		Ok(())
	}


	/// singleton regex matcher
	const ATTR_NAME_CHECKER_SINGLETON: OnceCell<Regex> = OnceCell::new();
	/// Checks if an attribute name is valid
	fn check_attr_name(name: &str) -> Result<(), InvalidAttributeName> {
		let singleton = Element::ATTR_NAME_CHECKER_SINGLETON;
		let checker = singleton.get_or_init(
			|| Regex::new(r#"^[_a-zA-Z]\S*$"#).unwrap()
		);
		if checker.is_match(name) {
			Ok(())
		} else {
			Err(InvalidAttributeName::new(format!("'{}' is not a valid attribute name", name)))
		}
	}
	/// singleton regex matcher
	const NAME_CHECKER_SINGLETON: OnceCell<Regex> = OnceCell::new();
	/// Checks if an attribute name is valid
	fn check_elem_name(name: &str) -> Result<(), InvalidElementName> {
		let singleton = Element::NAME_CHECKER_SINGLETON;
		let checker = singleton.get_or_init(
			|| Regex::new(r#"^[_a-zA-Z]\S*$"#).unwrap()
		);
		if checker.is_match(name) {
			Ok(())
		} else {
			Err(InvalidElementName::new(format!("'{}' is not a valid name", name)))
		}
	}
	/** Deletes an attribute from this element */
	pub fn remove_attr(&mut self, attr_name: impl Into<String>) -> Option<String> {
		let n: String = attr_name.into();
		self.attributes.remove(&n)
	}
	/** Deletes all attributes from this element */
	pub fn clear_attributes(&mut self) {
		self.attributes.clear()
	}
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
	pub fn search<'a, P>(&'a self, predicate: P) -> Box<dyn Iterator<Item = &Box<dyn Node>> + '_> where P: FnMut(&&Box<dyn Node>) -> bool + 'a {
		// recursive
		Box::new(
			self.child_nodes.iter()
				.chain(
					self.child_elements().map(|e| e.child_nodes.iter()
					).flatten()
				).filter(predicate)
		)
	}
	/*
	NOTE: cannot do recursive mutable iterator (imagine if the iterator has both
	 parent and child elements in it and the user called `remove_all()` on the
	 parent element before reaching the child element)
	 */
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
	pub fn search_elements<'a, P>(&'a self, predicate: P) ->  Box<dyn Iterator<Item = &Element> + '_> where P: FnMut(&&Element) -> bool + 'a {
		// recursive
		Box::new(
			self.child_elements()
				.chain(
					self.child_elements().map(|e| e.child_elements()).flatten()
				).filter(predicate)
		)
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
	pub fn search_elements_by_name(&self, name: impl Into<String>) ->  impl Iterator<Item = &Element>{
		// recursive
		let n: String = name.into();
		self.search_elements(move |e| e.name() == n)
	}
	/** Performs a recursive search of all the text nodes under this element and returns all text nodes that match the given predicate as an iterator */
	pub fn search_text<'a, P>(&'a self, predicate: P) -> Box<dyn Iterator<Item = &Text> + '_> where P: Fn(&&Text) -> bool + 'a {
		// recursive
		Box::new(
			self.search_elements(|n| n.is_text())
			.map(|n| n.as_text().expect("logic error"))
			.filter(predicate)
		)
	}

	/** Performs a recursive search of all the comments under this element and returns all comment nodes that match the given predicate as an iterator */
	pub fn search_comments<'a, P>(&'a self, predicate: P) -> Box<dyn Iterator<Item = &Comment> + '_> where P: Fn(&&Comment) -> bool + 'a {
		// recursive
		Box::new(
			self.search_elements(|n| n.is_comment())
				.map(|n| n.as_comment().expect("logic error"))
				.filter(predicate)
		)
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
		// Note: if this is an element, set the namespace context
		let is_element = node.is_element();
		self.child_nodes.push(node.boxed());
		if is_element {
			let df_xmlns = self.default_namespace().clone();
			let xmlns_context = self.get_namespace_context().clone();
			// update xmlns prefix context if we just added an element
			self.child_nodes.last_mut().expect("logic error")
				.as_element_mut().expect("logic error")
				.set_namespace_context(
					df_xmlns,
					xmlns_context
				);
		}
		// clean-up text nodes
		self.cleanup_text_nodes();
	}
	/** Discards merges sequential text nodes and then whitespace-only text nodes */
	fn cleanup_text_nodes(&mut self) {
		// check if there are children
		if self.child_nodes.len() == 0 {return;}
		// merge sequential text nodes (back-to-front order for performance)
		let mut index = self.child_nodes.len() - 1;
		while index > 0 {
			if self.child_nodes[index].is_text()
			&& self.child_nodes[index-1].is_text() {
				// index-1 and index are text nodes, merge them
				let back = self.child_nodes.remove(index);
				let front = self.child_nodes.remove(index-1);
				let merged = Text::concat(front.as_text().expect("logic error"), back.as_text().expect("logic error"));
				self.child_nodes.insert(index-1, merged.boxed());
			}
			index -= 1;
		}
		// remove text nodes that are whitespace
		let mut index = self.child_nodes.len() - 1;
		while index >= 0 {
			if self.child_nodes[index].is_text()
				&& self.child_nodes[index]
				.as_text().expect("logic error").is_whitespace() {
				self.child_nodes.remove(index);
			}
			index -= 1;
		}
		// Done.
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
	pub fn append_all(&mut self, children: Vec<Box<dyn Node>>) {
		// first add every node, keeping a record of which ones were elements
		let mut elem_indices: Vec<usize> = Vec::with_capacity(children.len());
		let mut i = self.child_nodes.len();
		for child in children {
			if child.is_element() {
				elem_indices.push(i);
			}
			self.child_nodes.push(child);
			i += 1;
		}
		// then apply the xmlns context to the elements
		let df_xmlns = self.default_namespace().clone();
		let xmlns_context = self.get_namespace_context().clone();
		for i in elem_indices {
			self.child_nodes[i]
				.as_element_mut().expect("logic error (bad index)")
				.set_namespace_context(
					df_xmlns.clone(),
					xmlns_context.clone()
				)
		}
		// clean-up text nodes
		self.cleanup_text_nodes();
	}
	/**
	Inserts the given node at the given index in this element's list of child nodes (see the `children()` method). If the index is invalid, an error result is returned.
	 */
	pub fn insert(&mut self, index: usize, node: impl Node) -> Result<(), IndexOutOfBounds> {
		if index > self.child_nodes.len() {
			return Err(IndexOutOfBounds::new(index as isize, Some((0, self.child_nodes.len() as isize))));
		}
		// Note: if this is an element, set the namespace context
		let is_element = node.is_element();
		self.child_nodes.insert(index, node.boxed());
		if is_element {
			let df_xmlns = self.default_namespace().clone();
			let xmlns_context = self.get_namespace_context().clone();
			// update xmlns prefix context if we just added an element
			self.child_nodes.get_mut(index).expect("logic error")
				.as_element_mut().expect("logic error")
				.set_namespace_context(
					df_xmlns,
					xmlns_context
				);
		}
		// clean-up text nodes
		self.cleanup_text_nodes();
		// done
		Ok(())
	}
	/**
	Removes the given node at the given index in this element's list of child nodes (see the `children()` method). If the index is invalid, an Err result is returned, otherwise the removed node is return as an Ok result.
	 */
	pub fn remove(&mut self, index: usize) -> Result<Box<dyn Node>, IndexOutOfBounds> {
		if index > self.child_nodes.len() {
			return Err(IndexOutOfBounds::new(index as isize, Some((0, self.child_nodes.len() as isize))));
		}
		Ok(self.child_nodes.remove(index))
	}
	/** Removes all child nodes matching the given predicate function, returning the number of removed nodes (non-recursive). */
	pub fn remove_all<P>(&mut self, predicate: P) -> usize where P: Fn(&dyn Node) -> bool {
		let mut rm_indices: Vec<usize> = Vec::new();
		for i in (0..self.child_nodes.len()).rev() {
			if predicate(self.child_nodes[i].as_ref()) {
				rm_indices.push(i);
			}
		}
		let count =  rm_indices.len();
		for i in rm_indices {
			self.child_nodes.remove(i);
		}
		return count;
	}
	/** Removes the Nth child element from this element, returning it as a result (or an `IndexOutOfBounds` error result if the index is out of range) */
	pub fn remove_element(&mut self, index: usize) -> Result<Element, IndexOutOfBounds> {
		// first, index the child elements
		let mut elems: Vec<usize> = Vec::new();
		for i in 0..self.child_nodes.len() {
			if self.child_nodes[i].is_element(){ elems.push(i); }
		}
		// now remove the requested element
		if index > elems.len() {
			return Err(IndexOutOfBounds::new(index as isize, Some((0, elems.len() as isize))));
		}
		let removed = self.child_nodes.remove(elems[index]);
		Ok(removed.as_element().expect("logic error").clone())
	}
	/** Removes all child elements matching the given predicate function, returning the number of removed elements.

	This removal is non-recursive, meaning that it can only remove children of this element, not children-of-children. For a recursive removal, use `remove_all_elements(...)` instead. */
	pub fn remove_elements<P>(&mut self, predicate: P) -> usize where P: Fn(&Element) -> bool {
		let mut rm_indices: Vec<usize> = Vec::new();
		for i in (0..self.child_nodes.len()).rev() {
			if self.child_nodes[i].is_element() {
				if predicate(
					self.child_nodes[i].as_element().expect("logic error")
				) {
					rm_indices.push(i);
				}
			}
		}
		let count =  rm_indices.len();
		for i in rm_indices {
			self.child_nodes.remove(i);
		}
		return count;
	}
	/** Removes all child elements matching the given element name (regardless of namespace), returning the number of removed elements.

	This removal is non-recursive, meaning that it can only remove children of this element, not children-of-children. For a recursive removal, use `remove_all_elements(...)` instead. */
	pub fn remove_elements_by_name(&mut self, name: impl Into<String>) -> usize {
		let n: String = name.into();
		self.remove_elements(move |e| e.name == n)
	}

	/// Implementation of writing DOM to XML string
	fn to_string_with_prefix_and_indent(&self, prefix: &str, indent: &str) -> String {
		let mut out = String::from(prefix);
		// tag name
		let tag_name = match self.namespace_prefix(){
			None => self.name.clone(),
			Some(xmlns_prefix) => {
				format!("{}:{}", xmlns_prefix, self.name)
			}
		};
		out.push_str("<");
		out.push_str(tag_name.as_str());

		// attributes
		let attrs = self.attributes();
		for (k, v) in attrs {
			out.push_str(" ");
			out.push_str(k.as_str());
			out.push_str("=\"");
			out.push_str(crate::attribute_escape(v).as_str());
			out.push_str("\"");
		}
		// children (or not)
		let child_count = self.child_nodes.len();
		if child_count == 0 {
			out.push_str("/>");
		} else if child_count == 1 && !self.child_nodes[0].is_element() {
			// single non-element child, display inline
			out.push_str(">");
			out.push_str(&self.child_nodes[0].to_string_with_indent(indent));
			out.push_str("</");
			out.push_str(self.name().as_str());
			out.push_str(">");
		} else {
			// multiple children, prettify
			out.push_str(">\n");
			// prettify variables
			let mut next_prefix = String::from(prefix);
			next_prefix.push_str(indent);
			let mut line_end = String::from("\n");
			line_end.push_str(next_prefix.as_str());
			for c in &self.child_nodes {
				if c.is_text() {
					// indent start of each line of text
					let text = crate::text_escape(
						c.text().or(Some(String::new()))
							.expect("logic error")
					);
					out.push_str(next_prefix.as_str());
					out.push_str(text.replace("\n", line_end.as_str()).as_str());
				} else if c.is_comment() {
					// indent start of comment
					out.push_str(next_prefix.as_str());
					out.push_str(c.to_string_with_indent(indent).as_str());
				} else {
					// child element, recurse
					out.push_str(c.as_element().expect("logic error").to_string_with_prefix_and_indent(next_prefix.as_str(), indent).as_str());
				}
			}
			// closing tag
			out.push_str("\n");
			out.push_str(prefix);
			out.push_str("</");
			out.push_str(tag_name.as_str());
			out.push_str(">");
		}
		return out;
	}

}

impl Node for Element {

	fn text(&self) -> Option<String> {
		// Note: this is recursive
		let mut builder = String::new();
		for c in &self.child_nodes {
			if c.is_text() || c.is_element() {
				match c.text(){
					None => {}
					Some(txt) => builder.push_str(txt.as_str())
				};
			}
		}
		if builder.is_empty(){
			None
		} else {
			Some(builder)
		}
	}

	fn is_element(&self) -> bool {
		true
	}

	fn is_text(&self) -> bool {
		false
	}

	fn is_comment(&self) -> bool {
		false
	}

	fn as_element(&self) -> Result<&Element, TypeCastError> {Ok(&self)}

	fn as_comment(&self) -> Result<&Comment, TypeCastError> {Err(TypeCastError::new("Cannot cast Element as Comment"))}

	fn as_text(&self) -> Result<&Text, TypeCastError> {Err(TypeCastError::new("Cannot cast Element as Text"))}

	fn as_element_mut(&mut self) -> Result<&mut Element, TypeCastError> {Ok(self)}

	fn as_comment_mut(&mut self) -> Result<&mut Comment, TypeCastError> {Err(TypeCastError::new("Cannot cast Element as Comment"))}

	fn as_text_mut(&mut self) -> Result<&mut Text, TypeCastError> {Err(TypeCastError::new("Cannot cast Element as Text"))}

	fn as_node(&self) -> &dyn Node {self}

	fn as_node_mut(&mut self) -> &mut dyn Node {self}

	fn to_string_with_indent(&self, indent: &str) -> String {
		self.to_string_with_prefix_and_indent("", indent)
	}

	fn boxed(self) -> Box<dyn Node> {
		Box::new(self)
	}
}

impl Clone for Element {
	fn clone(&self) -> Self {
		let mut new_children: Vec<Box<dyn Node>> = Vec::with_capacity(self.child_nodes.len());
		for c in &self.child_nodes {
			new_children.push(clone_node(c))
		}
		Self {
			name: self.name.clone(),
			child_nodes: new_children,
			attributes: self.attributes.clone(),
			xmlns: self.xmlns.clone(),
			xmlns_prefix: self.xmlns_prefix.clone(),
			xmlns_context: self.xmlns_context.clone(),
		}
	}
}

impl Default for Element {
	fn default() -> Self {
		Self {
			name: "x".to_string(),
			child_nodes: vec![],
			attributes: Default::default(),
			xmlns: None,
			xmlns_prefix: None,
			xmlns_context: None,
		}
	}
}

impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.name.partial_cmp(&other.name)
	}
}

impl PartialEq<Self> for Element {
	fn eq(&self, other: &Self) -> bool {
		if self.name == other.name && self.xmlns == other.xmlns
			&& self.xmlns_prefix == other.xmlns_prefix
			&& self.attributes == other.attributes
			&& self.child_nodes.len() == other.child_nodes.len() {
			for i in 0..self.child_nodes.len() {
				if !node_eq(&self.child_nodes[i], &other.child_nodes[i]) {
					return false;
				}
			}
			return true;
		}
		return false;
	}
}

impl Hash for Element {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
		self.xmlns.hash(state);
	}
}


impl std::fmt::Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl std::fmt::Debug for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

/// Represents a string of text in the XML DOM
#[derive(Clone)]
pub struct Text {
	/// The content of this Text node
	pub content: String
}

/// singleton regex matcher
const WSP_MATCHER_SINGLETON: OnceCell<Regex> = OnceCell::new();

impl Text {
	/** Construct a new Text node from the provided string-like object */
	pub fn new(text: impl Into<String>) -> Self {
		let content: String = text.into();
		Self{content}
	}

	/** Returns a new Text node that is equivalent to this one plus the given Text node */
	pub fn concat(&self, other: &Text) -> Text {
		let mut content = String::new();
		content.push_str(self.content.as_str());
		content.push_str(other.content.as_str());
		Text{content}
	}

	fn is_whitespace(&self) -> bool {
		let singleton = WSP_MATCHER_SINGLETON;
		let wsp_matcher = singleton.get_or_init(|| Regex::new(r#"^\s+$"#).unwrap());
		wsp_matcher.is_match(self.content.as_str())
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

impl Node for Text {

	fn text(&self) -> Option<String> {
		Some(self.content.clone())
	}

	fn is_element(&self) -> bool {
		false
	}

	fn is_text(&self) -> bool {
		true
	}

	fn is_comment(&self) -> bool {
		false
	}

	fn as_element(&self) -> Result<&Element, TypeCastError> {Err(TypeCastError::new("Cannot cast Text as Element"))}

	fn as_comment(&self) -> Result<&Comment, TypeCastError> {Err(TypeCastError::new("Cannot cast Text as Comment"))}

	fn as_text(&self) -> Result<&Text, TypeCastError> {Ok(&self)}

	fn as_element_mut(&mut self) -> Result<&mut Element, TypeCastError> {Err(TypeCastError::new("Cannot cast Text as Element"))}

	fn as_comment_mut(&mut self) -> Result<&mut Comment, TypeCastError> {Err(TypeCastError::new("Cannot cast Text as Comment"))}

	fn as_text_mut(&mut self) -> Result<&mut Text, TypeCastError> {Ok(self)}

	fn as_node(&self) -> &dyn Node {self}

	fn as_node_mut(&mut self) -> &mut dyn Node {self}

	fn to_string_with_indent(&self, _indent: &str) -> String {
		self.content.clone()
	}

	fn boxed(self) -> Box<dyn Node> {
		Box::new(self)
	}
}

impl PartialOrd for Text {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.content.partial_cmp(&other.content)
	}
}

impl PartialEq<Self> for Text {
	fn eq(&self, other: &Self) -> bool {
		self.content.eq(&other.content)
	}
}

impl Hash for Text {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.content.hash(state)
	}
}


impl std::fmt::Display for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl std::fmt::Debug for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
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
		let content: String = comment.into();
		Self{content}
	}
}

impl Node for Comment {

	fn text(&self) -> Option<String> {
		Some(self.content.clone())
	}

	fn is_element(&self) -> bool {
		false
	}

	fn is_text(&self) -> bool {
		false
	}

	fn is_comment(&self) -> bool {
		true
	}

	fn as_element(&self) -> Result<&Element, TypeCastError> {Err(TypeCastError::new("Cannot cast Comment as Element"))}

	fn as_comment(&self) -> Result<&Comment, TypeCastError> {Ok(&self)}

	fn as_text(&self) -> Result<&Text, TypeCastError> {Err(TypeCastError::new("Cannot cast Comment as Text"))}

	fn as_element_mut(&mut self) -> Result<&mut Element, TypeCastError> {Err(TypeCastError::new("Cannot cast Comment as Element"))}

	fn as_comment_mut(&mut self) -> Result<&mut Comment, TypeCastError> {Ok(self)}

	fn as_text_mut(&mut self) -> Result<&mut Text, TypeCastError> {Err(TypeCastError::new("Cannot cast Comment as Text"))}

	fn as_node(&self) -> &dyn Node {self}

	fn as_node_mut(&mut self) -> &mut dyn Node {self}

	fn to_string_with_indent(&self, _indent: &str) -> String {
		format!("<!--{}-->", self.content)
	}

	fn boxed(self) -> Box<dyn Node> {
		Box::new(self)
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
		self.content.partial_cmp(&other.content)
	}
}

impl PartialEq<Self> for Comment {
	fn eq(&self, other: &Self) -> bool {
		self.content.eq(&other.content)
	}
}

impl Hash for Comment {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.content.hash(state)
	}
}

impl std::fmt::Display for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string_with_indent("  "))
	}
}

impl std::fmt::Debug for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string_with_indent("  "))
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
