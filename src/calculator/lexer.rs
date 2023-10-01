use core::str::SplitAsciiWhitespace;

#[derive(std::fmt::Debug)]
pub enum Token {
    Number(f32),
    Plus(Precedence),
    Minus(Precedence),
    Mul(Precedence), //Multiply
    Div(Precedence), //Division
    Pow(Precedence), //Caret ^
    InvalidToken(String),
    EOF,
}

impl Token {
    pub fn get_operator_precedence(&self) -> Precedence {
        match self {
            Token::Plus(a) | 
            Token::Minus(a) | 
            Token::Mul(a) | 
            Token::Div(a) | 
            Token::Pow(a) => {
               *a 
            }
            
            _ => Precedence::Min
        }
    }
}

/*
* Precedence ordered from least to greatest
*/
#[derive(std::fmt::Debug, Clone, Copy)]
#[derive(
    std::cmp::Eq, 
    std::cmp::PartialEq, 
    std::cmp::Ord, 
    std::cmp::PartialOrd
)]
pub enum Precedence {
    Min, 
    Term, // add/sub
    Factor, // mul/div
    Power, // pow
}

pub struct Lexer;

impl Lexer {
    pub fn next_token(input: &mut SplitAsciiWhitespace) -> Token {
        let data = input.next();
        
        match data {
            Some(item) => {
                match item {
                    "+" => Token::Plus(Precedence::Term),
                    "-" => Token::Minus(Precedence::Term),
                    "*" => Token::Mul(Precedence::Factor),
                    "/" => Token::Div(Precedence::Factor),
                    "^" => Token::Pow(Precedence::Power),
                    _ => {
                        let digit = item.parse::<f32>();
                        match digit {
                            Ok(num) => Token::Number(num),
                            Err(_) => Token::InvalidToken(item.to_string()),
                        }
                    }
                } 
            }
            None => { 
                Token::EOF
            }
        } 
    }
 
}


