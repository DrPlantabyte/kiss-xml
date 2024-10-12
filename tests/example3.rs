//! Example usage tests

/// API usage example
#[test]
fn example3() -> Result<(), kiss_xml::errors::KissXmlError>{
	use kiss_xml;
	use kiss_xml::dom::*;
	// make a DOM from scratch
	let mut doc = Document::new(Element::new_from_name("politicians").unwrap());
	doc.root_element_mut().append(Element::new_with_text("person", "Hillary Clinton").unwrap());
	doc.root_element_mut().append(Element::new_with_text("person", "Bob Dole").unwrap());
	// print to terminal
	println!("{}", doc.root_element());
	// write to file
	doc.write_to_filepath("politics.xml")?;
	Ok(())
}
