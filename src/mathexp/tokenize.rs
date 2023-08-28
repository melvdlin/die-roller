use crate::tokenize::{Token, TokenBuffer, TokenDesc, Tokenizer};
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::sync::LazyLock;

#[derive(Debug)]
pub enum TokenizeError {
    UnknownToken(Range<usize>, String),
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

impl<'tokens> TokenBuffer<'tokens> for Tokens<'tokens> {
    type TokenDesc = TokenType;
    type TokenIter<'iter> = &'iter [Token<'tokens, Self::TokenDesc>] where 'tokens: 'iter;

    fn iter(&self) -> Self::TokenIter<'_> {
        &self.tokens
    }
}

impl Tokenizer for TokenType {
    type TokenDesc = TokenType;
    type TokenBuffer<'tokens> = Tokens<'tokens>;
    type TokenizeError = TokenizeError;

    fn tokenize<'tokens>(
        mut source: &'tokens str,
    ) -> Result<Self::TokenBuffer<'tokens>, Self::TokenizeError> {
        let mut tokens = Tokens {
            source,
            tokens: Vec::new(),
        };
        let mut offset: usize = 0;
        while !source.is_empty() {
            let mut matched = false;
            for (token_type, re) in [
                (None, &WHITESPACE_RE),
                (Some(TokenType::Literal), &LITERAL_RE),
                (Some(TokenType::Operator), &OPERATOR_RE),
                (Some(TokenType::LeftParen), &LEFT_PAREN_RE),
                (Some(TokenType::RightParen), &RIGHT_PAREN_RE),
            ] {
                if let Some(mat) = re.find(source) {
                    matched = true;
                    source = &source[mat.end()..];
                    let range = mat.range();
                    let src = tokens.source;
                    if let Some(token_type) = token_type {
                        let abs_start = mat.start() + offset;
                        let abs_end = mat.end() + offset;
                        tokens.add_token(abs_start..abs_end, token_type);
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
