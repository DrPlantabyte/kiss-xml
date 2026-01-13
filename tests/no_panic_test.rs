//! Tests to ensure that bad input causes errors instead of panics

/// list of tokens for building test input
const TOKENS: [&str; 16] =
[
	"<", ">", "a", "xmlns:", ":", "\"", "=", "&", ";", "-", "!", "/", "<a ", "</a>", "<!--", "-->"
];

/// tests all possible permutations of the selected test tokens for a fixed number of tokens
#[test]
fn test_no_panic_many_permutations() {
	use kiss_xml;
	let num_tokens = 5;
	let mut all_passed = true;
	for i in 0..TOKENS.len().pow(num_tokens) {
		let mut tmp_i = i;
		let mut buffer = String::new();
		for _ in 0..num_tokens {
			let token_index = tmp_i % TOKENS.len();
			buffer.push_str(TOKENS[token_index]);
			tmp_i = tmp_i  / TOKENS.len();
		}
		let xml = buffer.as_str();
		let catch_result = std::panic::catch_unwind(|| {let _ = kiss_xml::parse_str(xml);});
		match  catch_result {
			Ok(_) => {},
			Err(_) => {
				all_passed = false;
				eprintln!("panic on XML input '{}'", buffer);
			}
		}
		assert!(all_passed);
	}
}