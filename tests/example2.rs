
#[test]
fn example1() {
	use kiss_xml;
	use kiss_xml::dom::Element;
	use std::collections::HashMap;

	let xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- Blue triangle SVG graphic -->
<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <g id="layer1">
    <path style="fill:#00a6c2;fill-opacity:1;stroke:none"
       d="M 3,58 57,11 42,64 Z" id="triangle" />
  </g>
</svg>"#;
	// parse SVG XML to a document object model (DOM)
	let mut doc = kiss_xml::parse_str(xml).expect("Error parsing XML");
	// add a red square
	doc.root_element_mut()
		.first_element_by_name_mut("g").expect("no <g> element found")
		.append(Element::new_with_attributes("path", HashMap::from([
			("style", "fill:#ff0000;fill-opacity:0.5;stroke:none"),
			("d", "M 25,25 25,75 75,75, 75,25 Z"),
			("id", "square")
		])).unwrap());
	// print the new modified SVG XML
	println!("{}", doc.to_string())
}
