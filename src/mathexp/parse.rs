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
        let mut result = if peek.eq("(") {
            tokens.next();
            let exp = Self::parse_with_precedence(tokens, None)?;
            let (idx, peek) = tokens.peek().ok_or(ParseError::EndOfTokenStream)?;
            if peek.ne(")") {
                Err(ParseError::ImbalancedParentheses(*idx))
            } else {
                tokens.next();
                Ok(Exp::GroupedExp(GroupedExp::new(exp)))
            }
        } else if let Some(lit) = Lit::parse(peek) {
            tokens.next();
            Ok(Exp::Lit(lit))
        } else if let Some(op) = UnOpPre::parse(peek) {
            tokens.next();
            let exp = Self::parse_with_precedence(tokens, Some(op.precedence()))?;
            Ok(Exp::UnExpPre(UnExpPre::new(op, exp)))
        } else {
            Err(ParseError::UnknownToken(*idx))
        };

        if let Ok(mut exp) = result {
            #[allow(clippy::never_loop)]
            loop {
                if let Some((idx, peek)) = tokens.peek() {
                    if let Some(op) = UnOpPost::parse(peek) {
                        if precedence.is_none() || op.precedence() <= precedence.unwrap() {
                            exp = Exp::UnExpPost(UnExpPost::new(exp, op));
                            tokens.next();
                        }
                    }
                }
                break;
            }

            if let Some((idx, peek)) = tokens.peek() {
                if let Some(op) = BinOp::parse(peek) {
                    match op.associativity() {
                        | Direction::Left => {
                            if precedence.is_none() || precedence.unwrap() > op.precedence() {
                                tokens.next();
                                exp = Exp::BinExp(BinExp::new(
                                    exp,
                                    op,
                                    Self::parse_with_precedence(tokens, Some(op.precedence()))?,
                                ));
                            }
                        }
                        | Direction::Right => {
                            if precedence.is_some() && precedence.unwrap() <= op.precedence() {
                                tokens.next();
                                exp = Exp::BinExp(BinExp::new(
                                    exp,
                                    op,
                                    Self::parse_with_precedence(tokens, Some(op.precedence()))?,
                                ))
                            }
                        }
                    }
                }
            }
            return Ok(exp);
        }

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
