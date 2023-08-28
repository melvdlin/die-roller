use die_roller::mathexp::tokenize::{TokenType, Tokens};
use die_roller::tokenize::TokenBuffer;

fn main() {
    let expr_str = "109876543 + (-31316 * 4318 % (-932))";
    let result = Tokens::tokenize::<TokenType>(expr_str);
    println!("Expression to parse: {expr_str}");
    match result {
        | Ok(tokens) => println!(
            "Ok: [\n{}\n]",
            tokens
                .iter()
                .iter()
                .map(|token| format!("{token}"))
                .collect::<Vec<_>>()
                .join(",\n    ")
        ),
        | Err(err) => println!("Err: {err}"),
    }
}
