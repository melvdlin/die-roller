use crate::mathexp::ast::{BinOp, DecDigit, DecLit, Exp, GroupedExp, Lit, UnOpPost, UnOpPre};
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

#[derive(Debug, Copy, Clone)]
enum Operator {
    BinOp(BinOp),
    UnOpPre(UnOpPre),
    UnOpPost(UnOpPost),
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
        todo!()
    }
}

impl Parser {
    fn parse_with_precedence<'iter>(
        tokens: &mut Peekable<Enumerate<<Tokens as TokenBuffer>::TokenIter<'iter>>>,
        precedence: Option<usize>,
    ) -> Result<Exp> {
        let (idx, peek) = tokens.peek().ok_or(ParseError::EndOfTokenStream)?;
        let result = if peek.eq("(") {
            tokens.next();
            let exp = Self::parse_with_precedence(tokens, None)?;
            let (idx, peek) = tokens.peek().ok_or(ParseError::EndOfTokenStream)?;
            if peek.ne(")") {
                Err(ParseError::ImbalancedParentheses(*idx))
            } else {
                Ok(Exp::GroupedExp(GroupedExp::new(exp)))
            }
        } else if let Some(lit) = Lit::parse(peek) {
            Ok(Exp::Lit(lit))
        } else if todo!() {
            todo!()
        } else {
            Err(ParseError::UnknownToken(*idx))
        };

        tokens.next();
        result
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

impl Precedence for Operator {
    fn precedence(&self) -> usize {
        match self {
            | Operator::BinOp(op) => op.precedence(),
            | Operator::UnOpPre(op) => op.precedence(),
            | Operator::UnOpPost(op) => op.precedence(),
        }
    }
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
        match *self {}
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
            | _ => None,
        }
    }
}
