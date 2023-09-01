use crate::mathexp::ast::*;
use crate::mathexp::tokenize::Tokens;
use crate::tokenize::{Token, TokenBuffer};
use std::fmt::{Display, Formatter};

pub type Error = ParseError;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ParseError {
    EndOfTokenStream,
    UnexpectedToken(usize, String),
    ImbalancedParentheses(usize),
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Fragment {
    // SubExp(Exp),
    GroupedExp,
    UnExpPre(UnOpPre),
    BinExp(Exp, BinOp),
}

pub struct Parser {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse Error: {}",
            match self {
                | ParseError::EndOfTokenStream => "Unexpected end of token stream".to_string(),
                | ParseError::UnexpectedToken(idx, token) =>
                    format!("Unexpected token at index {idx}: {token}"),
                | ParseError::ImbalancedParentheses(idx) =>
                    format!("Imbalanced parentheses at index {idx}"),
            }
        )
    }
}

impl std::error::Error for ParseError {}

impl crate::parse::Parser for Parser {
    type ASTNode = Exp;
    type TokenBuffer = Tokens;
    type ParseError = ParseError;

    fn parse(tokens: &Self::TokenBuffer) -> Result<Self::ASTNode> {
        let mut tokens = tokens.iter().enumerate().peekable();
        let mut frags = Vec::<Fragment>::new();
        let mut sub_exp: Option<Exp> = None;

        while !frags.is_empty() || tokens.peek().is_some() {
            // token could be:
            // ")":         must be preceded by [ "(" SubExp ]              -> SubExp
            // Lit:         must be preceded by [ "(" | UnOpPre | BinOp ]   -> SubExp
            // UnOpPost     must be preceded by [ SubExp ]                  -> SubExp
            // "("          must be preceded by [ "(" | UnOpPre | BinOp ]   -> GroupedExp
            // UnOpPre      must be preceded by [ "(" | UnOpPre | BinOp ]   -> UnExpPre
            // BinOp        must be preceded by [ SubExp ]                  -> BinExp
            // every iteration:
            // 1: try to resolve stack with token
            //      if sub_exp available and next token = [ BinOp | UnOpPost ] = op
            //          if TOS = BinExp
            //              if TOS.associativity = Left
            //                  if op.precedence < TOS.precedence
            //                      push BinOp(SubExp, op)
            //                  else
            //                      sub_exp = pop TOS + sub_exp
            //                      if token = BinOp
            // 2: otherwise create new fragment with token and push fragment onto stack

            //

            if let Some(exp) = sub_exp.take() {
                if let Some((idx, token)) = tokens.peek() {
                    if let Some(token_op) = BinOp::parse(token) {
                        if let Some(tos) = frags.pop() {
                            match tos {
                                | Fragment::UnExpPre(tos_op) => {
                                    if tos_op.has_precedence(&token_op) {
                                        sub_exp.replace(Exp::UnExpPre(UnExpPre::new(tos_op, exp)));
                                    } else {
                                        frags.push(tos);
                                        frags.push(Fragment::BinExp(exp, token_op));
                                        tokens.next();
                                    }
                                }
                                | Fragment::BinExp(tos_exp, tos_op) => {
                                    if tos_op.has_associative_precedence(&token_op) {
                                        sub_exp.replace(Exp::BinExp(BinExp::new(
                                            tos_exp, tos_op, exp,
                                        )));
                                    } else {
                                        frags.push(Fragment::BinExp(tos_exp, tos_op));
                                        frags.push(Fragment::BinExp(exp, token_op));
                                        tokens.next();
                                    }
                                }
                                | Fragment::GroupedExp => {
                                    frags.push(tos);
                                    frags.push(Fragment::BinExp(exp, token_op));
                                    tokens.next();
                                }
                            }
                            continue;
                        } else {
                            frags.push(Fragment::BinExp(exp, token_op));
                            tokens.next();
                            continue;
                        }
                    } else if let Some(token_op) = UnOpPost::parse(token) {
                        if let Some(tos) = frags.pop() {
                            match tos {
                                | Fragment::UnExpPre(tos_op) => {
                                    if tos_op.has_precedence(&token_op) {
                                        sub_exp.replace(Exp::UnExpPre(UnExpPre::new(tos_op, exp)));
                                        continue;
                                    } else {
                                        sub_exp
                                            .replace(Exp::UnExpPost(UnExpPost::new(exp, token_op)));
                                        frags.push(tos);
                                        tokens.next();
                                        continue;
                                    }
                                }
                                | Fragment::BinExp(tos_exp, tos_op) => {
                                    if tos_op.has_associative_precedence(&token_op) {
                                        sub_exp.replace(Exp::BinExp(BinExp::new(
                                            tos_exp, tos_op, exp,
                                        )));
                                        continue;
                                    } else {
                                        sub_exp
                                            .replace(Exp::UnExpPost(UnExpPost::new(exp, token_op)));
                                        frags.push(Fragment::BinExp(tos_exp, tos_op));
                                        tokens.next();
                                        continue;
                                    }
                                }
                                | _ => {
                                    frags.push(tos);
                                }
                            }
                        } else {
                            sub_exp.replace(Exp::UnExpPost(UnExpPost::new(exp, token_op)));
                            tokens.next();
                            continue;
                        }
                    } else if token.eq(")") {
                        if let Some(tos) = frags.pop() {
                            match tos {
                                | Fragment::UnExpPre(tos_op) => {
                                    sub_exp.replace(Exp::UnExpPre(UnExpPre::new(tos_op, exp)));
                                }
                                | Fragment::BinExp(tos_exp, tos_op) => {
                                    sub_exp.replace(Exp::BinExp(BinExp::new(tos_exp, tos_op, exp)));
                                }
                                | Fragment::GroupedExp => {
                                    sub_exp.replace(Exp::GroupedExp(GroupedExp::new(exp)));
                                    tokens.next();
                                }
                            }
                            continue;
                        } else {
                            return Err(ParseError::ImbalancedParentheses(*idx));
                        }
                    } else {
                        return Err(ParseError::UnexpectedToken(*idx, token.to_string()));
                    }
                } else if let Some(tos) = frags.pop() {
                    match tos {
                        | Fragment::GroupedExp => {
                            return Err(ParseError::EndOfTokenStream);
                        }
                        | Fragment::UnExpPre(tos_op) => {
                            sub_exp.replace(Exp::UnExpPre(UnExpPre::new(tos_op, exp)));
                            continue;
                        }
                        | Fragment::BinExp(tos_exp, tos_op) => {
                            sub_exp.replace(Exp::BinExp(BinExp::new(tos_exp, tos_op, exp)));
                            continue;
                        }
                    }
                }
            } else if let Some((idx, token)) = tokens.peek() {
                if let Some(token_lit) = Lit::parse(token) {
                    sub_exp.replace(Exp::Lit(token_lit));
                    tokens.next();
                    continue;
                } else if let Some(token_op) = UnOpPre::parse(token) {
                    frags.push(Fragment::UnExpPre(token_op));
                    tokens.next();
                    continue;
                } else if token.eq("(") {
                    frags.push(Fragment::GroupedExp);
                    tokens.next();
                    continue;
                } else {
                    return Err(ParseError::UnexpectedToken(*idx, token.to_string()));
                }
            } else {
                return Err(ParseError::EndOfTokenStream);
            }
        }

        sub_exp.ok_or(ParseError::EndOfTokenStream)
    }
}

trait ASTLeaf: Sized {
    fn parse(token: &Token) -> Option<Self>;
}

trait Precedence {
    fn precedence(&self) -> usize;

    fn has_precedence(&self, over: &impl Precedence) -> bool {
        self.precedence() < over.precedence()
    }

    fn has_associative_precedence(&self, over: &impl Precedence) -> bool
    where
        Self: Associativity,
    {
        match self.associativity() {
            | Direction::Left => self.precedence() <= over.precedence(),
            | Direction::Right => self.precedence() < over.precedence(),
        }
    }
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
