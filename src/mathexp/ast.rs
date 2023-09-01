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
pub enum UnOpPost {}

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
