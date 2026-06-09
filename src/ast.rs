use crate::peg_parser::Parsed;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Parsed<FileItem>>
}

#[derive(Debug)]
pub enum FileItem {
    Statement(Statement),
    TypeDecl(TypeDecl),
    ImportDecl(ImportDecl),
}

#[derive(Debug)]
pub struct TypeDecl {
}

#[derive(Debug)]
pub struct ImportDecl {
}

#[derive(Debug)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement(ExpressionStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement(BreakStatement),
    ContinueStatement(ContinueStatement),
    BlockStatement(BlockStatement),
    ForStatement(ForStatement),
    SwitchStatement(SwitchStatement),
    VarDeclStatement(VarDeclStatement),
    FunctionDeclStatement(FunctionDeclStatement),
    LabeledStatement(LabeledStatement),
    TryStatement(TryStatement),
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
    
    pub fn block_statement(stmts: Parsed<Vec<Parsed<Statement>>>) -> Self {
        Self::BlockStatement(BlockStatement {
            stmts: Box::new(stmts),
        })
    }
    
    pub fn for_statement(init: Option<Parsed<ForInit>>, test: Option<Parsed<Expression>>, advance: Option<Parsed<Expression>>, stmt: Parsed<Statement>) -> Self {
        Self::ForStatement(ForStatement {
            init: init.map(|v| Box::new(v)),
            test: test.map(|v| Box::new(v)),
            advance: advance.map(|v| Box::new(v)),
            stmt: Box::new(stmt),
        })
    }
    
    pub fn switch_statement(exp: Parsed<Expression>, items: Parsed<Vec<Parsed<SwitchItem>>>) -> Self {
        Self::SwitchStatement(SwitchStatement {
            exp: Box::new(exp),
            items: Box::new(items),
        })
    }

    pub fn var_decl_statement(let_or_const: Parsed<LetOrConst>, name: Parsed<String>, init: Option<Parsed<Expression>>) -> Self {
        Self::VarDeclStatement(VarDeclStatement {
            let_or_const,
            name,
            init: init.map(|v| Box::new(v)),
        })
    }

    pub fn function_decl_statement(name: Parsed<String>, args: Parsed<Vec<Parsed<FunctionDeclArg>>>, body: Parsed<Statement>) -> Self {
        Self::FunctionDeclStatement(FunctionDeclStatement {
            name,
            args,
            body: Box::new(body),
        })
    }

    pub fn labeled_statement(name: Parsed<String>, stmt: Parsed<Statement>) -> Self {
        Self::LabeledStatement(LabeledStatement {
            name,
            stmt: Box::new(stmt),
        })
    }

    pub fn try_statement(stmt: Parsed<Statement>, catch_clause: Option<Parsed<CatchClause>>, finally_clause: Option<Parsed<Statement>>) -> Self {
        Self::TryStatement(TryStatement {
            stmt: Box::new(stmt),
            catch_clause: catch_clause.map(|v| Box::new(v)),
            finally_clause: finally_clause.map(|v| Box::new(v)),
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
pub struct BlockStatement {
    pub stmts: Box<Parsed<Vec<Parsed<Statement>>>>,
}

#[derive(Debug)]
pub struct ForStatement {
    pub init: Option<Box<Parsed<ForInit>>>,
    pub test: Option<Box<Parsed<Expression>>>,
    pub advance: Option<Box<Parsed<Expression>>>,
    pub stmt: Box<Parsed<Statement>>,
}

#[derive(Debug)]
pub enum ForInit {
    Expression(Parsed<Expression>),
    VarDecl(Parsed<Statement>),
}

#[derive(Debug)]
pub struct SwitchStatement {
    pub exp: Box<Parsed<Expression>>,
    pub items: Box<Parsed<Vec<Parsed<SwitchItem>>>>,
}

#[derive(Debug)]
pub enum SwitchItem {
    Statement(Parsed<Statement>),
    Case(Parsed<Expression>),
    Default,
}

#[derive(Debug)]
pub struct VarDeclStatement {
    pub let_or_const: Parsed<LetOrConst>,
    pub name: Parsed<String>,
    pub init: Option<Box<Parsed<Expression>>>,
}

#[derive(Debug)]
pub struct FunctionDeclStatement {
    pub name: Parsed<String>,
    pub args: Parsed<Vec<Parsed<FunctionDeclArg>>>,
    // FIXME - add return type
    pub body: Box<Parsed<Statement>>,
}

#[derive(Debug)]
pub struct FunctionDeclArg {
    pub name: Parsed<String>,
    // FIXME - add arg type
}

#[derive(Debug)]
pub struct LabeledStatement {
    pub name: Parsed<String>,
    pub stmt: Box<Parsed<Statement>>,
}

#[derive(Debug)]
pub struct TryStatement {
    pub stmt: Box<Parsed<Statement>>,
    pub catch_clause: Option<Box<Parsed<CatchClause>>>,
    pub finally_clause: Option<Box<Parsed<Statement>>>,
}

#[derive(Debug)]
pub struct CatchClause {
    pub name: Option<Parsed<String>>,
    pub stmt: Box<Parsed<Statement>>,
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
    IntLiteral(IntLiteral),
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

    pub fn int_literal(value: u64, radix: Radix, suffix: Option<Parsed<IntLiteralSuffix>>) -> Self {
        Self::IntLiteral(IntLiteral {value, radix, suffix})
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
pub struct IntLiteral {
    pub value: u64,
    pub radix: Radix,
    pub suffix: Option<Parsed<IntLiteralSuffix>>,
}

#[derive(Debug, Clone, Copy)]
pub enum LetOrConst {
    Let,
    Const,
}

#[derive(Debug, Clone, Copy)]
pub enum IntLiteralSuffix {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
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
