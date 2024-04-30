use std::collections::HashMap;

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

fn sample_xml_1() -> &'static str {
	r#"<?xml version="1.0" encoding="UTF-8"?>
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
"#
}

fn sample_xml_2() -> &'static str {
	r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<!--comment-->
	<mydata>
		This is my data
		<properties>
			<property name="a" value="1" />
			<property name="b" value="2" />
		</properties>
		<meta>
			My metadata goes here
		</meta>
		<other/>
		<other/>
	</mydata>
</root>"#
}

fn sample_xml_3() -> &'static str {
	r#"<?xml version="1.0" encoding="UTF-8"?>
<root xmlns="internal://ns/a">
	<width>200</width>
	<height>150</height>
</root>"#
}

fn sample_xml_4() -> &'static str {
	r#"<?xml version="1.0" encoding="UTF-8"?>
<root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
	<width>200</width>
	<height>150</height>
	<depth>50</depth>
	<img:width>200</img:width>
	<img:height>150</img:height>
	<dim:width>200</dim:width>
</root>"#
}

fn sample_xml_5() -> &'static str {
	// Note: XML elements only inherit the default namespace of their parent, not the prefixed namespace
	r#"<?xml version="1.0" encoding="UTF-8"?>
<img:root xmlns:img="internal://ns/a" xmlns:dim="internal://ns/b">
	<width>200</width>
	<height>150</height>
	<img:width>200</img:width>
	<img:height>150</img:height>
	<dim:width>200</dim:width>
	<dim:height>150</dim:height>
</root>"#
}

#[test]
fn test_load_from_file() {
	use kiss_xml;
	use tempfile::tempdir;
	use std::fs::File;
	use std::io::{Write};
	// Write sample XML to a file
	let dir = tempdir().unwrap();
	let file_path = dir.path().join("Note.xml");
	let mut tmpfile = File::create(file_path.clone()).unwrap();
	write!(tmpfile, "{}", sample_xml_1()).unwrap();
	drop(tmpfile); // close the file before re-opening

	// read the sample XML
	let doc = kiss_xml::parse_filepath(file_path.as_path()).unwrap();
	let doc2 = kiss_xml::parse_str(sample_xml_1()).unwrap();
	assert_eq!(doc, doc2, "File and string parsers diverged!");
}

