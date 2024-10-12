//! Example usage tests

/// Simple example of read-only usage
#[test]
fn example1() {
	use kiss_xml;

	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<config>
	<name>My Settings</name>
	<sound>
		<property name="volume" value="11" />
		<property name="mixer" value="standard" />
	</sound>
</config>
"#;
	// parse XML to a document object model (DOM)
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	// print the DOM
	println!("Parsed:\n{dom}");
	// print children of the root element
	for e in dom.root_element().child_elements() {
		println!("child element <{}>", e.name())
	}
	// print all elements as a flattened list
	for e in dom.root_element().search_elements(|_| true) {
		println!("found element <{}>", e.name())
	}
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
}
