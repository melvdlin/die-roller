use crate::tokenize::{Token, TokenBuffer};
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum TokenizeError {
    UnknownToken(Range<usize>, String),
}

pub type Error = TokenizeError;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Tokenizer;

#[derive(Debug)]
pub struct Tokens {
    source: String,
    tokens: Vec<Token>,
}

static WHITESPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\s+").unwrap());
static TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A(([0-9]+)|[+\-*/%^!()])").unwrap());

impl Display for TokenizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            | TokenizeError::UnknownToken(range, token) => {
                write!(
                    f,
                    "Unknown Token at [{}..{}]: {token}",
                    range.start, range.end
                )
            }
        }
    }
}

impl std::error::Error for TokenizeError {}

impl crate::tokenize::Tokenizer for Tokenizer {
    type TokenBuffer = Tokens;
    type TokenizeError = TokenizeError;

    fn tokenize(mut source: &str) -> Result<Self::TokenBuffer> {
        let mut tokens = Tokens {
            source: source.to_string(),
            tokens: Vec::new(),
        };
        let mut offset: usize = 0;
        while !source.is_empty() {
            if let Some(mat) = WHITESPACE_RE.find(source) {
                source = &source[mat.end()..];
                offset += mat.end();
            } else if let Some(mat) = TOKEN_RE.find(source) {
                let abs_start = mat.start() + offset;
                let abs_end = mat.end() + offset;
                tokens.add_token(abs_start..abs_end);

                source = &source[mat.end()..];
                offset += mat.end();
            } else {
                return Err(TokenizeError::UnknownToken(
                    offset..(offset + source.len()),
                    source.to_string(),
                ));
            }
        }
        Ok(tokens)
    }
}

impl Tokens {
    fn add_token(&mut self, range: Range<usize>) {
        self.tokens
            .push(Token::new(&self.source[range.clone()], range));
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tokens in {}: [{}]",
            self.source,
            self.tokens
                .iter()
                .map(|token| format!("{token}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl TokenBuffer for Tokens {
    type TokenIter<'iter> = std::slice::Iter<'iter, Token>;

    fn iter<'iter, 'self_ref: 'iter>(&'self_ref self) -> Self::TokenIter<'iter> {
        self.tokens.iter()
    }
}
