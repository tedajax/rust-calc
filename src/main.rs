use exprtree::{ExprNode, ExprTree};

mod exprtree;

fn main() {
	let expr = ExprNode::new("-",
	 						 None, 
	 						 Some(ExprNode::new("*",
	 						 	  Some(ExprNode::new("5", None, None)),
	 						 	  Some(ExprNode::new("2", None, None)))));
	let tree = ExprTree::new(Some(expr));
	println!("{}", tree.eval());
}