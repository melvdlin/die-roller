use crate::parse::Parser;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, Range};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Token<'token> {
    source: &'token str,
    range: Range<usize>,
}

impl<'token> Display for Token<'token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.source[self.range.clone()])
    }
}

impl<'token> AsRef<str> for Token<'token> {
    fn as_ref(&self) -> &str {
        &self.source[self.range.clone()]
    }
}

impl<'token> Deref for Token<'token> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

pub trait TokenStream<'tokens>:
    Sized + FromStr + IntoIterator<Item = Token<'tokens>>
{
    fn parse<T: Parser>(self) -> T::ASTNode {
        T::parse::<'tokens, Self>(self)
    }
}
