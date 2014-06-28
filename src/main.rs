use exprtree::{ExprNode, ExprTree};

mod exprtree;

fn main() {
    println!("{}", ExprTree::build("15+3*6").eval());
}