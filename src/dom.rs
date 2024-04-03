use std::rc::Rc;

pub struct Document {

	// implements a graph the Rust way, which is to not use a graph at all
	// (see https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
	/// A flat array of all nodes in the document
	all_nodes: Vec<Node>,
	/// A flat array of all 1-to-1 relationships
	all_relations: Vec<ParentChild>,

}

pub enum Node {
	Element {
		doc: Rc<Document>,
		uuid: usize
	},
	Text {
		content: String
	},
	Comment {
		content: String
	}
}

impl Node::Element {
	pub fn get_children_by_name(&self, name: &str) -> Vec<>
}


pub struct Declaration {
	content: String
}
pub struct DTD {
	content: String
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ParentChild {
	parent_index: usize,
	child_index: usize
}
