use crate::parse::Parser;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, Range};

#[derive(Clone, Debug)]
pub struct Token {
    raw: String,
    range: Range<usize>,
}

impl Token {
    pub fn new(raw: &str, range: Range<usize>) -> Self {
        Token {
            raw: raw.to_owned(),
            range,
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.raw)
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.raw[self.range.clone()]
    }
}

impl Deref for Token {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub trait Tokenizer {
    type TokenBuffer: TokenBuffer;
    type TokenizeError: Error;

    fn tokenize(source: &str) -> Result<Self::TokenBuffer, Self::TokenizeError>;
}

pub trait TokenBuffer: Sized {
    type TokenIter<'iter>: Iterator<Item = &'iter Token>
    where
        Self: 'iter;

    fn iter<'iter, 'self_ref: 'iter>(&'self_ref self) -> Self::TokenIter<'iter>;

    fn tokenize<T: Tokenizer<TokenBuffer = Self>>(source: &str) -> Result<Self, T::TokenizeError> {
        T::tokenize(source)
    }

    fn parse<P>(&self) -> Result<P::ASTNode, P::ParseError>
    where
        P: Parser<TokenBuffer = Self>,
    {
        P::parse(self)
    }
}
