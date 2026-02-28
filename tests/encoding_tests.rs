/*!
tests for handling of non-UTF8 XML
*/
use encoding_rs::

fn utf8_xml_bytes() -> &'static [u8] {
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root author="some dude">
	<!--comment-->
	<mydata>
		<desc>This is my data</desc>
		<properties>
			<property name="a" value="1"/>
			<property name="b" value="2"/>
		</properties>
		<charset>0123456789-ABCDEFGHIJKLMNOPQRSTUVWXYZ-abcdefghijklmnopqrstuvwxyz-üéâäàåêëèïîì-±≥≤÷≈°²</charset>
		<meta>My metadata goes here</meta>
		<other/>
		<other/>
	</mydata>
</root>
"#;
	xml.as_bytes()
}

fn cp437_xml_bytes() -> &'static [u8] {
	let xml = r#"<?xml version="1.0" encoding='CP437'?>
<root author="some dude">
	<!--comment-->
	<mydata>
		<desc>This is my data</desc>
		<properties>
			<property name="a" value="1"/>
			<property name="b" value="2"/>
		</properties>
		<charset>0123456789-ABCDEFGHIJKLMNOPQRSTUVWXYZ-abcdefghijklmnopqrstuvwxyz-üéâäàåêëèïîì-±≥≤÷≈°²</charset>
		<meta>My metadata goes here</meta>
		<other/>
		<other/>
	</mydata>
</root>
"#;

}
