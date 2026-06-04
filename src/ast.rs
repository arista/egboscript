use crate::peg_parser::Parsed;

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement(ExpressionStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
}

impl Statement {
    pub fn empty_statement() -> Self {
        Self::EmptyStatement
    }
    
    pub fn expression_statement(exp: Parsed<Expression>) -> Self {
        Self::ExpressionStatement(ExpressionStatement {
            exp: Box::new(exp),
        })
    }
    
    pub fn if_statement(test: Parsed<Expression>, if_true: Parsed<Statement>, if_false: Option<Parsed<Statement>>) -> Self {
        Self::IfStatement(IfStatement {
            test: Box::new(test),
            if_true: Box::new(if_true),
            if_false: if_false.map(|v| Box::new(v)),
        })
    }
    
    pub fn while_statement(test: Parsed<Expression>, stmt: Parsed<Statement>) -> Self {
        Self::WhileStatement(WhileStatement {
            test: Box::new(test),
            stmt: Box::new(stmt),
        })
    }
    
    pub fn return_statement(exp: Option<Parsed<Expression>>) -> Self {
        Self::ReturnStatement(ReturnStatement {
            exp: exp.map(|v| Box::new(v)),
        })
    }
    
    pub fn break_statement(label: Option<Parsed<String>>) -> Self {
        Self::BreakStatement(BreakStatement {
            label,
        })
    }
    
    pub fn continue_statement(label: Option<Parsed<String>>) -> Self {
        Self::ContinueStatement(ContinueStatement {
            label,
        })
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub exp: Box<Parsed<Expression>>
}

#[derive(Debug)]
pub struct IfStatement {
    pub test: Box<Parsed<Expression>>,
    pub if_true: Box<Parsed<Statement>>,
    pub if_false: Option<Box<Parsed<Statement>>>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub test: Box<Parsed<Expression>>,
    pub stmt: Box<Parsed<Statement>>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub exp: Option<Box<Parsed<Expression>>>,
}

#[derive(Debug)]
pub struct BreakStatement {
    pub label: Option<Parsed<String>>,
}

#[derive(Debug)]
pub struct ContinueStatement {
    pub label: Option<Parsed<String>>,
}


#[derive(Debug)]
pub enum Expression {
    CommaExpression(CommaExpression),
    TernaryExpression(TernaryExpression),

    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    BooleanLiteral(bool),
    NullLiteral,
    StringLiteral(String),
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

    pub fn null_literal() -> Self {
        Self::NullLiteral
    }

    pub fn string_literal(str: String) -> Self {
        Self::StringLiteral(str)
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
