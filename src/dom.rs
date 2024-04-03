use std::rc::Rc;

pub struct Document {
	// TODO
}

pub enum Node {
	Element {
		// TODO
	},
	Text {
		content: String
	},
	Comment {
		content: String
	}
}

pub struct Declaration {
	content: String
}
pub struct DTD {
	content: String
}
