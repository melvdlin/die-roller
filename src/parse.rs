use crate::tokenize::TokenBuffer;
use std::error::Error;

pub trait Parser {
    type ASTNode;
    type TokenBuffer: TokenBuffer;
    type ParseError: Error;

    fn parse(tokens: &Self::TokenBuffer) -> Result<Self::ASTNode, Self::ParseError>;
}
