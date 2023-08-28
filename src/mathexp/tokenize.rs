use crate::tokenize::{Token, TokenBuffer, TokenDesc, Tokenizer};
use regex::Regex;
use std::sync::LazyLock;

enum TokenizerError {}

enum TokenType {
    Literal,
    Operator,
    LeftParen,
    RightParen,
}

static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\w*").unwrap());
static LITERAL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[0-9]+").unwrap());
static OPERATOR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[+\-*/%]").unwrap());
static LEFT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\(").unwrap());
static RIGHT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\)").unwrap());

struct Tokens<'tokens> {
    source: &'tokens str,
    tokens: Vec<Token<'tokens, TokenType>>,
}

impl TokenDesc for TokenType {}

impl<'tokens> TokenBuffer<'tokens> for Tokens<'tokens> {
    type TokenDesc = TokenType;
    type TokenIter<'iter> = &'iter [Token<'tokens, Self::TokenDesc>] where 'tokens: 'iter;

    fn iter(&self) -> Self::TokenIter<'_> {
        &self.tokens
    }
}

impl<'tokens> Tokenizer<'tokens> for TokenType {
    type TokenDesc = TokenType;
    type TokenBuffer = Tokens<'tokens>;

    fn tokenize(source: &'tokens str) -> Self::TokenBuffer {
        todo!()
    }
}
