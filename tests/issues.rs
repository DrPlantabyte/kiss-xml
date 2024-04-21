#![deny(unused_must_use)]
#![deny(missing_docs)]

/**
# Summary
This test confirms that attributes are sorted in correct order when the DOM is converted to a string.

See https://github.com/DrPlantabyte/kiss-xml/issues/12
*/
fn test_issue_12() {
	use kiss_xml;
	let unsorted = r#"<root beta="1" alpha="2" xmlns:b="internal://b/b" xmlns="internal://a/b" xmlns:a="internal://a/a"/>"#;
	let sorted = r#"<root xmlns="internal://a/b"  xmlns:a="internal://a/a" xmlns:b="internal://b/b" alpha="2" beta="1"/>"#;
	assert_eq!(
		kiss_xml::parse_str(unsorted).expect("failed to parse XML").to_string().as_str(),
		sorted,
		"Test failed for issue 12: https://github.com/DrPlantabyte/kiss-xml/issues/12"
	);
}
