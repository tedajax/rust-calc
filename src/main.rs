use exprtree::ExprTree;
use std::os;

mod exprtree;

enum Options {
    Verbose,
}

// fn parse_options(args: Vec<String>) -> Vec<Options> {
//     let mut result: Vec<Options> = vec![];

//     for arg in args.iter() {

//     }

// }

fn main() {
    let args = os::args();
    let expression = args.get(1);

    let tree = ExprTree::build(expression.as_slice());
    tree.print();
    println!("{}", tree.eval());
}
