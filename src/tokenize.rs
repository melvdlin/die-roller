use crate::parse::Parser;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, Range};

#[derive(Clone, Debug)]
pub struct Token<TokenDesc> {
    raw: String,
    range: Range<usize>,
    desc: TokenDesc,
}

impl<TokenDesc> Token<TokenDesc> {
    pub fn new(raw: &str, range: Range<usize>, desc: TokenDesc) -> Self {
        Token {
            raw: raw.to_owned(),
            range,
            desc,
        }
    }
}

impl<TokenDesc: Display> Display for Token<TokenDesc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.desc, &self.raw)
    }
}

impl<TokenDesc> AsRef<str> for Token<TokenDesc> {
    fn as_ref(&self) -> &str {
        &self.raw[self.range.clone()]
    }
}

impl<TokenDesc> Deref for Token<TokenDesc> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub trait TokenDesc: Sized {}

pub trait Tokenizer {
    type TokenDesc: TokenDesc;
    type TokenBuffer: TokenBuffer<TokenDesc = Self::TokenDesc>;
    type TokenizeError: Error;

    fn tokenize(source: &str) -> Result<Self::TokenBuffer, Self::TokenizeError>;
}

pub trait TokenBuffer: Sized {
    type TokenDesc: TokenDesc;
    type TokenIter<'iter>: Iterator<Item = &'iter Token<Self::TokenDesc>>
    where
        Self: 'iter;

    fn iter<'iter, 'self_ref: 'iter>(&'self_ref self) -> Self::TokenIter<'iter>;

    fn tokenize<T: Tokenizer<TokenBuffer = Self>>(source: &str) -> Result<Self, T::TokenizeError> {
        T::tokenize(source)
    }

    fn parse<P>(&self) -> Result<P::ASTNode, P::ParseError>
    where
        P: Parser<TokenDesc = Self::TokenDesc, TokenBuffer = Self>,
    {
        for t in self.iter() {}
        P::parse(self)
    }
}
