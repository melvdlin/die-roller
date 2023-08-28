use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use crate::tokenize::{Token, TokenBuffer, TokenDesc, Tokenizer};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum TokenizeError {
    UnknownToken(Range<usize>, String)
}

#[derive(Debug)]
pub enum TokenType {
    Literal,
    Operator,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub struct Tokens<'tokens> {
    source: &'tokens str,
    tokens: Vec<Token<'tokens, TokenType>>,
}

static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\s+").unwrap());
static LITERAL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[0-9]+").unwrap());
static OPERATOR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[+\-*/%]").unwrap());
static LEFT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\(").unwrap());
static RIGHT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\)").unwrap());


impl Display for TokenizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::UnknownToken(range, token) => {
                write!(f, "Unknown Token at [{}..{}]: {token}", range.start, range.end)
            }
        }
    }
}

impl Error for TokenizeError {}

impl TokenDesc for TokenType {}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'tokens> Tokens<'tokens> {
    fn add_token(&mut self, range: Range<usize>, token_type: TokenType) {
        self.tokens.push(Token::new(self.source, range, token_type));
    }
}

impl<'tokens> Display for Tokens<'tokens> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tokens.iter().map(|token| format!("{token}")).collect::<Vec<_>>())
    }
}

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
    type TokenizeError = TokenizeError;

    fn tokenize(mut source: &'tokens str) -> Result<Self::TokenBuffer, Self::TokenizeError> {
        let mut tokens = Tokens {
            source,
            tokens: Vec::new(),
        };
        let mut offset: usize = 0;
        'tokenize_loop:
        while !source.is_empty() {
            for (token_type, re) in [
                (None, &WHITESPACE_RE),
                (Some(TokenType::Literal), &LITERAL_RE),
                (Some(TokenType::Operator), &OPERATOR_RE),
                (Some(TokenType::LeftParen), &LEFT_PAREN_RE),
                (Some(TokenType::RightParen), &RIGHT_PAREN_RE),
            ] {
                let pat = re.to_string();
                let pat = pat;
                if let Some(mat) = re.find(source) {
                    offset += mat.end();
                    source = &source[mat.end()..];
                    let range = mat.range();
                    let src = tokens.source;
                    if let Some(token_type) = token_type {
                        let abs_start = mat.start() + offset;
                        let abs_end = mat.end() + offset;
                        tokens.add_token(abs_start..abs_end, token_type);
                        continue 'tokenize_loop;
                    }
                }
            }
            return Err(TokenizeError::UnknownToken(0..0, source.to_string()));
        }
        Ok(tokens)
    }
}
