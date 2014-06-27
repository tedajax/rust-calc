use std::string::String;

pub struct ExprNode {
    token: String,
    value: Option<f64>,
    left: Option<Box<ExprNode>>,
    right: Option<Box<ExprNode>>,
}

impl ExprNode {
    pub fn new(token: &str,
        left: Option<ExprNode>,
        right: Option<ExprNode>) -> ExprNode {

        ExprNode {
            token: String::from_str(token),
            value: from_str::<f64>(token.as_slice()),
            left: match left {
                None => None,
                Some(l) => Some(box l),
            },
            right: match right {
                None => None,
                Some(r) => Some(box r),
            },
        }
    }
}

enum OperatorType {
    Unary,
    Binary,
    Both,
    NoOp,
}

impl OperatorType {
    fn of_operator(operator: &String) -> OperatorType {
        match operator.as_slice() {
            "+"|"*"|"/"|"^" => Binary,
            "ln" => Unary,
            "-" => Both,
            _ => NoOp,
        }
    }
}

pub struct ExprTree {
    root: Option<Box<ExprNode>>,
}

impl ExprTree {
    pub fn new(root: Option<ExprNode>) -> ExprTree {
        ExprTree {
            root: match root {
                None => None,
                Some(r) => Some(box r),
            },
        }
    }

    pub fn build(expression: &str) {
        
    }

    pub fn eval(&self) -> f64 {
        match self.root {
            None => 0_f64,
            Some(ref node) => ExprTree::eval_node(node),
        }
    }

    fn eval_node(node: &Box<ExprNode>) -> f64 {
        match node.value {
            Some(v) => v,
            None => {
                let ref operator = node.token;
                let ot = OperatorType::of_operator(operator);
                
                match node.right {
                    None => 0_f64,
                    Some(ref right) => {
                        match ot {
                            Unary => {
                                ExprTree::eval_unary(operator,
                                    ExprTree::eval_node(right))
                            },
                            Binary => {
                                match node.left {
                                    None => 0_f64,
                                    Some(ref left) => 
                                        ExprTree::eval_binary(operator,
                                        ExprTree::eval_node(left),
                                        ExprTree::eval_node(right))
                                }
                            },
                            Both => {
                                match node.left {
                                    None => ExprTree::eval_unary(operator,
                                        ExprTree::eval_node(right)),
                                    Some(ref left) => 
                                        ExprTree::eval_binary(operator,
                                        ExprTree::eval_node(left),
                                        ExprTree::eval_node(right))
                                }   
                            },
                            _ => 0_f64,
                        }
                    }
                }
            },
        }
    }

    fn eval_unary(operator: &String, value: f64) -> f64 {
        match operator.as_slice() {
            "-" => -value,
            "ln" => value.ln(),
            _ => 0_f64,
        }
    }

    fn eval_binary(operator: &String, lhs: f64, rhs: f64) -> f64 {
        match operator.as_slice() {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => 0_f64,
        }
    }
}