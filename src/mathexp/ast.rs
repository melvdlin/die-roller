use syn::Expr;

pub enum Expression {
    GroupedExpression(GroupedExpression),
    LiteralExpression(LiteralExpression),
    OperatorExpression(OperatorExpression),
}

pub struct GroupedExpression {
    expr: Box<Expr>,
}

pub enum LiteralExpression {
    DecLiteral(DecLiteral),
}

pub struct DecLiteral {
    digits: Vec<DecDigit>,
}

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

pub enum OperatorExpression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
}

pub struct BinaryExpression {
    lhs: Box<Expression>,
    operator: BinaryOperator,
    rhs: Box<Expression>,
}

pub struct UnaryExpression {
    operator: UnaryOperator,
    expr: Box<Expression>,
}

pub enum BinaryOperator {
    ArithmeticOperator(ArithmeticOperator),
}

pub enum ArithmeticOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}

pub enum UnaryOperator {
    NegationOperator(NegationOperator),
}

pub enum NegationOperator {
    Minus,
}
