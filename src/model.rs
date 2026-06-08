use std::{collections::HashMap, marker::PhantomData};

pub enum ModelError {
    TypeMismatch(&'static str)
}

pub type ModelResult<T> = Result<T, ModelError>;

pub struct Model {
    pub source_files: SourceFiles,
    pub span_table: HashMap<ItemKey, SourceLocation>,

    // Dense arrays of each item type, including polymorphic enum types (Statement, Expression, etc.)
    pub files: ModelItems<File>,
    pub file_items: ModelItems<FileItem>,
    pub type_decls: ModelItems<TypeDecl>,
    pub import_decls: ModelItems<ImportDecl>,
    pub statements: ModelItems<Statement>,
    pub empty_statements: ModelItems<EmptyStatement>,
    pub expression_statements: ModelItems<ExpressionStatement>,
    pub if_statements: ModelItems<IfStatement>,
    pub while_statements: ModelItems<WhileStatement>,
    pub return_statements: ModelItems<ReturnStatement>,
    pub break_statements: ModelItems<BreakStatement>,
    pub continue_statements: ModelItems<ContinueStatement>,
    pub block_statements: ModelItems<BlockStatement>,
    pub for_statements: ModelItems<ForStatement>,
    pub switch_statements: ModelItems<SwitchStatement>,
    pub var_decl_statements: ModelItems<VarDeclStatement>,
    pub function_decl_statements: ModelItems<FunctionDeclStatement>,
    pub labeled_statements: ModelItems<LabeledStatement>,
    pub try_statements: ModelItems<TryStatement>,
    pub catch_clauses: ModelItems<CatchClause>,
    pub expressions: ModelItems<Expression>,
    pub comma_expressions: ModelItems<CommaExpression>,
    pub ternary_expressions: ModelItems<TernaryExpression>,
    pub binary_expressions: ModelItems<BinaryExpression>,
    pub unary_expressions: ModelItems<UnaryExpression>,
    pub boolean_literals: ModelItems<BooleanLiteral>,
    pub null_literals: ModelItems<NullLiteral>,
    pub string_literals: ModelItems<StringLiteral>,
    pub int_literals: ModelItems<IntLiteral>,
    pub member_expressions: ModelItems<MemberExpression>,
    pub identifier_expressions: ModelItems<IdentifierExpression>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            source_files: SourceFiles::new(),
            span_table: HashMap::new(),
            
            files: ModelItems::new(ItemKind::File),
            file_items: ModelItems::new(ItemKind::FileItem),
            type_decls: ModelItems::new(ItemKind::TypeDecl),
            import_decls: ModelItems::new(ItemKind::ImportDecl),
            statements: ModelItems::new(ItemKind::Statement),
            empty_statements: ModelItems::new(ItemKind::EmptyStatement),
            expression_statements: ModelItems::new(ItemKind::ExpressionStatement),
            if_statements: ModelItems::new(ItemKind::IfStatement),
            while_statements: ModelItems::new(ItemKind::WhileStatement),
            return_statements: ModelItems::new(ItemKind::ReturnStatement),
            break_statements: ModelItems::new(ItemKind::BreakStatement),
            continue_statements: ModelItems::new(ItemKind::ContinueStatement),
            block_statements: ModelItems::new(ItemKind::BlockStatement),
            for_statements: ModelItems::new(ItemKind::ForStatement),
            switch_statements: ModelItems::new(ItemKind::SwitchStatement),
            var_decl_statements: ModelItems::new(ItemKind::VarDeclStatement),
            function_decl_statements: ModelItems::new(ItemKind::FunctionDeclStatement),
            labeled_statements: ModelItems::new(ItemKind::LabeledStatement),
            try_statements: ModelItems::new(ItemKind::TryStatement),
            catch_clauses: ModelItems::new(ItemKind::CatchClause),
            expressions: ModelItems::new(ItemKind::Expression),
            comma_expressions: ModelItems::new(ItemKind::CommaExpression),
            ternary_expressions: ModelItems::new(ItemKind::TernaryExpression),
            binary_expressions: ModelItems::new(ItemKind::BinaryExpression),
            unary_expressions: ModelItems::new(ItemKind::UnaryExpression),
            boolean_literals: ModelItems::new(ItemKind::BooleanLiteral),
            null_literals: ModelItems::new(ItemKind::NullLiteral),
            string_literals: ModelItems::new(ItemKind::StringLiteral),
            int_literals: ModelItems::new(ItemKind::IntLiteral),
            member_expressions: ModelItems::new(ItemKind::MemberExpression),
            identifier_expressions: ModelItems::new(ItemKind::IdentifierExpression),
        }
    }
}

pub struct ModelItems<T>
{
    pub items: Vec<T>,
    pub kind: ItemKind
}

impl<T> ModelItems<T> {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            items: Vec::new(),
            kind,
        }
    }

    pub fn add(&mut self, item: T) -> ItemPtr<T> {
        let id = self.items.len();
        self.items.push(item);
        ItemPtr {
            kind: self.kind,
            id,
            _marker: PhantomData,
        }
    }
}

//--------------------------------------------------
// SourceFiles and SourceLocations

pub struct SourceFile {
    pub name: String
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct SourceFilePtr {
    id: usize
}

pub struct SourceFiles {
    source_files: HashMap<SourceFilePtr, SourceFile>,
    id_counter: usize
}

impl SourceFiles {
    pub fn new() -> Self {
        Self {
            source_files: HashMap::new(),
            id_counter: 1,
        }
    }

