use die_roller::mathexp::parse::Parser;
use die_roller::mathexp::tokenize::Tokenizer;

fn main() {
    let expr = "1 + 2 + 3 * 4 * 5 / 6 / 7 ^ 8 ^ 9 * 10 / 11 + 12";
    let tokens = <Tokenizer as die_roller::tokenize::Tokenizer>::tokenize(expr).unwrap();
    let ast = <Parser as die_roller::parse::Parser>::parse(&tokens).unwrap();
    println!("{expr}");
    println!("{ast}");
}
