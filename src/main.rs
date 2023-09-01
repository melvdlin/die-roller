use die_roller::mathexp::tokenize::{Tokenizer, Tokens};
use die_roller::tokenize::TokenBuffer;

fn main() {
    let expr_str = "109876543 + (-31316 * 4318 % (-932))";
    let result: die_roller::mathexp::tokenize::Result<_> = Tokens::tokenize::<Tokenizer>(expr_str);
    println!("Expression to parse: {expr_str}");
    match result {
        | Ok(tokens) => println!(
            "Ok: [\n{}\n]",
            tokens
                .iter()
                .map(|token| format!("{token}"))
                .collect::<Vec<_>>()
                .join(",\n    ")
        ),
        | Err(err) => println!("Err: {err}"),
    }
}
