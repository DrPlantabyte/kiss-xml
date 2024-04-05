//#![deny(unused_must_use)]
//#![deny(missing_docs)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::slice::Iter;

pub struct Document {
	// TODO
}

impl Document {
	pub fn doctype_defs(&self) -> Iter<DTD> {
		todo!()
	}
}

impl Document {
	pub fn declaration(&self) -> Option<&Declaration> {
		todo!()
	}

	pub fn to_string(&self, indent: &str) -> String {
		todo!()
	}
}

impl Document {
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

pub enum Node {
	Element(Element),
	Text(Text),
	Comment(Comment)
}

impl Node {

	pub fn name(&self) -> String {
		todo!()
	}

	pub fn text(&self) -> String {
		todo!()
	}

	pub fn is_element(&self) -> bool {
		todo!()
	}

	pub fn is_text(&self) -> bool {
		todo!()
	}

	pub fn is_comment(&self) -> bool {
		todo!()
	}
}

impl std::fmt::Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::fmt::Debug for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl PartialEq<Self> for Node {
	fn eq(&self, other: &Self) -> bool {
		todo!()
	}
}

impl PartialOrd for Node{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		todo!()
	}
}

impl Hash for Node {
	fn hash<H: Hasher>(&self, state: &mut H) {
		todo!()
	}
}

#[derive(Clone)]
pub struct Element{
	// TODO
}

impl Element {

	pub fn new(name: &str) -> Self {
		todo!()
	}

	pub fn new_with_attributes(name: &str, attributes: HashMap<String, String>) -> Self {
		todo!()
	}

	pub fn child_elements(&self) -> Iter<Element>{
		todo!()
	}

	pub fn children(&self) -> Iter<Node>{
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

	pub fn get_attr(&self, attr_name: &str) -> Option<String> {
		todo!()
	}

	pub fn set_attr(&mut self, attr_name: &str, value: &str) {
		todo!()
	}

	pub fn search<P>(&self, predicate: P) -> Iter<Node> where P: FnMut(&Node) -> bool {
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

	pub fn append(&mut self, node: Node) {
		todo!()
	}

	pub fn insert(&mut self, index: usize, node: Node) {
		todo!()
	}

	pub fn remove(&mut self, index: usize) -> Option<Node> {
		todo!()
	}

	pub fn remove_all<P>(&mut self, predicate: P) -> usize where P: FnMut(&Node) -> bool {
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

#[derive(Clone)]
pub struct Text{
	// TODO
}

impl Text {
	pub fn new(text: &str) -> Self {
		todo!()
	}
}

#[derive(Clone)]
pub struct Comment{
	// TODO
}

impl Comment {
	pub fn new(comment: &str) -> Self {
		todo!()
	}
}

pub struct Declaration {
	// TODO
}
pub struct DTD {
	// TODO
}
