//! Tests to ensure that bad input causes errors instead of panics

const FULLY_FEATURED_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!-- This comment is not allowed and will be ignored -->
<!ENTITY nbsp "&#xA0;">
<!ENTITY copyright "Copyright: Public domain.">
<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" xmlns:doc="internal://ns/a">
<!-- Blue triangle SVG graphic -->
  <g id="layer1">
    <path style="fill:#00a6c2;fill-opacity:1;stroke:none"
       d="M 3,58 57,11 42,64 Z" id="triangle" />
    <doc:note>blue triangle</doc:note>
  </g>
</svg>"#;


/// list of tokens for building test input
const TOKENS: [&str; 16] =
[
	"<", ">", "a", "xmlns:", ":", "\"", "=", "&", ";", "-", "!", "/", "<a ", "</a>", "<!--", "-->"
];

/// tests all possible permutations of the selected test tokens for a fixed number of tokens do not
/// result in a panic
#[test]
#[ignore = "This test is very, very slow (and CPU intensive)"]
fn test_no_panic_many_permutations() {
	use kiss_xml;
	let num_tokens = 5;
	let mut all_passed = true;
	for i in 0..(TOKENS.len() as u64).pow(num_tokens) {
		let mut tmp_i = i;
		let mut buffer = String::new();
		for _ in 0..num_tokens {
			let token_index = (tmp_i % TOKENS.len() as u64) as usize;
			buffer.push_str(TOKENS[token_index]);
			tmp_i = tmp_i  / TOKENS.len() as u64;
		}
		let xml = buffer.as_str();
		match std::panic::catch_unwind(|| {let _ = kiss_xml::parse_str(xml);}) {
			Ok(_) => {},
			Err(_) => {
				all_passed = false;
				eprintln!("panic on XML input '{}'", buffer);
			}
		}
	}
	assert!(all_passed);
}

/// tests that bad input does not result in a panic by truncating a string
#[test]
fn test_no_panic_truncate(){
	use kiss_xml;
	let mut all_passed = true;
	for i in 0..FULLY_FEATURED_XML.len() {
		let front_trunc = &FULLY_FEATURED_XML[i..FULLY_FEATURED_XML.len()];
		let rear_trunc = &FULLY_FEATURED_XML[0..i];
		match std::panic::catch_unwind(|| {let _ = kiss_xml::parse_str(front_trunc);}) {
			Ok(_) => {},
			Err(_) => {
				eprintln!("panic on XML input '{}'", front_trunc);
				all_passed = false;
			}
		}
		match std::panic::catch_unwind(|| {let _ = kiss_xml::parse_str(rear_trunc);}) {
			Ok(_) => {},
			Err(_) => {
				eprintln!("panic on XML input '{}'", rear_trunc);
				all_passed = false;
			}
		}
	}
	assert!(all_passed);
}

/// tests that bad input does not result in a panic by deleting chars
#[test]
fn test_no_panic_del_char(){
	use kiss_xml;
	let mut all_passed = true;
	for i in 0..FULLY_FEATURED_XML.len()-1 {
		let front_trunc = &FULLY_FEATURED_XML[0..i];
		let rear_trunc = &FULLY_FEATURED_XML[i + 1..FULLY_FEATURED_XML.len()];
		let mut xml = String::new();
		xml.push_str(front_trunc);
		xml.push_str(rear_trunc);
		match std::panic::catch_unwind(|| { let _ = kiss_xml::parse_str(xml.clone()); }) {
			Ok(_) => {},
			Err(_) => {
				all_passed = false;
				eprintln!("panic on XML input '{}'", xml);
			}
		}
	}
	assert!(all_passed);
}
