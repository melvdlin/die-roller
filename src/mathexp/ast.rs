use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Exp {
    BinExp(BinExp),
    UnExpPre(UnExpPre),
    UnExpPost(UnExpPost),
    GroupedExp(GroupedExp),
    Lit(Lit),
}

#[derive(Debug, Clone)]
pub struct BinExp {
    pub lhs: Box<Exp>,
    pub op: BinOp,
    pub rhs: Box<Exp>,
}

#[derive(Debug, Copy, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
}

#[derive(Debug, Clone)]
pub struct UnExpPre {
    pub op: UnOpPre,
    pub exp: Box<Exp>,
}

#[derive(Debug, Copy, Clone)]
pub enum UnOpPre {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
pub struct UnExpPost {
    pub exp: Box<Exp>,
    pub op: UnOpPost,
}

#[derive(Debug, Copy, Clone)]
pub enum UnOpPost {
    ExclamationMark,
}

#[derive(Debug, Clone)]
pub struct GroupedExp {
    pub exp: Box<Exp>,
}

#[derive(Debug, Clone)]
pub enum Lit {
    DecLit(DecLit),
}

#[derive(Debug, Clone)]
pub struct DecLit {
    pub digits: Vec<DecDigit>,
}

#[derive(Debug, Copy, Clone)]
pub enum DecDigit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl BinExp {
    pub fn new(lhs: Exp, op: BinOp, rhs: Exp) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

impl UnExpPre {
    pub fn new(op: UnOpPre, exp: Exp) -> Self {
        Self {
            op,
            exp: Box::new(exp),
        }
    }
}

impl UnExpPost {
    pub fn new(exp: Exp, op: UnOpPost) -> Self {
        Self {
            exp: Box::new(exp),
            op,
        }
    }
}

impl GroupedExp {
    pub fn new(exp: Exp) -> Self {
        Self { exp: Box::new(exp) }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            | Exp::BinExp(bin_exp) => write!(f, "[{bin_exp}]"),
            | Exp::UnExpPre(un_exp_pre) => write!(f, "[{un_exp_pre}]"),
            | Exp::UnExpPost(un_exp_post) => write!(f, "[{un_exp_post}]"),
            | Exp::GroupedExp(grouped_exp) => write!(f, "[{grouped_exp}]"),
            | Exp::Lit(lit) => write!(f, "[{lit}]"),
        }
    }
}

impl Display for BinExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

impl Display for UnExpPre {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.op, self.exp)
    }
}

impl Display for UnExpPost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.exp, self.op)
    }
}

impl Display for GroupedExp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.exp)
    }
}

impl Display for Lit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            | Lit::DecLit(dec_lit) => write!(f, "{dec_lit}"),
        }
    }
}

impl Display for DecLit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_iter(
                self.digits
                    .iter()
                    .map(|digit| (*digit as usize).to_string())
            )
        )
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | BinOp::Plus => "+",
                | BinOp::Minus => "-",
                | BinOp::Star => "*",
                | BinOp::Slash => "/",
                | BinOp::Percent => "%",
                | BinOp::Caret => "^",
            }
        )
    }
}

impl Display for UnOpPre {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | UnOpPre::Plus => "+",
                | UnOpPre::Minus => "-",
            }
        )
    }
}

impl Display for UnOpPost {
    #[allow(unreachable_code)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | UnOpPost::ExclamationMark => "!",
            }
        )
    }
}