#[test]
fn test_dom_parsing() {
	use kiss_xml;
	use kiss_xml::dom::*;
	let doc = kiss_xml::parse_str(sample_xml_1()).unwrap();
	// check the results
	assert_eq!(doc.root_element().name().as_str(), "note", "Root element <note> missing from document");
	assert!(doc.declaration().is_some(), "XML declaration not detected in the XML file");
	assert_eq!(doc.doctype_defs().count(), 1, "XML DTD not detected in the XML file");
	let root = doc.root_element();
	assert_eq!(root.child_elements().count(), 7, "Wrong number of child elements found in DOM");
	assert_eq!(root.children().count(), 8, "Wrong number of child nodes found in DOM (should be 8: 1 comment and 7 elements)");
	assert_eq!(root.children().filter(|n| n.is_element()).count(), 7, "Wrong number of element nodes found in root child nodes");
	assert_eq!(root.children().filter(|n| n.is_comment()).count(), 1, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.children().filter(|n| n.is_text()).count(), 0, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.first_element_by_name("to").unwrap().text().unwrap().as_str(), "Jani", "content of <to> is wrong");
	assert_eq!(root.elements_by_name("paragraph").count(), 2, "Wrong number of <paragraph> elements found in DOM");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().text().unwrap().as_str(), "Don't forget me this weekend!", "content of first <paragraph> is wrong");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().children().collect::<Vec<_>>()[0].text().unwrap().as_str(), "Don't forget ", "content of first <paragraph> first node is wrong");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().children().count(), 3, "First <paragraph> should have 3 nodes: text, element, text");
	assert_eq!(root.first_element_by_name("paragraph").unwrap().children().filter(|n| n.is_text()).count(), 0, "Wrong number of comment nodes found in root child nodes");
	assert_eq!(root.elements_by_name("paragraph").collect::<Vec<_>>()[1].text().unwrap().as_str(), " - Jani", "Wrong number of <paragraph> elements found in DOM");
	assert_eq!(root.first_element_by_name("signed").unwrap().get_attr("signer").unwrap(), "Jani Jane", "Attribute 'signer' of <signed> should be 'Jani Jane'");
	assert!(root.first_element_by_name("signed").unwrap().get_attr("nonexistant").is_none(), "<signed> should not have attribute 'nonexistant'");
	assert_eq!(root.search(|_| true).count(), 18, "Wrong number of nodes found in recursive search of root element");
	assert_eq!(root.search(|n| n.is_text()).count(), 8, "Wrong number of text nodes found in recursive search of root element");
	assert!(root.first_element_by_name("b").is_none(), "<b> is not a child of the root element (is grand-child)");
	assert_eq!(root.search_elements_by_name("b").count(), 1, "Did not find <b> in recursive search");
	assert_eq!(root.search_elements_by_name("b").collect::<Vec<_>>().first().unwrap().text().unwrap(), "me", "Did not find text for <b> in recursive search");
	assert_eq!(root.search_elements(|e| e.name() == "b").count(), 1, "Did not find <b> in recursive search");
	assert_eq!(root.search_text(|s| s.text().unwrap().contains("weekend")).map(|s| s.text().unwrap()).collect::<Vec<String>>().first().unwrap().as_str(), " this weekend!", "Did not find ' this weekend!' in recursive text search");
	assert_eq!(root.search_comments(|c| c.content.contains("Note:")).count(), 1, "Did not find comment in recursive search");
	assert_eq!(root.search_comments(|c| c.content.contains("this does not exist")).count(), 0, "Found non-existent comment in recursive search");
}

#[test]
fn test_modify_dom() {
	use kiss_xml;
	use kiss_xml::dom::*;
	use std::collections::HashMap;
	let mut doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	doc.root_element_mut().set_attr("author", "some dude");
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.first_element_by_name_mut("properties").unwrap()
		.append(Element::new_with_attributes("property", HashMap::from([
			("name", "c"),
			("value", "3"),
		])).unwrap());
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.first_element_by_name_mut("properties").unwrap()
		.insert(0, Element::new_with_attributes("property", HashMap::from([
			("name", "z"),
			("value", "0"),
		])).unwrap());
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.insert(1, Comment::new("inserted comment"));
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.append(Text::new("inserted text"));
	let indent = "\t";
	let expected_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<!--comment-->
	<mydata author="some dude">
		This is my data
		<!--inserted comment-->
		<properties>
			<property name="z" value="0" />
			<property name="a" value="1" />
			<property name="b" value="2" />
			<property name="c" value="3" />
		</properties>
		<meta>
			My metadata goes here
		</meta>
		<other/>
		<other/>
		inserted text
	</mydata>
</root>"#;
	assert_eq!(doc.to_string_with_indent(indent).as_str(), expected_str, "Incorrect XML generated");
}

#[test]
fn test_remove_1(){
	use kiss_xml;
	let mut doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	doc.root_element_mut().remove_attr("author");
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.remove(0);
	doc.root_element_mut()
		.remove_all(
			|n| n.is_text() && n.text().unwrap().contains("My metadata")
		);
	doc.root_element_mut().remove_elements_by_name("other");
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.first_element_by_name_mut("properties").unwrap()
		.remove_element(1);
	
	let expected_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
	<!--comment-->
	<mydata>
		<properties>
			<property name="a" value="1" />
		</properties>
		<meta/>
	</mydata>
</root>"#;
	let indent = "\t";
	assert_eq!(doc.to_string_with_indent(indent).as_str(), expected_str, "Incorrect XML generated");
}

