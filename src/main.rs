pub mod calculator;

use calculator::parser::*;

fn main() {
    let expr = "60 + 3 ^ 2";

    let mut parser = Parser::parse_str(expr);
    let ast = parser.get_expression();

    let answer = ast.evaluate();

    println!("{expr} = {answer}");
}


