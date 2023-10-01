use super::{lexer::*, ast::Expression};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn parse_str(expression: &str) -> Parser {
        let mut expression = expression.split_ascii_whitespace();
        let mut tokens : Vec<Token> = Vec::new();
        loop {
            let token = Lexer::next_token(&mut expression);

            if let Token::EOF = token {
                break;
            } else if let Token::InvalidToken(err) = token {
                panic!("Invalid token found: {err}");
            }

            tokens.push(token); 
        } 

        //Simple check to verify valid infix notation
        //since user could pass postfix notation and it would be valid

        let mut is_operand = false;
        for token in tokens.iter() {
            if let Token::Number(_) = token {
                if is_operand {
                    panic!("No operator between operands!");
                }
            } else {
                if !is_operand {
                    panic!("No operand after operator!");
                }
            } 

            is_operand = !is_operand
        }

        Parser {
            tokens
        }
    } 

    pub fn get_expression(&mut self) -> Box<Expression> {
 
        let mut output : Vec<&Token> = Vec::new();
        let mut stack : Vec<&Token> = Vec::new();

        for token in self.tokens.iter() {
            if let Token::Number(_) = token {
                output.push(token);
            } else if let Token::InvalidToken(_) | Token::EOF = token {
                panic!("Error parsing expression");
            } else {
                while !stack.is_empty() {
                    let last_op = stack.last();
                    match last_op {
                        Some(op) => {
                            let precedence = token.get_operator_precedence();
                            if op.get_operator_precedence() >= precedence {
                                output.push(stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        None => {
                            panic!("No operator found!");
                        }
                    }
                }
                stack.push(token);
            }
        }            

        while !stack.is_empty() {
            output.push(stack.pop().unwrap());
        }

        let mut expressions : Vec<Box<Expression>> = Vec::new();

        for token in output.iter() {
            if let Token::Number(num) = token {
                expressions.push(Expression::num(*num));
            } else if let Token::InvalidToken(_) | Token::EOF = token {
                panic!("Error parsing expression!");
            } else {
                match token {
                    Token::Plus(_) => {
                        let e1 = expressions.pop().unwrap(); 
                        let e2 = expressions.pop().unwrap();
                        expressions.push(Expression::add(e2, e1));
                    } 
                    Token::Minus(_) => {
                        let e1 = expressions.pop().unwrap(); 
                        let e2 = expressions.pop().unwrap();
                        expressions.push(Expression::sub(e2, e1));
                    }
                    Token::Mul(_) => {
                        let e1 = expressions.pop().unwrap(); 
                        let e2 = expressions.pop().unwrap();
                        expressions.push(Expression::mul(e2, e1));
                    }
                    Token::Div(_) => {
                        let e1 = expressions.pop().unwrap(); 
                        let e2 = expressions.pop().unwrap();
                        expressions.push(Expression::div(e2, e1));
                    }
                    Token::Pow(_) => {
                        let e1 = expressions.pop().unwrap(); 
                        let e2 = expressions.pop().unwrap();
                        expressions.push(Expression::pow(e2, e1));
                    }
                    _ => ()
                } 
            }
        }

        /*
            Debug:

            println!("{:?}", output);
            println!("{:?}", final_expr);
        */

        match expressions.pop() {
            Some(expr) => expr,
            None => panic!("Unable to complete expression!")
        }
    }
 

    pub fn print_debug(&self) {
        println!("{:?}", self.tokens);
    }
}
