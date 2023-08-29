use crate::tokenize::{TokenBuffer, TokenDesc};
use std::error::Error;

pub trait Parser {
    type ASTNode;
    type TokenDesc: TokenDesc;
    type TokenBuffer: TokenBuffer<TokenDesc = Self::TokenDesc>;
    type ParseError: Error;

    fn parse(tokens: &Self::TokenBuffer) -> Result<Self::ASTNode, Self::ParseError>;
}
