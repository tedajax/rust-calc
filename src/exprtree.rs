use std::str;
use std::string::String;

#[deriving(Clone)]
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
            value: from_str::<f64>(token),
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

#[deriving(PartialEq)]
enum OperatorAssoc {
    LeftAssoc,
    RightAssoc,
}

fn operator_precedence(operator: &String) -> i32 {
    match operator.as_slice() {
        "^" => 4,
        "*"|"/" => 3,
        "+"|"-" => 2,
        _ => 1,
    }
}

fn operator_assoc(operator: &String) -> OperatorAssoc {
    match operator.as_slice() {
        "^" => RightAssoc,
        _ => LeftAssoc,
    }
}

#[deriving(Show, Clone, PartialEq)]
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

// token type, token string, token precedence
struct Token(TokenType, String, i32);

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
        let tokens = ExprTree::parse_tokens(expression);
        let rpn = ExprTree::build_rpn(tokens);
        ExprTree::from_rpn(rpn)
    }

    fn from_rpn(rpn: Vec<Token>) -> ExprTree {
        let mut stack: Vec<ExprNode> = vec![];

        for token in rpn.iter() {
            let &Token(ttype, ref tstr, tprec) = token;

            match ttype {
                Numeric => stack.push(ExprNode::new(tstr.as_slice(), None, None)),
                Operator => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(ExprNode::new(tstr.as_slice(), left, right));
                },
                _ => {},
            }
        }

        ExprTree::new(Some(stack.get(0).clone()))
    }

    fn parse_tokens(expression: &str) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];

        let mut i = 0;
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
                    let op_prec = operator_precedence(&op_str);
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
                    result.push(Token(LeftParen, String::from_str("("), 0));
                },
                RightParen => {
                    result.push(Token(RightParen, String::from_str(")"), 0));
                },
                _ => {},
            }

            i += 1;
        }

        result
    }

    // put the tokens into reverse polish notation
    fn build_rpn(tokens: Vec<Token>) -> Vec<Token> {
        let mut output_queue: Vec<Token> = vec![];
        let mut input_stack: Vec<Token> = vec![];

        for token in tokens.iter() {
            let &Token(ttype, ref tstr, tprec) = token;

            match ttype {
                Numeric => {
                    output_queue.push(Token(ttype, tstr.clone(), tprec))
                },
                Operator => {
                    loop {
                        match input_stack.pop() {
                            None => break,
                            Some(o2) => {
                                let Token(o2type, ref o2str, o2prec) = o2;
                                
                                let assoc = operator_assoc(o2str);

                                if o2type == Operator &&
                                   (assoc == LeftAssoc && tprec <= o2prec ||
                                    tprec < o2prec) {
                                    output_queue.push(Token(o2type, o2str.clone(), o2prec));
                                } else {
                                    input_stack.push(Token(o2type, o2str.clone(), o2prec));
                                    break;
                                }
                            },
                        }
                    }
                    input_stack.push(Token(ttype, tstr.clone(), tprec));
                },
                LeftParen => input_stack.push(Token(ttype, tstr.clone(), tprec)),
                RightParen => {
                    loop {
                        match input_stack.pop() {
                            None => fail!("Parenthesis mismatch!"),
                            Some(o2) => {
                                let Token(o2type, ref o2str, o2prec) = o2;
                                if o2type != LeftParen {
                                    output_queue.push(Token(o2type, o2str.clone(), o2prec));
                                } else {
                                    break;
                                }
                            },
                        }
                    }
                },
                _ => {},
            }
        }

        loop {
            match input_stack.pop() {
                None => break,
                Some(o2) => {
                    let Token(o2type, ref o2str, o2prec) = o2;
                    match o2type {
                        LeftParen|RightParen => fail!("Parenthesis mismatch!"),
                        _ => {
                            output_queue.push(Token(o2type, o2str.clone(), o2prec));
                        },
                    }
                },
            }
        }
       
        return output_queue;
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
            "^" => lhs.powf(rhs),
            _ => 0_f64,
        }
    }
}