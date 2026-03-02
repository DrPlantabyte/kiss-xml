/*!
This test file contains misc. tests to improve code coverage in tests.
*/
use kiss_xml;
use kiss_xml::dom::*;
use kiss_xml::errors::*;

/// check error printing
#[test]
fn test_error_printing(){
	eprintln!("=== printing example error messages ===");
	eprintln!("ParsingError: {}", KissXmlError::from(ParsingError::new("test message")));
	eprintln!("TypeCastError: {}", KissXmlError::from(TypeCastError::new("test message")));
	eprintln!("DoesNotExistError: {}", KissXmlError::from(DoesNotExistError::new("test message")));
	eprintln!("IndexOutOfBounds: {}", KissXmlError::from(IndexOutOfBounds::new(2, Some((0, 1)))));
	eprintln!("InvalidAttributeName: {}", KissXmlError::from(InvalidAttributeName::new("test message")));
	eprintln!("InvalidElementName: {}", KissXmlError::from(InvalidElementName::new("test message")));
	eprintln!("InvalidContent: {}", KissXmlError::from(InvalidContent::new("test message")));
	eprintln!("NotSupportedError: {}", KissXmlError::from(NotSupportedError::new("test message")));
	eprintln!("=== done printing error messages ===");
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

/// test DTDs
#[test]
fn test_dtd_handling(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE note [
<!ENTITY ignore "kiss-xml ignores DOCTYPE stuff">
<!ENTITY nbsp "&#xA0;">
<!ENTITY writer "Writer: Donald Duck.">
<!ENTITY copyright "Copyright: W3Schools.">
]>
<note>
	<!-- Note: commented out the following elements:
	<region>somewhere</region>
	<language>ISL-2108</language>
	-->
	<to>Tove</to>
	<from>Jani</from>
	<heading>Reminder</heading>
	<paragraph>Don't forget <b>me</b> this weekend!</paragraph>
	<paragraph> - Jani</paragraph>
	<footer>&writer;&nbsp;&copyright;</footer>
	<signed signer="Jani Jane"/>
</note>"#;
	let mut doc = kiss_xml::parse_str(xml).unwrap();
	assert!(doc.doctype_defs().count() == 1);
	assert!(doc.doctype_defs_mut().count() == 1);
	assert!(DTD::from_string("note1").is_err());
	assert!(DTD::from_string("<!DOCTYPE note1 >").is_ok());
	let new_dtds = [
		DTD::from_string("<!DOCTYPE note1 >").unwrap(),
		DTD::from_string("<!DOCTYPE note2 >").unwrap(),
	];
	doc.set_doctype_defs(Some(&new_dtds));
	assert!(doc.doctype_defs().count() == new_dtds.len());
}

/// test custom/removal of XML declaration
#[test]
fn test_declaration(){
	let mut doc = Document::new(Element::new_from_name("root").unwrap());
	assert!(doc.to_string().starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
	doc.set_declaration(None);
	assert!(doc.to_string().starts_with("<root"));
}
