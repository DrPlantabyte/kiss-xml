#[test]
fn test_xml_escapes() {
	use kiss_xml;
	let unescaped = r#"&<>'""#;
	let escaped = "&amp;&lt;&gt;&apos;&quot;";
	let escaped_text = r#"&amp;&lt;&gt;'""#;
	let escaped_attribute = "&amp;<>&apos;&quot;";
	assert_eq!(kiss_xml::escape(unescaped), escaped, "Incorrect escaping of XML reserved characters");
	assert_eq!(kiss_xml::unescape(escaped), unescaped, "Incorrect unescaping of XML reserved characters");
	assert_eq!(kiss_xml::text_escape(unescaped), escaped_text, "Incorrect escaping of XML reserved characters");
	assert_eq!(kiss_xml::attribute_escape(unescaped), escaped_attribute, "Incorrect escaping of XML reserved characters");
}

#[test]
fn test_load_from_file() {
	use kiss_xml;
	use tempfile::tempdir;
	use std::fs::File;
	use std::io::{Write};
	// Write sample XML to a file
	let dir = tempdir()?;
	let file_path = dir.path().join("Note.xml");
	let mut tmpfile = File::create(file_path.clone())?;
	write!(tmpfile, r#"<?xml version="1.0" encoding="UTF-8"?>

<!DOCTYPE note [
<!ENTITY ignore "kiss-xml ignores DOCTYPE stuff">
<!ENTITY nbsp "&#xA0;">
<!ENTITY writer "Writer: Donald Duck.">
<!ENTITY copyright "Copyright: W3Schools.">
]>

<note>
	<to>Tove</to>
	<from>Jani</from>
	<heading>Reminder</heading>
	<body>Don't forget <b>me</b> this weekend!</body>
	<footer>&writer;&nbsp;&copyright;</footer>
</note>
"#).unwrap();
	drop(tmpfile); // close the file before re-opening

	// read the sample XML
	let root = kiss_xml::read_from_filepath(file_path.into()).unwrap();

}