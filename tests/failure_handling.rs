/// check that bad XML is identified as such
#[test]
fn test_unclosed_root() {
	use kiss_xml;
	assert!(kiss_xml::parse_str(
		r#"<?xml version="1.0" encoding="UTF-8"?>
<config>
	<name>My Settings</name>
	<sound>
		<property name="volume" value="11" />
		<property name="mixer" value="standard" />
	</sound>
<config>
"#
	).is_err(),
	"Should have errored due to unclosed root element"
	)
}