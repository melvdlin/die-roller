use crate::tokenize::TokenStream;
use std::error::Error;
use std::string::ParseError;

pub trait Parser {
    type ASTNode;
    type ParseError: Error;

    fn parse<'tokens, T: TokenStream<'tokens>>(
        tokens: T,
    ) -> Result<Self::ASTNode, ParseError>;
}
