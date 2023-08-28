use crate::tokenize::{TokenBuffer, TokenDesc};
use std::error::Error;

pub trait Parser {
    type ASTNode;
    type TokenDesc: TokenDesc;
    type ParseError: Error;

    fn parse<'tokens, T: TokenBuffer<'tokens, TokenDesc = Self::TokenDesc>>(
        tokens: &T,
    ) -> Result<Self::ASTNode, Self::ParseError>;
}
