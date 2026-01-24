//! additional tests for correct handling of XML escapes and special characters
use kiss_xml;
use kiss_xml::dom::Node;

/// content escape handling test: interpret all escapes and apply most escapes
/// (all but ' and ") on re-serialization
#[test]
fn test_content_escapes(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>&lt;&gt;&amp;&quot;&apos;</root>
"#;
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	assert_eq!(dom.root_element().text().as_str(), "<>&\"'");
	assert_eq!(dom.to_string().as_str(),r#"<?xml version="1.0" encoding="UTF-8"?>
<root>&lt;&gt;&amp;"'</root>
"#);
}

/// attribute escape handling test: interpret all escapes and apply all escapes
/// on re-serialization (technically, only <, &, and " need to be escaped, but
/// we've decided to do all in attributes for consistency)
#[test]
fn test_attribute_escapes(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root a="&lt;&gt;&amp;&quot;&apos;"/>
"#;
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	assert_eq!(dom.root_element().get_attr("a").unwrap().as_str(), "<>&\"'");
	assert_eq!(dom.to_string().as_str(), xml);
}

/// comments should not be escaped
#[test]
fn test_comment_escapes(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root><!-- &lt;&gt;&amp;&quot;&apos; --></root>
"#;
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	let comment = dom.root_element().children().next().unwrap().as_comment().unwrap();
	assert_eq!(comment.get_content(), " &lt;&gt;&amp;&quot;&apos; ");
	assert_eq!(dom.to_string().as_str(), xml);
}

/// CData should not be escaped
#[test]
fn test_cdata_escapes(){
	let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<root><![CDATA[<greeting>&lt;&gt;&amp;&quot;&apos;</greeting>]]></root>
"#;
	let dom = kiss_xml::parse_str(xml).expect("Error parsing XML");
	let cdata = dom.root_element().children().next().unwrap().as_cdata().unwrap();
	assert_eq!(cdata.get_content(), "<greeting>&lt;&gt;&amp;&quot;&apos;</greeting>");
	assert_eq!(dom.to_string().as_str(), xml);
}
