#[test]
fn test_xml_escapes() {
	use kiss_xml;
	let unescaped = r#"&<>'""#;
	let escaped = "&amp;&lt;&gt;&apos;&quot;";
	let escaped_text = r#"&amp;&lt;&gt;'""#;
	let escaped_attribute = "&amp;<>&apos;&quot;";
	assert_eq!(kiss_xml::escape(unescaped), escaped, "Incorrect escaping of XML reserved characters");
	assert_eq!(kiss_xml::unescape(escaped), unescaped, "Incorrect unescaping of XML reserved characters");
	assert_eq!(kiss_xml::text_escape(unescaped), escaped_text, "Incorrect escaping of XML reserved characters");
	assert_eq!(kiss_xml::attribute_escape(unescaped), escaped_attribute, "Incorrect escaping of XML reserved characters");
}

#[test]
fn test_load_from_file() {
	use kiss_xml;
	use kiss_xml::dom::*;
	use tempfile::tempdir;
	use std::fs::File;
	use std::io::{Write};
	// Write sample XML to a file
	let dir = tempdir()?;
	let file_path = dir.path().join("Note.xml");
	let mut tmpfile = File::create(file_path.clone())?;
	write!(tmpfile, r#"<?xml version="1.0" encoding="UTF-8"?>

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
</note>
"#).unwrap();
	drop(tmpfile); // close the file before re-opening

	// read the sample XML
	let doc = kiss_xml::read_from_filepath(file_path.into()).unwrap();

	// check the results
	assert_eq!(doc.root_element().name().as_str(), "note", "Root element <note> missing from document");
	assert!(doc.declaration().is_some(), "XML declaration not detected in the XML file");
	assert_eq!(doc.doctype_defs().count(), 1, "XML DTD not detected in the XML file");
	let root: Node::Element = doc.root_element();
	assert_eq!(root.child_elements().count(), 7, "Wrong number of child elements found in DOM");
	assert_eq!(root.child_nodes().count(), 8, "Wrong number of child nodes found in DOM (should be 8: 1 comment and 7 elements)");
	assert_eq!(root.child_nodes().filter(|n| n.is_element()).count(), 7, "Wrong number of element nodes found in root child nodes");
	assert_eq!(root.child_nodes().filter(|n| n.is_comment()).count(), 1, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.child_nodes().filter(|n| n.is_text()).count(), 0, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.first_element_by_name("to").unwrap().text().unwrap().as_str(), "Jani", "content of <to> is wrong");
	assert_eq!(root.elements_by_name("paragraph").count(), 2, "Wrong number of <paragraph> elements found in DOM");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().text().unwrap().as_str(), "Don't forget me this weekend!", "content of first <paragraph> is wrong");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().all_nodes()[0].text().unwrap().as_str(), "Don't forget ", "content of first <paragraph> first node is wrong");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().child_nodes().count(), 3, "First <paragraph> should have 3 nodes: text, element, text");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().child_nodes().filter(|n| n.is_text()).count(), 0, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.elements_by_name("paragraph")[1].text().unwrap().as_str(), " - Jani", "Wrong number of <paragraph> elements found in DOM");
	assert_eq!(root.first_element_by_name("signed").unwrap().get_attr("signer").unwrap(), "Jani Jane", "Attribute 'signer' of <signed> should be 'Jani Jane'");
	assert!(root.first_element_by_name("signed").unwrap().get_attr("nonexistant").is_none(), "<signed> should not have attribute 'nonexistant'");
	assert_eq!(root.search_nodes().count(), 18, "Wrong number of nodes found in recursive search of root element");
	assert!(root.first_element_by_name("b").is_none(), "<b> is not a child of the root element (is grand-child)");
	assert_eq!(root.search_elements_by_name("b").count(), 1, "Did not find <b> in recursive search");
	assert_eq!(root.search_elements_by_name("b").collect().first().unwrap().text(), "me", "Did not find text for <b> in recursive search");
}