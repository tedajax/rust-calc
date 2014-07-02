use exprtree::ExprTree;
use std::os;

mod exprtree;

fn main() {
    let args = os::args();
    let expression = args.get(1);

    let tree = ExprTree::build(expression.as_slice());
    println!("{}", tree.eval());
}
