use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;
/**
 this module contains utilities exclusive to parsing
 */

use crate::dom::*;
use crate::errors::*;

/** special tree data structure for parsing which uses ID's as keys in a HashMap to work around limitations in Rust's lifetime syntax. It is used like a stack, though internally it uses a HashMap based data arena */
#[derive(Debug, Default)]
pub struct ParseTree{
	/** hold data in a non-tree format because Rust's lifetime syntax doesn't let you retrieve the lifetime from the parent of a node at runtime (lifetimes exist only at compile time, and even then there is no syntax for separately disentanlging reference-lifetimes from data-lifetimes) */
	data: HashMap<usize, ParseTreeNode>,
	/// current tip of the parsing tree, pointing to the "top" of the stack
	pos: Option<usize>
}

impl ParseTree {
	/// new parser tree
	pub fn new() -> Self {
		Self::default()
	}
	/// push a new element to the stack
	pub fn push(&mut self, new_element: Element) {
		println!("pushing: <{}>", new_element.tag_name());
		if self.pos.is_none() {
			self.data.insert(0, ParseTreeNode{
				id: 0,
				value: Box::new(new_element),
				parent_id: None,
				child_ids: Vec::new(),
			});
			self.pos = Some(0);
		} else {
			let old_pos = self.pos.unwrap();
			let new_pos = self.data.len();
			self.data.get_mut(&old_pos).expect("logic error")
				.child_ids.push(new_pos);
			self.data.insert(new_pos, ParseTreeNode{
				id: new_pos,
				value: Box::new(new_element),
				parent_id: Some(old_pos),
				child_ids: Vec::new(),
			});
			self.pos = Some(new_pos);
		}
	}
	/// pop the top element from the stack
	pub fn pop(&mut self) -> Result<(), KissXmlError> {
		if self.pos.is_none() {
			return Err(ParsingError::new("closing tag without corresponding open tag").into());
		}
		let pos = self.pos.unwrap();
		println!("popping: <{}>", self.data
			.get(&pos).expect("logic error").value.as_element().unwrap().tag_name());
		let new_pos = self.data
			.get(&pos).expect("logic error").parent_id;
		self.pos = new_pos;
		Ok(())
	}
	/// append a node to the top element on the stack (without adding the new node to the stack)
	pub fn append(&mut self, n: impl Node + 'static) -> Result<(), KissXmlError> {
		println!("appending: {:?}", n);
		if self.pos.is_none() {
			return Err(ParsingError::new("no root element").into());
		}
		let pos = self.pos.unwrap();
		let new_id = self.data.len();
		self.data.get_mut(&pos).expect("logic error")
			.child_ids.push(new_id);
		self.data.insert(new_id, ParseTreeNode{
			id: new_id,
			value: Box::new(n),
			parent_id: Some(pos),
			child_ids: Vec::new(),
		});
		Ok(())
	}
	/// reference to the current element on top of the stack
	pub fn top_element(&self) -> Option<&Element> {
		match self.pos {
			None => None,
			Some(pos) => Some(
				self.data.get(&pos).expect("logic error")
					.value.as_element().expect("logic error")
			)
		}
	}
	/// converts the whole parse tree to a DOM, returning the root element
	pub fn to_dom(mut self) -> Result<Element, KissXmlError> {
		if self.pos.is_none() || self.data.is_empty() {
			return Err(ParsingError::new("no root element").into());
		}
		// depth-first DOM construction
		// the stack-based API ensures children always have higher ID number
		// than their parents
		// the challenge is that reverse index iteration will add children
		// in opposite order
		// solution is to flip the children when removing from the map
		// (index 0 is root, so not included in loop)
		for i in (1..self.data.len()).rev() {
			let mut node = self.data.remove(&i).expect("logic error: missing index");
			if node.value.is_element() {
				// flip the children of element
				node.value.as_element_mut().unwrap().reverse_children()
			}
			let parent_index = node.parent_id.expect("logic error: non-root node with no parent");
			self.data.get_mut(&parent_index).expect("logic error: parent is missing")
				.value.as_element_mut().expect("logic error: parent is not an Element")
				.append_boxed(node.value);
		}
		let root_node = self.data.remove(&0).expect("logic error: no root element");
		let mut root = root_node.destruct();
		// let mut e = **(root.as_any()
		// 	.downcast_ref::<Box<Element>>()
		// 	.expect("logic error: root is not an element"));
		let e = root.as_element_mut().expect("logic error: root is not an element");
		// flip children because they were added in reverse order
		e.reverse_children();
		// done
		// use mem::take to do a "DerefMove" operation
		return Ok(std::mem::take(e));
	}
}

/** nodes in the parser tree */
#[derive(Debug)]
pub struct ParseTreeNode{
	/// unique ID
	id: usize,
	/// DOM node
	value: Box<dyn Node>,
	/// parent element of this DOM node
	parent_id: Option<usize>,
	/// children of this DOM element
	child_ids: Vec<usize>
}

impl ParseTreeNode {
	/// used to move the value out of the struct
	fn destruct(self) -> Box<dyn Node>{self.value}
}

impl PartialEq for ParseTreeNode {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for ParseTreeNode{}
impl std::hash::Hash for ParseTreeNode {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state)
	}
}
impl PartialOrd for ParseTreeNode {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.id.cmp(&other.id))
	}
}
impl Ord for ParseTreeNode{
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}