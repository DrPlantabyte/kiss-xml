/*!
This module contains code for handling non-UTF-8 character encodings.
Only a specific subset of common encodings are supported
*/

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum CharacterEncoding {
	UTF8,
	UTF16LE,
	UTF16BE,
	UTF32LE,
	UTF32BE,
	ISO8859_1, // aka latin-1
	CP1252,    // aka Windows-1252
	IBM437,    // aka CP-437
}

impl CharacterEncoding {
	/// mapping of UTF byte-order-marks
	pub(crate) fn bom_table() -> HashMap<CharacterEncoding, &'static [u8]> {
		let utf8_bom = &[0xEFu8, 0xBBu8, 0xBFu8];
		let utf16le_bom = &[0xFFu8, 0xFEu8];
		let utf16be_bom = &[0xFEu8, 0xFFu8];
		let utf32le_bom = &[0xFFu8, 0xFEu8, 0x00, 0x00];
		let utf32be_bom = &[0x00, 0x00, 0xFEu8, 0xFFu8];
		HashMap::from([
			(CharacterEncoding::UTF8, &utf8_bom[..]),
			(CharacterEncoding::UTF16LE, &utf16le_bom[..]),
			(CharacterEncoding::UTF16BE, &utf16be_bom[..]),
			(CharacterEncoding::UTF32LE, &utf32le_bom[..]),
			(CharacterEncoding::UTF32BE, &utf32be_bom[..]),
		])
	}
}