#[test]
fn test_remove_2(){
	use kiss_xml;
	use kiss_xml::dom::*;
	let mut doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.remove_elements(|e| e.name() == "meta");
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.remove_elements(|e| e.name() == "other");
	let expected_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<!--comment-->
	<mydata>
		This is my data
		<properties>
			<property name="a" value="1" />
			<property name="b" value="2" />
		</properties>
	</mydata>
</root>"#;
	let indent = "\t";
	assert_eq!(doc.to_string_with_indent(indent).as_str(), expected_str, "Incorrect XML generated");
}

#[test]
fn test_remove_3(){
	use kiss_xml;
	let mut doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	doc.root_element_mut()
		.first_element_by_name_mut("mydata").unwrap()
		.first_element_by_name_mut("properties").unwrap()
		.remove_elements_by_name("property");
	doc.root_element_mut()
		.remove_all(|n| n.is_comment() || (n.is_element() && n.as_element().unwrap().name() == "other"));
	let expected_str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<mydata>
		This is my data
		<properties/>
		<meta>
			My metadata goes here
		</meta>
	</mydata>
</root>"#;
	let indent = "\t";
	assert_eq!(doc.to_string_with_indent(indent).as_str(), expected_str, "Incorrect XML generated");
}

#[test]
fn test_dom_to_string() {
	use kiss_xml;
	let xml_str = sample_xml_2();
	let doc = kiss_xml::parse_str(xml_str).unwrap();
	let indent = "\t";
	assert_eq!(doc.to_string_with_indent(indent).as_str(), xml_str, "Source XML not recreated by to_string() method");
}

#[test]
fn test_dom_to_file() {
	use kiss_xml;
	use tempfile::tempdir;
	use std::fs::File;
	let xml_str = sample_xml_2();
	let doc = kiss_xml::parse_str(xml_str).unwrap();
	let indent = "\t";
	// Write sample XML to a file
	let dir = tempdir().unwrap();
	let file_path = dir.path().join("Note.xml");
	let mut tmpfile = File::create(file_path.clone()).unwrap();
	doc.write_to_file_with_indent(&mut tmpfile, indent).unwrap();
	drop(tmpfile); // close the file before re-opening
	// check what was written
	let file_content = std::fs::read_to_string(file_path).unwrap();
	assert_eq!(file_content.as_str(), xml_str, "Source XML not recreated by write_to_file() method");
}

#[test]
fn test_dom_to_filepath() {
	use kiss_xml;
	use tempfile::tempdir;
	let xml_str = sample_xml_2();
	let doc = kiss_xml::parse_str(xml_str).unwrap();
	let indent = "\t";
	assert_eq!(doc.to_string_with_indent(indent).as_str(), xml_str, "Source XML not recreated by to_string() method");
	// Write sample XML to a file
	let dir = tempdir().unwrap();
	let file_path = dir.path().join("Note.xml");
	doc.write_to_filepath_with_indent(&file_path, indent).unwrap();
	// check what was written
	let file_content = std::fs::read_to_string(file_path).unwrap();
	assert_eq!(file_content.as_str(), xml_str, "Source XML not recreated by write_to_filepath() method");
}

#[test]
fn test_display(){
	use kiss_xml;
	let doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	println!("Document:\n{}\n\n", doc);
	println!("Root Element:\n{}\n\n", doc.root_element());
}

#[test]
fn test_debug_display(){
	use kiss_xml;
	let doc = kiss_xml::parse_str(sample_xml_2()).unwrap();
	println!("Document:\n{:?}\n\n", doc);
	println!("Root Element:\n{:?}\n\n", doc.root_element());
}

