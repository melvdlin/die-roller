use core::slice;
use std::str::FromStr;
use crate::tokenize::{Token, TokenStream};

enum TokenizerError {

}

struct Tokens<'tokens> {
    source: &'tokens str,
    tokens: Vec<Token<'tokens>>,
}

impl<'tokens> FromStr for Tokens<'tokens> {
    type Err = TokenizerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl<'tokens> TokenStream<'tokens> for Tokens<'tokens> {
    type TokenIter = slice::Iter<'tokens, Token<'tokens>>;

    fn iter(&self) -> Self::TokenIter {
        self.tokens.iter()
    }
}