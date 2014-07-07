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

        let value = match from_str::<f64>(token) {
            Some(v) => Some(v),
            None => constant_value(token.as_slice()),
        };
        
        ExprNode {
            token: String::from_str(token),
            value: value,
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
    NoOp,
}

impl OperatorType {
    fn of_operator(operator: &String) -> OperatorType {
        let mut c = ' ';
        for ch in operator.as_slice().chars() {
            c = ch;
            break;
        }

        match TokenType::of_char(c) {
            Alphabetical => Unary,
            _ => match operator.as_slice() {
                "+"|"-"|"*"|"/"|"^" => Binary,
                _ => NoOp,
            }
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

fn constant_value(constant: &str) -> Option<f64> {
    match constant.as_slice() {
        "pi" => Some(Float::pi()),
        "e" => Some(Float::e()),
        _ => None,
    }
}

#[deriving(Show, Clone, PartialEq)]
enum TokenType {
    Numeric,
    Alphabetical,
    Functional,
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

    pub fn of_alphabeticals(s: String) -> TokenType {
        match s.as_slice() {
            "pi" => Numeric,
            _ => Functional,
        }
    }
}

// token type, token string, token precedence
struct Token(TokenType, String, i32);

pub struct ExprTree {
    root: Option<Box<ExprNode>>,
}

fn print_token_list(title: &str, tokens: &Vec<Token>) {
    print!("{}: ", title);
    for t in tokens.iter() {
        let &Token(_, ref ts, _) = t;
        print!("{} ", ts);
    }
    println!("");
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
            let &Token(ttype, ref tstr, _) = token;

            match ttype {
                Numeric => stack.push(ExprNode::new(tstr.as_slice(), None, None)),
                Operator => {
                    let right = stack.pop();
                    let left = stack.pop();
                    stack.push(ExprNode::new(tstr.as_slice(), left, right));
                },
                Functional => {
                    let right = stack.pop();
                    stack.push(ExprNode::new(tstr.as_slice(), None, right));
                }
                _ => {},
            }
        }

        ExprTree::new(Some(stack.get(0).clone()))
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
                Functional => {
                    input_stack.push(Token(ttype, tstr.clone(), tprec))
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

            print_token_list("output", &output_queue);
            print_token_list("input", &input_stack);
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
            print_token_list("output", &output_queue);
            print_token_list("input", &input_stack);
        }
       
        return output_queue;
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
                Alphabetical => {
                    accumulator.push_char(c);
                    let mut j = i + 1;
                    while j < len {
                        let ncopt = expression.chars().nth(j);
                        match ncopt {
                            Some(nc) =>
                                match TokenType::of_char(nc) {
                                    Alphabetical => accumulator.push_char(nc),
                                    _ => break,
                                },
                            _ => {},
                        }
                        j += 1;
                    }

                    let alpha_str = accumulator.clone();
                    let atype = TokenType::of_alphabeticals(alpha_str.clone());
                    result.push(Token(atype, alpha_str, 0));
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

    pub fn eval(&self) -> f64 {
        match self.root {
            None => 0_f64,
            Some(ref node) => ExprTree::eval_node(node),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        match self.root {
            Some(ref node) => ExprTree::print_node(node),
            None => {},
        }
        println!("");
    }

    #[allow(dead_code)]
    fn print_node(node: &Box<ExprNode>) {
        match node.value {
            Some(v) => print!("{}", v),
            None => {
                print!("(");
                match node.left {
                    Some(ref left) => {
                        ExprTree::print_node(left);
                        print!(" ");
                    },
                    None => {},
                }
                print!("{} ", node.token);
                match node.right {
                    Some(ref right) => ExprTree::print_node(right),
                    None => {},
                }
                print!(")");
            }
        }
    }

    fn eval_node(node: &Box<ExprNode>) -> f64 {
        match node.value {
            Some(v) => v,
            None => {
                let ref operator = node.token;
                let ot = OperatorType::of_operator(operator);
                
                match node.right {
                    None => fail!("No available value for operator."),
                    Some(ref right) => {
                        match ot {
                            Unary => {
                                ExprTree::eval_unary(operator,
                                    ExprTree::eval_node(right))
                            },
                            Binary => {
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
            "lg" => value.log2(),
            "log" => value.log10(),
            "sin" => value.sin(),
            "cos" => value.cos(),
            "tan" => value.tan(),
            "csc" => 1_f64 / value.sin(),
            "sec" => 1_f64 / value.cos(),
            "cot" => 1_f64 / value.tan(),
            "neg" => -value,
            "sgn" => value.signum(),
            _ => fail!("Invalid unary operator"),
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