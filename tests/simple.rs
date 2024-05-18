use kiss_xml;
use kiss_xml::dom::*;
use std::collections::HashMap;
use std::error::Error;

#[test]
fn test_strings() -> Result<(), Box<dyn Error>> {
	eprintln!("starting test test_strings()...");
	let mut e1 = Element::new_with_text(
		"tree", "bark!"
	)?;
	eprintln!("{}", e1);
	let c1 = Comment::new("no comment");
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
r#"<tree>
  bark!
  <bob a="b">
    I'm Bob.
    <greeting>hi there!</greeting>
  </bob>
  <!--no comment-->
  bark! bark!
</tree>"#,
		"incorrect string representation"
	);
	//
	eprintln!("...test test_strings() complete");
	Ok(())
}