#[test]
fn test_namespaces_1() {
	use std::str::FromStr;
	use kiss_xml;
	use kiss_xml::dom::*;
	let mut doc = kiss_xml::parse_str(sample_xml_3()).unwrap();
	// check that namespaces were correctly parsed (no prefix)
	assert_eq!(doc.root_element().namespace().unwrap(), "internal://ns/a", "XML namespace not correctly parsed");
	assert!(doc.root_element().namespace_prefix().is_none(), "XML namespace prefix not correctly parsed");
	assert_eq!(doc.root_element().first_element_by_name("width").unwrap().namespace().unwrap(), "internal://ns/a", "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().first_element_by_name("height").unwrap().namespace().unwrap(), "internal://ns/a", "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace(Some("internal://ns/a")).count(), 2, "XML namespace not correctly inherited");
	// check that adding a new element inherits the namespace of the parent unless otherwise specified
	doc.root_element_mut().append(Element::new::<&str,&str>("depth", Some("50"), None, None, None, None).unwrap());
	assert_eq!(doc.root_element().first_element_by_name("depth").unwrap().namespace().unwrap(), "internal://ns/a", "XML namespace not correctly inherited");
	assert!(doc.root_element().first_element_by_name("depth").unwrap().namespace_prefix().is_none(), "XML namespace prefix not correctly inherited");
}

#[test]
fn test_namespaces_2() {
	use std::str::FromStr;
	use kiss_xml;
	let mut doc = kiss_xml::parse_str(sample_xml_4()).unwrap();
	assert!(doc.root_element().namespace().is_none(), "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace(None).count(), 3, "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace_prefix(Some("img")).count(), 2, "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace_prefix(Some("dim")).count(), 1, "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace(Some("internal://ns/a")).count(), 2, "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace(Some("internal://ns/b")).count(), 1, "XML namespace not correctly parsed");
	// check to_string
	assert_eq!(doc.to_string_with_indent("\t").as_str(), sample_xml_4(), "XML not regenerated correctly")
}

#[test]
fn test_namespaces_3() {
	use kiss_xml;
	let mut doc = kiss_xml::parse_str(sample_xml_5()).unwrap();
	assert_eq!(doc.root_element().namespace_prefix().unwrap().as_str(), "img", "XML namespace not correctly parsed");
	assert_eq!(doc.root_element().elements_by_namespace_prefix(Some("img")).count(), 2, "XML namespace not correctly parsed or inherited");
	assert_eq!(doc.root_element().elements_by_namespace_prefix(None).count(), 2, "XML namespace not correctly parsed or inherited");
}

#[test]
fn test_modify_text_and_comments() {
	use kiss_xml;
	use kiss_xml::dom::*;
	use std::collections::HashMap;
	let mut doc = kiss_xml::parse_str(
r#"<html>
	<!-- this is a comment ->
	<body>
		TODO: content here
	</body>
</html>"#
	).unwrap();
	// read and remove the first comment
	let all_comments = doc.root_element().children()
		.filter(|n| n.is_comment())
		.collect::<Vec<_>>();
	let first_comment = all_comments.first().unwrap();
	println!("Comment: {}", first_comment.text().unwrap());
	doc.root_element_mut().remove_all(|n| n.is_comment());
	// replace content of <body> with some HTML
	doc.root_element_mut().first_element_by_name_mut("body").unwrap().remove_all(|_| true);
	doc.root_element_mut().first_element_by_name_mut("body").unwrap().append_all(
		vec![
			Element::new_with_text("h1", "Chapter 1").unwrap().boxed(),
			Comment::new("Note: there is only one chapter").boxed(),
			Element::new_with_children("p", vec![
				Text::new("Once upon a time, there was a little ").unwrap().boxed(),
				Element::new_with_attributes_and_text(
					"a",
					HashMap::from([("href","https://en.wikipedia.org/wiki/Gnome")]),
					"gnome"
				).unwrap().boxed(),
				Text::new(" who lived in a walnut tree...").boxed()
			]).unwrap().boxed()
		]
	);
	// print the results
	println!("{}", doc.to_string());
}