    pub fn add(&mut self, name: String) -> SourceFilePtr {
        let ret = SourceFilePtr {id: self.id_counter};
        self.source_files.insert(ret, SourceFile {
            name
        });
        ret
    }
}

pub struct SourceLocation {
    pub source_file: SourceFilePtr,
    pub span: Span,
}

pub struct Span {
    // The position of the first character
    pub start: usize,
    // The position *after* the last character
    pub end: usize,
}


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ItemKind {
    File,
    FileItem,
    TypeDecl,
    ImportDecl,
    Statement,
    EmptyStatement,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    ReturnStatement,
    BreakStatement,
    ContinueStatement,
    BlockStatement,
    ForStatement,
    SwitchStatement,
    VarDeclStatement,
    FunctionDeclStatement,
    LabeledStatement,
    TryStatement,
    CatchClause,
    Expression,
    CommaExpression,
    TernaryExpression,
    BinaryExpression,
    UnaryExpression,
    BooleanLiteral,
    NullLiteral,
    StringLiteral,
    IntLiteral,
    MemberExpression,
    IdentifierExpression,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ItemPtr<T> {
    kind: ItemKind,
    id: usize,
    _marker: PhantomData<T>,
}

// Implement Clone and Copy manually (derive is too conservative about requiring T to also be Clone and Copy)
impl<T> Clone for ItemPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ItemPtr<T> {}


impl<T> ItemPtr<T> {
    pub fn key(&self) -> ItemKey {
        ItemKey {kind: self.kind, id: self.id}
    }
}

// Type-erased version of a ptr, for use in tables
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct ItemKey {
    kind: ItemKind,
    id: usize,
}

#[derive(Debug)]
pub struct File {
    pub items: Vec<ItemPtr<FileItem>>
}

#[derive(Debug)]
pub enum FileItem {
    ImportDecl(ItemPtr<ImportDecl>),
    TypeDecl(ItemPtr<TypeDecl>),
    Statement(ItemPtr<Statement>),
}

#[derive(Debug)]
pub struct TypeDecl {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct ImportDecl {
    // FIXME - implement this
}

#[derive(Debug)]
pub enum Statement {
    EmptyStatement(ItemPtr<EmptyStatement>),
    ExpressionStatement(ItemPtr<ExpressionStatement>),
    IfStatement(ItemPtr<IfStatement>),
    WhileStatement(ItemPtr<WhileStatement>),
    ReturnStatement(ItemPtr<ReturnStatement>),
    BreakStatement(ItemPtr<BreakStatement>),
    ContinueStatement(ItemPtr<ContinueStatement>),
    BlockStatement(ItemPtr<BlockStatement>),
    ForStatement(ItemPtr<ForStatement>),
    SwitchStatement(ItemPtr<SwitchStatement>),
    VarDeclStatement(ItemPtr<VarDeclStatement>),
    FunctionDeclStatement(ItemPtr<FunctionDeclStatement>),
    LabeledStatement(ItemPtr<LabeledStatement>),
    TryStatement(ItemPtr<TryStatement>),
}

#[derive(Debug)]
pub struct EmptyStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub exp: ItemPtr<Expression>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub test: ItemPtr<Expression>,
    pub if_true: ItemPtr<Statement>,
    pub if_false: Option<ItemPtr<Statement>>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub test: ItemPtr<Expression>,
    pub stmt: ItemPtr<Statement>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub exp: Option<ItemPtr<Expression>>,
}

#[derive(Debug)]
pub struct BreakStatement {
    pub label: Option<String>
}

#[derive(Debug)]
pub struct ContinueStatement {
    pub label: Option<String>
}

#[derive(Debug)]
pub struct BlockStatement {
    pub stmts: Vec<ItemPtr<Statement>>
}

#[derive(Debug)]
pub struct ForStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct SwitchStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct VarDeclStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct FunctionDeclStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct LabeledStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct TryStatement {
    pub stmt: ItemPtr<Statement>,
    pub catch_clause: Option<ItemPtr<CatchClause>>,
    pub finally_clause: Option<ItemPtr<Statement>>,
}

#[derive(Debug)]
pub struct CatchClause {
    pub name: Option<String>,
    pub stmt: ItemPtr<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    CommaExpression(ItemPtr<CommaExpression>),
    TernaryExpression(ItemPtr<TernaryExpression>),
    BinaryExpression(ItemPtr<BinaryExpression>),
    UnaryExpression(ItemPtr<UnaryExpression>),
    BooleanLiteral(ItemPtr<BooleanLiteral>),
    NullLiteral(ItemPtr<NullLiteral>),
    StringLiteral(ItemPtr<StringLiteral>),
    IntLiteral(ItemPtr<IntLiteral>),
    MemberExpression(ItemPtr<MemberExpression>),
    IdentifierExpression(ItemPtr<IdentifierExpression>),
}

#[derive(Debug)]
pub struct CommaExpression {
    pub exps: Vec<ItemPtr<Expression>>
}

#[derive(Debug)]
pub struct TernaryExpression {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct BinaryExpression {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub op: UnaryOp,
    pub exp: ItemPtr<Expression>
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool
}

#[derive(Debug)]
pub struct NullLiteral {
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String
}

#[derive(Debug)]
pub struct IntLiteral {
    pub value: u64,
    pub suffix: Option<IntLiteralSuffix>,
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

#[derive(Debug)]
pub struct MemberExpression {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct IdentifierExpression {
    pub name: String
}
