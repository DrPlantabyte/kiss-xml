use kiss_xml;
use kiss_xml::dom::*;
use std::collections::HashMap;
use std::error::Error;

#[test]
fn test_strings() -> Result<(), Box<dyn Error>> {
	eprintln!("starting test test_strings()...");
	let mut e1 = Element::new_with_children(
		"tree", vec![Element::new_with_text("speak", "bark!")?.boxed()]
	)?;
	eprintln!("{}", e1);
	let c1 = Comment::new("no comment")?;
	eprintln!("{}", c1);
	let t1 = Text::new("bark! bark!");
	eprintln!("{}", t1);
	let mut e2 = Element::new_with_attributes_and_text(
		"bob", HashMap::from([("a","b")]), "I'm Bob."
	)?;
	e2.append(Element::new_with_text("greeting", "hi there!")?);
	eprintln!("{}", e2);
	e1.append(e2);
	e1.append(c1);
	e1.append(Element::new_with_children("speak", vec![t1.boxed()])?);
	eprintln!("{}", e1);
	//
	assert_eq!(
		e1.to_string().as_str(),
		r#"<tree>
  <speak>bark!</speak>
  <bob a="b">I'm Bob.<greeting>hi there!</greeting></bob>
  <!--no comment-->
  <speak>bark! bark!</speak>
</tree>"#,
		"incorrect string representation"
	);
	//
	eprintln!("...test test_strings() complete");
	Ok(())
}

#[test]
fn test_strings_inline() -> Result<(), Box<dyn Error>> {
	eprintln!("starting test test_strings()...");
	let mut e1 = Element::new_with_text(
		"tree", "bark!"
	)?;
	eprintln!("{}", e1);
	let c1 = Comment::new("no comment")?;
	eprintln!("{}", c1);
	let t1 = Text::new("bark! bark!");
	eprintln!("{}", t1);
	let mut e2 = Element::new_with_attributes_and_text(
		"bob", HashMap::from([("a","b")]), "I'm Bob."
	)?;
	e2.append(Element::new_with_text("greeting", "hi there!")?);
	eprintln!("{}", e2);
	e1.append(e2);
	e1.append(c1);
	e1.append(t1);
	eprintln!("{}", e1);
	//
	assert_eq!(
		e1.to_string().as_str(),
		r#"<tree>bark!<bob a="b">I'm Bob.<greeting>hi there!</greeting></bob><!--no comment-->bark! bark!</tree>"#,
		"incorrect string representation"
	);
	//
	eprintln!("...test test_strings() complete");
	Ok(())
}

#[test]
fn test_node_eq() {
	use kiss_xml::dom::*;
	assert!(
		node_eq(&Text::new("Bobalina").boxed(), &Text::new("Bobalina").boxed()),
		"incorrect comparison"
	);
	assert!(
		!node_eq(&Text::new("Bob").boxed(), &Text::new("Bobalina").boxed()),
		"incorrect comparison"
	);
	assert!(
		!node_eq(&Text::new("Bobalina").boxed(), &Comment::new("Bobalina").unwrap().boxed()),
		"incorrect comparison"
	);
	assert!(
		node_eq(&Comment::new("Bobalina").unwrap().boxed(),
				&Comment::new("Bobalina").unwrap().boxed()),
		"incorrect comparison"
	);
	assert!(
		!node_eq(&Comment::new("Billbalina").unwrap().boxed(),
				&Comment::new("Bobalina").unwrap().boxed()),
		"incorrect comparison"
	);
	assert!(
		node_eq(&CData::new("Bobalina").unwrap().boxed(),
				&CData::new("Bobalina").unwrap().boxed()),
		"incorrect comparison"
	);
	assert!(
		!node_eq(&CData::new("Bobalina").unwrap().boxed(),
				&CData::new("Ted").unwrap().boxed()),
		"incorrect comparison"
	);
	assert!(
		node_eq(
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://some/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed(),
					Comment::new("no comment").unwrap().boxed(),
					CData::new("<html><body>Correct HTML<br>is not valid XML</body></html>").unwrap().boxed(),
					Element::new_with_text("child", "yet more text").unwrap().boxed()
				])
			).unwrap().boxed(),
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://some/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed(),
					Comment::new("no comment").unwrap().boxed(),
					CData::new("<html><body>Correct HTML<br>is not valid XML</body></html>").unwrap().boxed(),
					Element::new_with_text("child", "yet more text").unwrap().boxed()
				])
			).unwrap().boxed()
		),
		"incorrect comparison"
	);
	assert!(
		!node_eq(
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://some/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed(),
					Comment::new("no comment").unwrap().boxed(),
					CData::new("<html><body>Correct HTML<br>is not valid XML</body></html>").unwrap().boxed(),
					Element::new_with_text("child", "yet more text").unwrap().boxed()
				])
			).unwrap().boxed(),
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://other/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed(),
					Comment::new("no comment").unwrap().boxed(),
					CData::new("<html><body>Correct HTML<br>is not valid XML</body></html>").unwrap().boxed(),
					Element::new_with_text("child", "yet more text").unwrap().boxed()
				])
			).unwrap().boxed()
		),
		"incorrect comparison"
	);
	assert!(
		!node_eq(
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://some/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed()
				])
			).unwrap().boxed(),
			&Element::new(
				"tagname", Some("text".into()), Some(HashMap::from([("a","1")])),
				Some("http://some/xmlns".into()), Some("prefix".into()), Some(vec![
					Text::new("more text").boxed(),
					Comment::new("no comment").unwrap().boxed(),
					CData::new("<html><body>Correct HTML<br>is not valid XML</body></html>").unwrap().boxed(),
					Element::new_with_text("child", "yet more text").unwrap().boxed()
				])
			).unwrap().boxed()
		),
		"incorrect comparison"
	);
}

