use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hasher;
/**
 this module contains utilities exclusive to parsing
 */

use crate::dom::*;

/** special tree data structure for parsing which uses ID's as keys in a HashMap to work around limitations in Rust's lifetime syntax */
pub struct ParseTree{
	/** hold data in a non-tree format because Rust's lifetime syntax doesn't let you retrieve the lifetime from the parent of a node at runtime (lifetimes exist only at compile time, and even then there is no syntax for separately disentanlging reference-lifetimes from data-lifetimes) */
	data: HashMap<usize, ParseTreeNode>,
	/// current tip of the parsing tree
	pos: usize
}

impl ParseTree {
	pub fn new() -> Self{
		todo!()
	}
}

/** nodes in the parser tree */
#[derive(Clone, Debug, Eq, Ord)]
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

impl PartialEq for ParseTreeNode{
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl std::hash::Hash for ParseTreeNode{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state)
	}
}
impl PartialOrd for ParseTreeNode{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.id.partial_cmp(&other.id)
	}
}