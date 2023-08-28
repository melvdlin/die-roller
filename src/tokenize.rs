use crate::parse::Parser;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, Range};

#[derive(Clone, Debug)]
pub struct Token<'token, TokenDesc> {
    source: &'token str,
    range: Range<usize>,
    desc: TokenDesc,
}

impl<'token, TokenDesc> Token<'token, TokenDesc> {
    pub const fn new(source: &'token str, range: Range<usize>, desc: TokenDesc) -> Self {
        Token {
            source,
            range,
            desc,
        }
    }
}

impl<'token, TokenDesc: Display> Display for Token<'token, TokenDesc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.desc, &self.source[self.range.clone()])
    }
}

impl<'token, TokenDesc> AsRef<str> for Token<'token, TokenDesc> {
    fn as_ref(&self) -> &str {
        &self.source[self.range.clone()]
    }
}

impl<'token, TokenDesc> Deref for Token<'token, TokenDesc> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub trait TokenDesc: Sized {}

pub trait Tokenizer {
    type TokenDesc: TokenDesc;
    type TokenBuffer<'tokens>: TokenBuffer<'tokens, TokenDesc = Self::TokenDesc>;
    type TokenizeError: Error;

    fn tokenize<'tokens>(
        source: &'tokens str,
    ) -> Result<Self::TokenBuffer<'tokens>, Self::TokenizeError>;
}

pub trait TokenBuffer<'tokens>: Sized {
    type TokenDesc: TokenDesc + 'tokens;
    type TokenIter<'iter>: IntoIterator<Item = &'iter Token<'tokens, Self::TokenDesc>>
    where
        'tokens: 'iter,
        Self: 'iter;

    fn iter(&self) -> Self::TokenIter<'_>;

    fn tokenize<T: Tokenizer<TokenBuffer<'tokens> = Self>>(
        source: &'tokens str,
    ) -> Result<Self, T::TokenizeError> {
        T::tokenize(source)
    }

    fn parse<P>(&self) -> Result<P::ASTNode, P::ParseError>
    where
        P: Parser<TokenDesc = Self::TokenDesc>,
    {
        P::parse::<Self>(self)
    }
}
