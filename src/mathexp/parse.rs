use crate::mathexp::ast::Expression;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {}

type Error = ParseError;
type Result<T> = std::result::Result<T, Error>;

pub struct Parser {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error")
    }
}

impl std::error::Error for ParseError {}

impl crate::parse::Parser for Parser {
    type ASTNode = Expression;
    type TokenDesc = super::tokenize::TokenType;
    type TokenBuffer = super::tokenize::Tokens;
    type ParseError = ParseError;

    fn parse(tokens: &Self::TokenBuffer) -> Result<Self::ASTNode> {
        todo!()
    }
}
