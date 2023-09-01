use crate::mathexp::ast::{
    BinExp, BinOp, DecDigit, DecLit, Exp, GroupedExp, Lit, UnExpPost, UnExpPre, UnOpPost, UnOpPre,
};
use crate::mathexp::tokenize::Tokens;
use crate::tokenize::{Token, TokenBuffer};
use std::fmt::{Display, Formatter};
use std::iter::{Enumerate, Peekable};

#[derive(Debug)]
pub enum ParseError {
    EndOfTokenStream,
    UnknownToken(usize),
    ImbalancedParentheses(usize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

pub type Error = ParseError;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Parser {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error")
    }
}

impl std::error::Error for ParseError {}

impl crate::parse::Parser for Parser {
    type ASTNode = Exp;
    type TokenBuffer = Tokens;
    type ParseError = ParseError;

    fn parse(tokens: &Self::TokenBuffer) -> Result<Self::ASTNode> {
        let mut peekable = tokens.iter().enumerate().peekable();

        todo!()
    }
}

trait ASTLeaf: Sized {
    fn parse(token: &Token) -> Option<Self>;
}

trait Precedence {
    fn precedence(&self) -> usize;
}

trait Associativity {
    fn associativity(&self) -> Direction;
}

impl Precedence for BinOp {
    fn precedence(&self) -> usize {
        match self {
            | BinOp::Plus | BinOp::Minus => 5,
            | BinOp::Star | BinOp::Slash | BinOp::Percent => 4,
            | BinOp::Caret => 2,
        }
    }
}

impl Precedence for UnOpPre {
    fn precedence(&self) -> usize {
        match self {
            | UnOpPre::Plus | UnOpPre::Minus => 3,
        }
    }
}

impl Precedence for UnOpPost {
    fn precedence(&self) -> usize {
        match self {
            | UnOpPost::ExclamationMark => 3,
        }
    }
}

impl Associativity for BinOp {
    fn associativity(&self) -> Direction {
        match self {
            | BinOp::Plus | BinOp::Minus | BinOp::Star | BinOp::Slash | BinOp::Percent => {
                Direction::Left
            }
            | BinOp::Caret => Direction::Right,
        }
    }
}

impl ASTLeaf for Lit {
    fn parse(token: &Token) -> Option<Self> {
        let mut digits = Vec::with_capacity(token.raw().len());
        for chr in token.chars() {
            match chr {
                | '0' => digits.push(DecDigit::Zero),
                | '1' => digits.push(DecDigit::One),
                | '2' => digits.push(DecDigit::Two),
                | '3' => digits.push(DecDigit::Three),
                | '4' => digits.push(DecDigit::Four),
                | '5' => digits.push(DecDigit::Five),
                | '6' => digits.push(DecDigit::Six),
                | '7' => digits.push(DecDigit::Seven),
                | '8' => digits.push(DecDigit::Eight),
                | '9' => digits.push(DecDigit::Nine),
                | _ => return None,
            }
        }

        Some(Lit::DecLit(DecLit { digits }))
    }
}

impl ASTLeaf for BinOp {
    fn parse(token: &Token) -> Option<Self> {
        match token as &str {
            | "+" => Some(BinOp::Plus),
            | "-" => Some(BinOp::Minus),
            | "*" => Some(BinOp::Star),
            | "/" => Some(BinOp::Slash),
            | "%" => Some(BinOp::Percent),
            | "^" => Some(BinOp::Caret),
            | _ => None,
        }
    }
}

impl ASTLeaf for UnOpPre {
    fn parse(token: &Token) -> Option<Self> {
        match token as &str {
            | "+" => Some(UnOpPre::Plus),
            | "-" => Some(UnOpPre::Minus),
            | _ => None,
        }
    }
}

impl ASTLeaf for UnOpPost {
    fn parse(token: &Token) -> Option<Self> {
        match token as &str {
            | "!" => Some(UnOpPost::ExclamationMark),
            | _ => None,
        }
    }
}