#[test]
fn test_clone_node() {
	use kiss_xml;
	use kiss_xml::dom::*;
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<!--comment-->
	<mydata>
		This is my data
		<properties>
			<property name="a" value="1"/>
			<property name="b" value="2"/>
		</properties>
		<meta>My metadata goes here</meta>
		<other>
			<![CDATA[<html><body>This is<br>not XML</body></html>]]>
		</other>
		<other/>
	</mydata>
</root>
"#;
	let dom = kiss_xml::parse_str(xml).unwrap();
	for n1 in dom.root_element().all_children() {
		let n2 = clone_node(n1);
		assert!(node_eq(n1, &n2), "cloned node not equal to original")
	}
}

#[test]
fn test_leading_trailing_text() {
	use kiss_xml;
	let xml1 = r#"<xhtml>
  <body>Linda <b>ran</b> to the store <i>to buy ☼ cookies</i> for the party</body>
</xhtml>
	"#;
	eprintln!("xml1:\n{}", xml1.replace(" ", "·"));
	let doc = kiss_xml::parse_str(xml1).unwrap();
	let child_nodes = doc.root_element()
		.first_element_by_name("body").unwrap()
		.children().collect::<Vec<_>>();
	eprintln!("parsed to:\n{}", doc.to_string().replace(" ", "·"));
	for c in &child_nodes {
		eprintln!("{}: {}", c.node_type(), c.text().replace(" ", "·"));
	}
	assert_eq!(child_nodes[0].text().as_str(), "Linda ", "Failed to preserve trailing space while parsing");
	assert_eq!(child_nodes[2].text().as_str(), " to the store ",  "Failed to preserve leading space while parsing");
	let xml2 = doc.to_string();
	eprintln!("xml2:\n{}", xml2.replace(" ", "·"));
	let doc2 = kiss_xml::parse_str(xml2).unwrap();
	let child_nodes = doc2.root_element()
		.first_element_by_name("body").unwrap()
		.children().collect::<Vec<_>>();
	eprintln!("after serialize/de-serialize cycle:\n{}", doc2.to_string().replace(" ", "·"));
	for c in &child_nodes {
		eprintln!("{}: {}", c.node_type(), c.text().replace(" ", "·"));
	}
	assert_eq!(doc, doc2, "Lost information in serialize/de-serialize cycle");
	assert_eq!(child_nodes[0].text().as_str(), "Linda ", "Failed to preserve trailing space after serialize/de-serialize cycle");
	assert_eq!(child_nodes[2].text().as_str(), " to the store ",  "Failed to preserve leading after serialize/de-serialize cycle");
}
