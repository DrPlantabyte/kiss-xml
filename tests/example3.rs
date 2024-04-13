
#[test]
fn example3() {
	use kiss_xml;
	use kiss_xml::dom::*;
	// make a DOM from scratch
	let mut doc = Document::new(Element::new_from_name("politicians"));
	doc.root_element_mut().append(Element::new_with_text("person", "Hillary Clinton"));
	doc.root_element_mut().append(Element::new_with_text("person", "Bob Dole"));
	// write to file
	doc.write_to_filepath("politics.xml");
}