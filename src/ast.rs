use crate::peg_parser::Parsed;

#[derive(Debug)]
pub enum Expression {
    CommaExpression(CommaExpression),
    TernaryExpression(TernaryExpression),

    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    BooleanLiteral(bool),
    U32Literal(U32Literal),
    MemberExpression(MemberExpression),
    IdentifierExpression(IdentifierExpression),
}

impl Expression {
    pub fn comma_expression(exps: Vec<Parsed<Expression>>) -> Self {
        Self::CommaExpression(CommaExpression { exps })
    }

    pub fn ternary_expression(terms: Vec<Parsed<TernaryExpressionTerm>>, if_false: Parsed<Expression>) -> Self {
        Self::TernaryExpression(TernaryExpression {
            terms,
            if_false: Box::new(if_false),
        })
    }

    pub fn u32_literal(val: u32, radix: Radix) -> Self {
        Self::U32Literal(U32Literal {val, radix})
    }

    pub fn binary_expression(first: Parsed<Expression>, rest: Vec<Parsed<BinaryExpressionTerm>>) -> Self {
        Self::BinaryExpression(BinaryExpression {
            first: Box::new(first),
            rest,
        })
    }

    pub fn unary_expression(ops: Vec<Parsed<UnaryOp>>, exp: Parsed<Expression>) -> Self {
        Self::UnaryExpression(UnaryExpression {
            ops,
            exp: Box::new(exp),
        })
    }

    pub fn member_expression(first: Parsed<Expression>, rest: Vec<Parsed<MemberOp>>) -> Self {
        Self::MemberExpression(MemberExpression{
            first: Box::new(first),
            rest
        })
    }

    pub fn identifier_expression(name: String) -> Self {
        Self::IdentifierExpression(IdentifierExpression {name})
    }
}

#[derive(Debug)]
pub struct CommaExpression {
    pub exps: Vec<Parsed<Expression>>
}

#[derive(Debug)]
pub struct TernaryExpression {
    pub terms: Vec<Parsed<TernaryExpressionTerm>>,
    pub if_false: Box<Parsed<Expression>>
}

#[derive(Debug)]
pub struct TernaryExpressionTerm {
    pub test: Box<Parsed<Expression>>,
    pub if_true: Box<Parsed<Expression>>,
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

    Assign,
    PlusAssign,
    MinusAssign,
    TimesAssign,
    DivideAssign,
    ModAssign,
    ShiftLeftAssign,
    LogicalShiftRightAssign,
    ArithmeticShiftRightAssign,
    BitwiseAndAssign,
    BitwiseXorAssign,
    BitwiseOrAssign,
    LogicalAndAssign,
    LogicalOrAssign,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}

#[derive(Debug)]
pub enum MemberOp {
    DotAccess(DotAccess),
    FunctionCall(FunctionCall),
    IndexAccess(IndexAccess),
    NonNullAssert,
}

#[derive(Debug)]
pub struct MemberExpression {
    pub first: Box<Parsed<Expression>>,
    pub rest: Vec<Parsed<MemberOp>>,
}

impl MemberOp {
    pub fn dot_access(name: Parsed<String>) -> Self {
        MemberOp::DotAccess(DotAccess{name})
    }

    pub fn function_call(args: Parsed<Vec<Parsed<Expression>>>) -> Self {
        MemberOp::FunctionCall(FunctionCall{args})
    }

    pub fn index_access(exp: Parsed<Expression>) -> Self {
        MemberOp::IndexAccess(IndexAccess{exp: Box::new(exp)})
    }

    pub fn non_null_assert() -> Self {
        MemberOp::NonNullAssert
    }
}

#[derive(Debug)]
pub struct DotAccess {
    pub name: Parsed<String>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub args: Parsed<Vec<Parsed<Expression>>>
}

#[derive(Debug)]
pub struct IndexAccess {
    pub exp: Box<Parsed<Expression>>,
}

#[derive(Debug)]
pub struct IdentifierExpression {
    pub name: String,
}
