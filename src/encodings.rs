/*!
This module contains code for handling non-UTF-8 character encodings.
Only a specific subset of common encodings are supported
*/

pub enum CharacterEncoding {
	UTF8,
	UTF16LE,
	UTF16BE,
	ISO8859_1, // aka latin-1
	CP1252,    // aka Windows-1252
	IBM437,    // aka CP-437
}
