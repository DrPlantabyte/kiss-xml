#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(dead_code)]
//! This file is for adding unit tests that correspond to issues tracked in GitHub
//!
//! Each test here must have a description with a link to the issue in GitHub
//! and each issue in GitHub that is addressed must have a unit test here.

/**
# Summary
This test confirms that attributes are sorted in correct order when the DOM is converted to a string.

See https://github.com/DrPlantabyte/kiss-xml/issues/12
*/
#[test]
fn test_issue_12() {
	use kiss_xml;
	let unsorted = r#"<root beta="1" alpha="2" xmlns:b="internal://b/b" xmlns="internal://a/b" xmlns:a="internal://a/a"/>"#;
	let sorted = r#"<root xmlns="internal://a/b" xmlns:a="internal://a/a" xmlns:b="internal://b/b" alpha="2" beta="1"/>"#;
	assert_eq!(
		kiss_xml::parse_str(unsorted).expect("failed to parse XML").to_string().as_str().trim(),
		sorted.trim(),
		"Test failed for issue 12: https://github.com/DrPlantabyte/kiss-xml/issues/12"
	);
}

/**
Test for adding CDATA support

See https://github.com/DrPlantabyte/kiss-xml/issues/17
*/
#[test]
fn test_issue_17_parse() {
	use kiss_xml;
	use kiss_xml::dom::Node;
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
	<!--comment-->
	<properties>
		<property name="a" value="1"/>
	</properties>
	<mydata><![CDATA[<html><body>This is not<br>XML</body></html>]]></mydata>
</root>
"#;
	let dom = kiss_xml::parse_str(xml).unwrap();
	let mydata_elem = dom.root_element().first_element_by_name("mydata").unwrap();
	let cdata_node = mydata_elem.children().next().unwrap();
	assert!(cdata_node.is_cdata(), "<![CDATA[...]]> not parsed as CDATA");
	assert_eq!(
		cdata_node.as_cdata().unwrap().text(),
		"<html><body>This is not<br>XML</body></html>",
		"test failed for issue 17: https://github.com/DrPlantabyte/kiss-xml/issues/17"
	);
	assert_eq!(
		dom.to_string_with_indent("\t").as_str(),
		xml,
		"test failed for issue 17: https://github.com/DrPlantabyte/kiss-xml/issues/17"
	);
}

/**
Test for adding CDATA support

See https://github.com/DrPlantabyte/kiss-xml/issues/17
 */
#[test]
fn test_issue_17_modify() {
	use kiss_xml;
	use kiss_xml::dom::*;
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
	<!--comment-->
	<properties>
		<property name="a" value="1"/>
	</properties>
	<mydata/>
</root>
"#;
	let mut dom = kiss_xml::parse_str(xml).unwrap();
	let mydata_elem = dom.root_element_mut().first_element_by_name_mut("mydata").unwrap();
	mydata_elem.append(CData::new("<html><body>This is not<br>XML</body></html>").unwrap());
	assert_eq!(
		dom.to_string_with_indent("\t").as_str(),
		r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
	<!--comment-->
	<properties>
		<property name="a" value="1"/>
	</properties>
	<mydata><![CDATA[<html><body>This is not<br>XML</body></html>]]></mydata>
</root>
"#,
		"test failed for issue 17: https://github.com/DrPlantabyte/kiss-xml/issues/17"
	);
}


/**
Malformed XML tag with = in it is causing a panic.

See https://github.com/DrPlantabyte/kiss-xml/issues/21
*/
#[test]
fn test_issue_21_panic() {
	use kiss_xml;
	let xml = r#"
<property =
<property />
"#;
	let result = kiss_xml::parse_str(xml);
	assert!(result.is_err());
	println!("{:?}", result.err());
}

/**
Calling Element.remove_element(usize) when there are no children causes it to try to remove index 0 from a vec instead of creating the proper error

See https://github.com/DrPlantabyte/kiss-xml/issues/26
*/
#[test]
fn test_issue_26_panic() {
	use kiss_xml;
	let xml = r#"
<properties />
"#;
	let mut dom = kiss_xml::parse_str(xml).unwrap();
	let element: &mut kiss_xml::dom::Element = dom.root_element_mut();
	let result = element.remove_element(0);
	assert!(result.is_err());
	println!("{:?}", result.err());
}

/**
Panic on improperly quoted attributes

See https://github.com/DrPlantabyte/kiss-xml/issues/28
*/
#[test]
fn test_issue_28_panic() {
	use kiss_xml;
	let xml = r#"<ot Xttr=t Xttr"valud</ro>o">"#;

	assert!(kiss_xml::parse_str(xml).is_err());
}

/**
Panic on "<!-->" in content of root element.

See https://github.com/DrPlantabyte/kiss-xml/issues/29
*/
#[test]
fn test_issue_29_panic() {
	use kiss_xml;
	let xml = r#"<root><!--></root>"#;
	assert!(kiss_xml::parse_str(xml).is_err());
}

/**
Data corruption due to incorrect application of XML escapes

See https://github.com/DrPlantabyte/kiss-xml/issues/34
*/
#[test]
fn test_issue_34_escaped_text() {
	use kiss_xml;
	let xml = "<ProductName>&lt;AppName&gt;</ProductName>\n";
	assert_eq!(kiss_xml::parse_str(xml).expect("xml parse error").to_string().as_str(), xml);
}

/**
XML spec allows for upt ot one space before and after an equal sign in a tag's attributes

See https://github.com/DrPlantabyte/kiss-xml/issues/33
*/
#[test]
fn test_issue_33_spaced_attributes() {
	use kiss_xml;
	let xml = r#"<root a="1" b = "2" c= "3" d ="4" x="don't be fooled by =" y=" in value"/>"#;
	assert_eq!(
		kiss_xml::parse_str(xml).expect("xml parse error").to_string().as_str().trim(),
		r#"<root a="1" b="2" c="3" d="4" x="don&apos;t be fooled by =" y=" in value"/>"#
	);
}
