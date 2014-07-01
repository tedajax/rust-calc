use exprtree::{ExprNode, ExprTree};

mod exprtree;

fn main() {
    println!("{}", ExprTree::build("20.5*2").eval());
}