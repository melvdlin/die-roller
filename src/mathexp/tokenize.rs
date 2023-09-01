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
static LITERAL_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[0-9]+").unwrap());
static OPERATOR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[+\-*/%^]").unwrap());
static LEFT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\(").unwrap());
static RIGHT_PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A\)").unwrap());

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
            let mut matched = false;
            for (keep, re) in [
                (false, &WHITESPACE_RE),
                (true, &LITERAL_RE),
                (true, &OPERATOR_RE),
                (true, &LEFT_PAREN_RE),
                (true, &RIGHT_PAREN_RE),
            ] {
                if let Some(mat) = re.find(source) {
                    matched = true;
                    source = &source[mat.end()..];
                    if keep {
                        let abs_start = mat.start() + offset;
                        let abs_end = mat.end() + offset;
                        tokens.add_token(abs_start..abs_end);
                    }
                    offset += mat.end();
                    if matched {
                        break;
                    }
                }
            }
            if !matched {
                return Err(TokenizeError::UnknownToken(0..0, source.to_string()));
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
