#![deny(unused_must_use)]
#![deny(missing_docs)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::slice::Iter;

pub struct Document {
	// TODO
}

impl Document {

	pub fn new(root: Element) -> Self {
		todo!()
	}

	pub fn doctype_defs(&self) -> Iter<DTD> {
		todo!()
	}

	pub fn declaration(&self) -> Option<&Declaration> {
		todo!()
	}

	pub fn to_string(&self) -> String {
		self.to_string_with_indent("  ")
	}

	pub fn to_string_with_indent(&self, indent: &str) -> String {
		todo!()
	}

	pub fn write_to_filepath(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
		self.write_to_filepath_with_indent(path, "  ")
	}
	pub fn write_to_filepath_with_indent(&self, path: impl AsRef<Path>, indent: &str) -> std::io::Result<()> {
		todo!()
	}
	pub fn write_to_file(&self, file: &File, indent: &str) -> std::io::Result<()> {
		self.write_to_file_with_indent(file, "  ")
	}
	pub fn write_to_file_with_indent(&self, file: &File, indent: &str) -> std::io::Result<()> {
		todo!()
	}
	pub fn root_element(&self) -> &Element {
		todo!()
	}
	pub fn root_element_mut(&mut self) -> &mut Element {
		todo!()
	}
}

impl std::fmt::Display for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Document{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl PartialEq<Self> for Document {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

pub trait Node: dyn_clone::DynClone + std::fmt::Debug + std::fmt::Display {

	fn name(&self) -> String;

	fn text(&self) -> Option<String>;

	fn is_element(&self) -> bool;

	fn is_text(&self) -> bool;

	fn is_comment(&self) -> bool;
}

#[derive(Clone)]
pub struct Element {
	// TODO
}

impl Element {

	pub fn new(name: &str) -> Self {
		todo!()
	}

	pub fn new_with_attributes(name: &str, attributes: HashMap<&str, &str>) -> Self {
		todo!()
	}

	pub fn new_with_text(name: &str, text: impl Into<String>) -> Self {
		todo!()
	}

	pub fn new_with_attributes_and_text(name: &str, attributes: HashMap<&str, &str>, text: impl Into<String>) -> Self {
		todo!()
	}

	pub fn child_elements(&self) -> Iter<Element>{
		todo!()
	}

	pub fn children(&self) -> Iter<Box<dyn Node>>{
		todo!()
	}

	pub fn first_element_by_name(&self, name: &str) -> Option<&Element> {
		todo!()
	}

	pub fn first_element_by_name_mut(&mut self, name: &str) -> Option<&mut Element> {
		todo!()
	}

	pub fn elements_by_name(&self, name: &str) -> Iter<Element>{
		todo!()
	}

	pub fn attributes(&self) -> &HashMap<&str, &str> {
		todo!()
	}

	pub fn get_attr(&self, attr_name: &str) -> Option<String> {
		todo!()
	}

	pub fn set_attr(&mut self, attr_name: &str, value: &str) {
		todo!()
	}

	pub fn remove_attr(&mut self, attr_name: &str) -> Option<String> {
		todo!()
	}

	pub fn search<P>(&self, predicate: P) -> Iter<Box<dyn Node>> where P: FnMut(&Box<dyn Node>) -> bool {
		// recursive
		todo!()
	}

	pub fn search_elements<P>(&self, predicate: P) -> Iter<Element> where P: FnMut(&Element) -> bool {
		// recursive
		todo!()
	}

	pub fn search_elements_by_name(&self, name: &str) -> Iter<Element>{
		// recursive
		todo!()
	}

	pub fn search_text<P>(&self, predicate: P) -> Iter<Text> where P: FnMut(&Text) -> bool {
		// recursive
		todo!()
	}

	pub fn search_comments<P>(&self, predicate: P) -> Iter<Comment> where P: FnMut(&Comment) -> bool {
		// recursive
		todo!()
	}

	pub fn append(&mut self, node: impl Node) {
		todo!()
	}

	pub fn insert(&mut self, index: usize, node: impl Node) {
		todo!()
	}

	pub fn remove(&mut self, index: usize) -> Option<Box<dyn Node>> {
		todo!()
	}

	pub fn remove_all<P>(&mut self, predicate: P) -> usize where P: FnMut(&dyn Node) -> bool {
		// recursive, returns count
		todo!()
	}

	pub fn remove_all_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// recursive, returns count
		todo!()
	}

	pub fn remove_element(&mut self, index: usize) -> Option<Element> {
		todo!()
	}

	pub fn remove_elements<P>(&mut self, predicate: P) -> usize where P: FnMut(&Element) -> bool {
		// returns count
		todo!()
	}

	pub fn remove_elements_by_name(&mut self, name: &str) -> usize {
		// returns count
		todo!()
	}

}

impl Node for Element{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Element {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Element {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Element {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}


impl std::fmt::Display for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Element {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone)]
pub struct Text {
	pub content: String
}

impl Text {
	pub fn new(text: &str) -> Self {
		todo!()
	}

	pub fn name(&self) -> String {
		todo!()
	}
}

impl Node for Text{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Text {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Text {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Text {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}


impl std::fmt::Display for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Text {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone)]
pub struct Comment{
	pub content: String
}

impl Comment {
	pub fn new(comment: &str) -> Self {
		todo!()
	}

}

impl Node for Comment{
	fn name(&self) -> String {
		todo!()
	}

	fn text(&self) -> Option<String> {
		todo!()
	}

	fn is_element(&self) -> bool {
		todo!()
	}

	fn is_text(&self) -> bool {
		todo!()
	}

	fn is_comment(&self) -> bool {
		todo!()
	}
}

impl PartialOrd for Comment {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl PartialEq<Self> for Comment {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl Hash for Comment {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}

impl std::fmt::Display for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Comment {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Declaration {
	// TODO
}

impl std::fmt::Display for Declaration {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct DTD {
	// TODO
}

impl std::fmt::Display for DTD {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
