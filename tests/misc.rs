/*!
This test file contains misc. tests to improve code coverage in tests.
*/
use kiss_xml;
use kiss_xml::dom::*;
use kiss_xml::errors::*;

/// check error printing
#[test]
fn test_error_printing(){
	eprintln!("printing example error messages");
	eprintln!("ParsingError: {}", ParsingError::new("test message"));
	eprintln!("TypeCastError: {}", TypeCastError::new("test message"));
	eprintln!("DoesNotExistError: {}", DoesNotExistError::new("test message"));
	eprintln!("IndexOutOfBounds: {}", IndexOutOfBounds::new(2, Some((0, 1))));
	eprintln!("InvalidAttributeName: {}", InvalidAttributeName::new("test message"));
	eprintln!("InvalidElementName: {}", InvalidElementName::new("test message"));
	eprintln!("InvalidContent: {}", InvalidContent::new("test message"));
	eprintln!("NotSupportedError: {}", NotSupportedError::new("test message"));
}

/// test expected error conditions
#[test]
fn test_typecast_error(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>some text
	<!--comment-->
	more text
	<properties>
		<property name="a" value="1"/>
	</properties>
	even more text
	<![CDATA[<html><body>This is not<br>XML</body></html>]]>
</root>
"#;
	let doc = kiss_xml::parse_str(xml).unwrap();
	let root = doc.root_element();
	let children = root.children().collect::<Vec<_>>();
	assert!(children[0].as_cdata().is_err());
	assert!(children[0].as_comment().is_err());
	assert!(children[0].as_element().is_err());
	assert!(children[0].as_text().is_ok());
	assert!(children[1].as_cdata().is_err());
	assert!(children[1].as_comment().is_ok());
	assert!(children[1].as_element().is_err());
	assert!(children[1].as_text().is_err());
	assert!(children[3].as_cdata().is_err());
	assert!(children[3].as_comment().is_err());
	assert!(children[3].as_element().is_ok());
	assert!(children[3].as_text().is_err());
	assert!(children[5].as_cdata().is_ok());
	assert!(children[5].as_comment().is_err());
	assert!(children[5].as_element().is_err());
	assert!(children[5].as_text().is_err());
}

/// test expected error conditions
#[test]
fn test_dne_error(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root><properties/></root>
"#;
	let doc = kiss_xml::parse_str(xml).unwrap();
	let root = doc.root_element();
	assert!(root.first_element_by_name("does-not-exist").is_err());
}

/// test expected error conditions
#[test]
fn test_index_error(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root><properties/></root>
"#;
	let mut doc = kiss_xml::parse_str(xml).unwrap();
	let mut root = doc.root_element_mut();
	assert!(root.insert(2, Text::new("insert")).is_err());
	assert!(root.remove(1).is_err());
	assert!(root.remove_element(1).is_err());
}

/// test expected error conditions
#[test]
fn test_invalid_attribute_name_error(){
	let mut root = Element::new_from_name("root").unwrap();
	let result = root.set_attr("-invalid-attribute", "error");
	assert!(result.is_err());
}

/// test expected error conditions
#[test]
fn test_invalid_element_name_error(){
	let result = Element::new_from_name("-invalid-element");
	assert!(result.is_err());
	let e = result.unwrap_err();
	match e {
		KissXmlError::InvalidElementName(_) => {}
		_ => panic!("Expected InvalidElementName, got {}", e),
	}
}



