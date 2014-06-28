use std::str;
use std::string::String;

#[deriving(Clone)]
pub struct ExprNode {
    token: String,
    value: Option<f64>,
    precedence: i32,
    left: Option<Box<ExprNode>>,
    right: Option<Box<ExprNode>>,
}

impl ExprNode {
    pub fn new(token: &str,
        left: Option<ExprNode>,
        right: Option<ExprNode>) -> ExprNode {

        let token = String::from_str(token);
        let precedence: i32;
        let value = from_str::<f64>(token.as_slice());
        match value {
            None => precedence = operator_precedence(&token),
            Some(ref v) => precedence = 0,
        }

        ExprNode {
            token: token,
            value: value,
            precedence: precedence,
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

    pub fn is_operator(&self) -> bool {
        match self.value {
            None => true,
            Some(v) => false,
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

fn operator_precedence(operator: &String) -> i32 {
    match operator.as_slice() {
        "^" => 4,
        "*"|"/" => 3,
        "+"|"-" => 2,
        _ => 1,
    }
}

#[deriving(Show)]
enum TokenType {
    Numeric,
    Alphabetical,
    Operator,
    LeftParen,
    RightParen,
    Invalid,
}

impl TokenType {
    pub fn of_char(c: char) -> TokenType {
        if "0123456789.".contains_char(c) {
            Numeric
        } else if "abcdefghijklmnopqrstuvwxyz".contains_char(c) {
            Alphabetical
        } else if "+-*/%^".contains_char(c) {
            Operator
        } else if c == '(' {
            LeftParen
        } else if c == ')' {
            RightParen
        } else {
            Invalid
        }
    }
}

struct Token(TokenType, String, i32);

static MAX_PRECEDENCE: i32 = 5;

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

    pub fn build(expression: &str) -> ExprTree {
        let mut nodes: Vec<ExprNode> = vec![];

        let tokens = ExprTree::parse_tokens(expression);

        for token in tokens.iter() {
            let &Token(ref ttype, ref expr, ref prec) = token;
            nodes.push(ExprNode::new(expr.as_slice(), None, None));
        }

        let mut used_nodes_vec: Vec<bool> = vec![];
        for n in nodes.iter() {
            used_nodes_vec.push(false);
        }

        let mut used_nodes: &mut [bool] = used_nodes_vec.as_mut_slice();

        let tree_root: ExprNode;
        let mut top_index: uint = 0;

        loop {
            //find highest priority node
            let len = nodes.len();
            let mut max_prec = 0;
            for i in range(0, len) {
                let n = nodes.get(i);
                if n.precedence > max_prec && !used_nodes[i] {
                    max_prec = n.precedence;
                    top_index = i;
                }
            }

            used_nodes[top_index] = true;

            if max_prec == 0 {
                tree_root = nodes.get(top_index).clone();
                break; 
            }

            let left_index: uint;
            if top_index == 0 {
                left_index = 0;
            } else {
                left_index = top_index - 1;
            }

            let right_index = top_index + 1;

            let left = nodes.get(left_index).clone();
            let right = nodes.get(right_index).clone();

            let top = nodes.get_mut(top_index);

            if !left.is_operator() {
                top.left = Some(box left);
            }

            top.right = Some(box right);
        }

        ExprTree::new(Some(tree_root))
    }

    fn parse_tokens(expression: &str) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];

        let mut i = 0;
        let mut prec_mult = 0;
        let mut accumulator = String::new();
        let len = expression.len();
        while i < len {
            let copt = expression.chars().nth(i);
            let c = match copt {
                None => ' ',
                Some(ch) => ch,
            };

            let token_type = TokenType::of_char(c);
            match token_type {
                Operator => {
                    let op_str = str::from_char(c);
                    let mut op_prec = operator_precedence(&op_str);
                    op_prec += prec_mult * MAX_PRECEDENCE;
                    result.push(Token(token_type, op_str, op_prec));
                },
                Numeric => {
                    accumulator.push_char(c);
                    let mut j = i + 1;
                    while j < len {
                        let ncopt = expression.chars().nth(j);
                        match ncopt {
                            Some(nc) =>
                                match TokenType::of_char(nc) {
                                    Numeric => accumulator.push_char(nc),
                                    _ => break,
                                },
                            _ => {},
                        }
                        j += 1;
                    }

                    let num_str = accumulator.clone();
                    result.push(Token(token_type, num_str, 0));
                    accumulator.truncate(0);
                    i = j - 1;
                },
                LeftParen => {
                    prec_mult += 1;
                    //result.push(Token(token_type, String::from_str("("), 0));
                },
                RightParen => {
                    prec_mult -= 1;
                    //result.push(Token(token_type, String::from_str(")"), 0));
                    if prec_mult < 0 {
                        fail!("Parenthesis mismatch: \
                               more closing parenthesis than opening.");
                    }
                },
                _ => {},
            }

            i += 1;
        }

        if prec_mult > 0 {
            fail!("Parenthesis mismatch: \
                   more opening parenthesis than closing.");
        }

        result
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