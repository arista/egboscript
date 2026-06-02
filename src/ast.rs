use crate::peg_parser::Parsed;

#[derive(Debug)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
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

    pub fn unary_expression(ops: Vec<Parsed<UnaryOp>>, exp: Parsed<Expression>, ) -> Self {
        if ops.is_empty() {exp.value}
        else {
            Self::UnaryExpression(UnaryExpression {
                ops,
                exp: Box::new(exp),
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
pub struct UnaryExpression {
    pub ops: Vec<Parsed<UnaryOp>>,
    pub exp: Box<Parsed<Expression>>
}

#[derive(Debug)]
pub struct U32Literal {
    pub val: u32,
    pub radix: Radix,
}

// The source radix of an int literal
#[derive(Debug, Clone, Copy)]
pub enum Radix {
    Decimal,
    Hex,
    Octal,
    Binary,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}
