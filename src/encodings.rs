/*!
This module contains utilities for parsing non-UTF8 text. It wraps standard encoders from the
encoding_rs crate as well as providing custom encoders for special cases (eg IBM437)
*/

use std::collections::HashSet;
use std::io::{Read, Write};

pub trait CharacterEncoder {
	fn get_names() -> HashSet<String>;
	fn decode_bytes(&self, data: &[u8]) -> String;
	fn encode_str(&self, text: &str) -> Vec<u8>;
	fn stream_decoder(&self, src: impl Read) -> impl Read;
	fn stream_encoder(&self, src: impl Write) -> impl Write;
}

fn abc(){
	use std::fs; use std::io::*;
	let mut f = std::fs::File::open("myfile")?;
	let mut buf = Vec::new();
	f.read_to_end(&mut buf)?;
	println!("read: {}", String::from_utf8(buf)?);
	
}

