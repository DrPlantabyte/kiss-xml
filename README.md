# KISS-XML: Keep It Super Simple XML
![GitHub Workflow Build Status](https://github.com/DrPlantabyte/kiss-xml/actions/workflows/build-main.yml/badge.svg) ![GitHub Workflow Test Status](https://github.com/DrPlantabyte/kiss-xml/actions/workflows/unit-test-main.yml/badge.svg) [![codecov](https://codecov.io/gh/DrPlantabyte/kiss-xml/branch/main/graph/badge.svg?token=SA5UFPQG7A)](https://codecov.io/gh/DrPlantabyte/kiss-xml) [![Crate.io](https://img.shields.io/crates/v/kiss-xml)](https://crates.io/crates/kiss-xml) [![Redistribution license](https://img.shields.io/github/license/DrPlantabyte/kiss-xml?color=green)](https://github.com/DrPlantabyte/kiss-xml/blob/main/kiss-xml/LICENSE)

This Rust library provides an easy-to-use Document Object Model (DOM) for 
reading and writing XML files. Unlike many other XML parsers, KISS-XML simply
parses the given XML to a full DOM, which you can then modify and serialize back
to XML. No schemas or looping required.

## What's included:
KISS-XML provides the basics for XML documents, including:
* Parse XML files and strings to a DOM
* XML elements, text, and comments
* DOM is mutable and can be saved as a string and to files
* Easy to use

## What's NOT included:
* Namespace support
* Schema handling
* CDATA
* Document type declarations (DTDs will be preserved but not interpreted)
* Parsing character encodings other than UTF-8
* Typed XML data (eg integer attribute values)
* Performance optimizations (prioritizing easy-to-use over fast)

If you need any of the above XML features, then this library is too simple for
your needs. Try another XML parsing crate instead.

## Quickstart Guide
First, add the following to your Cargo.toml file:
```text
kiss_xml = "1"
```

Then to parse an XML file, all you need to do is call the
`kiss_xml::parse_filepath(...)` function, like this:

```rust
fn main() {
	let doc = kiss_xml::parse_filepath("my-file.xml").unwrap();
    println!("{}", doc.to_string());
}
```

TODO

## More Examples
TODO

