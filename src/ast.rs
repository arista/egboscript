use crate::peg_parser::Parsed;

#[derive(Debug)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    BooleanLiteral(bool),
    U32Literal(U32Literal),
}

impl Expression {
    pub fn u32_literal(val: u32, radix: Radix) -> Self {
        Self::U32Literal(U32Literal {val, radix})
    }

    pub fn binary_expression(first: Parsed<Expression>, rest: Vec<Parsed<BinaryExpressionTerm>>) -> Self {
        if rest.is_empty() {first.value}
        else {
            Self::BinaryExpression(BinaryExpression {
                first: Box::new(first),
                rest,
            })
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub first: Box<Parsed<Expression>>,
    pub rest: Vec<Parsed<BinaryExpressionTerm>>,
}

#[derive(Debug)]
pub struct BinaryExpressionTerm {
    pub op: Parsed<BinaryOp>,
    pub exp: Box<Parsed<Expression>>,
}

#[derive(Debug)]
pub struct U32Literal {
    pub val: u32,
    pub radix: Radix,
}

// The source radix of an int literal
#[derive(Debug)]
pub enum Radix {
    Decimal,
    Hex,
    Octal,
    Binary,
}

#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    ShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    Equals,
    NotEquals,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
}
