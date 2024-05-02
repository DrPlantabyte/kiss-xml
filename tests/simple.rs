use kiss_xml;
use kiss_xml::dom::*;
use kiss_xml::errors::*;
use std::collections::HashMap;
use std::error::Error;

#[test]
fn test_strings() -> Result<(), Box<dyn Error>> {
	eprintln!("starting test test_strings()...");
	let mut e1 = Element::new_with_text(
		"tree", "bark!"
	)?;
	eprintln!("{}", e1);
	let e2 = Element::new_with_attributes_and_text(
		"bob", HashMap::from([("a","b")]), "hi there!"
	)?;
	eprintln!("{}", e2);
	e1.append(e2);
	eprintln!("{}", e1);
	//
	eprintln!("...test test_strings() complete");
	Ok(())
}