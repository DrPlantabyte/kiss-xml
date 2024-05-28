#![deny(unused_must_use)]
#![deny(missing_docs)]
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
	use kiss_xml::dom::*;
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
	assert_eq!(cdata_node.as_cdata().unwrap().content.as_str(), xml, "XML not correctly formatted to string");
	assert_eq!(dom.to_string_with_indent("\t").as_str(), xml, "XML not correctly formatted to string");
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
	let mut mydata_elem = dom.root_element_mut().first_element_by_name_mut("mydata").unwrap();
	mydata_elem.append(CData::new("<![CDATA[<html><body>This is not<br>XML</body></html>]]>"));
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
		"XML not correctly formatted to string"
	);
}

