#[test]
fn mock_test() {
	use kiss_xml::mock;
	assert_eq!(mock(), 42, "is not 42");
}
