# KISS-XML: Keep It Super Simple XML
![GitHub Workflow Build Status](https://github.com/DrPlantabyte/kiss-xml/actions/workflows/build-main.yml/badge.svg) ![GitHub Workflow Test Status](https://github.com/DrPlantabyte/kiss-xml/actions/workflows/unit-test-main.yml/badge.svg) [![codecov](https://codecov.io/gh/DrPlantabyte/kiss-xml/branch/main/graph/badge.svg?token=SA5UFPQG7A)](https://codecov.io/gh/DrPlantabyte/kiss-xml) [![Crate.io](https://img.shields.io/crates/v/kiss-xml)](https://crates.io/crates/kiss-xml) [![Redistribution license](https://img.shields.io/github/license/DrPlantabyte/kiss-xml?color=green)](https://github.com/DrPlantabyte/kiss-xml/blob/main/kiss-xml/LICENSE)

This Rust library provides an easy-to-use Document Object Model (DOM) for 
reading and writing XML files. Unlike many other XML parsers, KISS-XML simply
parses the given XML to a full DOM, which you can then modify and serialize back
to XML. No schemas or looping required.

## What's included:
KISS-XML provides the basics for XML documents, including:
* Parse XML files and strings to a DOM
* XML elements, text, and comments
* DOM is mutable and can be saved as a string and to files
* Easy to use

## What's NOT included:
* Namespace support
* Schema handling
* CDATA
* Document type declarations (DTDs will be preserved but not interpreted)
* Parsing character encodings other than UTF-8
* Typed XML data (eg integer attribute values)
* Performance optimizations (prioritizing easy-to-use over fast)

If you need any of the above XML features, then this library is too simple for
your needs. Try another XML parsing crate instead.

## Quickstart Guide
First, add the following to your Cargo.toml file:
```text
kiss_xml = "1"
```

Then to parse an XML file, all you need to do is call the
`kiss_xml::parse_filepath(...)` function, like this:

```rust
fn main() {
	let doc = kiss_xml::parse_filepath("my-file.xml").unwrap();
    println!("{}", doc.to_string());
}
```

The XML content will be converted into a Document Object Model (DOM) with a
single root element. A DOM is a tree-like data structure made up of XML Element,
Text, and Comment nodes. You can explore the DOM element-by-element with the
`.elements_by_name(&str)` and `.first_element_by_name(&str)` methods, scan the
children of an element with the `.child_*()` methods, or do a recursive search
using the `.search(...)` and `.search_*(...)` methods.

For example:
```rust
fn main() {
	use kiss_xml;
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
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	// print all sound properties
	let properties = dom.root_element()
		.first_element_by_name("sound").expect("No <sound> element")
		.elements_by_name("property");
	for prop in properties {
		println!(
			"{} = {}",
			prop.get_attr("name").expect("missing name attribute"),
			prop.get_attr("value").expect("missing value attribute")
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
}
```

To modify the DOM, use the `.*_mut(...)` methods to get mutable references to
the elements, and you can convert the DOM to a string with the `.to_string()`
method or write it to a file with `.write_to_filepath(...)`.

For example:
```rust
fn main() {
	use kiss_xml;
	use kiss_xml::dom::*;
	// make a DOM from scratch
	let mut doc = Document::new(Element::new("politicians"));
	doc.root_element_mut().append(Element::new_with_text("person", "Hillary Clinton"));
	doc.root_element_mut().append(Element::new_with_text("person", "Bob Dole"));
	// write to file
	doc.write_to_filepath("politics.xml");
}
```

## License
This library is open source, licensed under the MIT License. You may use it
as-is or with modification, without any limitations.

