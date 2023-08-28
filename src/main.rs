use die_roller::mathexp::tokenize::{Tokens, TokenType};
use die_roller::tokenize::TokenBuffer;

fn main() {
    let expr_str = "109876543 + (-31316 * 4318 % (-932))";
    let result = Tokens::tokenize::<TokenType>(expr_str);
    match result {
        Ok(tokens) => println!("Ok:  {tokens}"),
        Err(err) => println!("Err: {err}"),
    }
}
