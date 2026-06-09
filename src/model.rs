use std::{collections::HashMap, marker::PhantomData};

//==================================================
// Section 1: Generic model machinery
//==================================================

pub enum ModelError {
    TypeMismatch(&'static str)
}

pub type ModelResult<T> = Result<T, ModelError>;

pub struct Model {
    pub source_files: SourceFiles,
    pub span_table: HashMap<ItemKey, SourceLocation>,

    // Dense arrays of each item type
    pub files: ModelItems<FileData>,
    pub type_decls: ModelItems<TypeDeclData>,
    pub import_decls: ModelItems<ImportDeclData>,
    pub empty_statements: ModelItems<EmptyStatementData>,
    pub expression_statements: ModelItems<ExpressionStatementData>,
    pub if_statements: ModelItems<IfStatementData>,
    pub while_statements: ModelItems<WhileStatementData>,
    pub return_statements: ModelItems<ReturnStatementData>,
    pub break_statements: ModelItems<BreakStatementData>,
    pub continue_statements: ModelItems<ContinueStatementData>,
    pub block_statements: ModelItems<BlockStatementData>,
    pub for_statements: ModelItems<ForStatementData>,
    pub switch_statements: ModelItems<SwitchStatementData>,
    pub switch_body_statements: ModelItems<SwitchBodyStatementData>,
    pub switch_cases: ModelItems<SwitchCaseData>,
    pub switch_defaults: ModelItems<SwitchDefaultData>,
    pub var_decl_statements: ModelItems<VarDeclStatementData>,
    pub function_decl_statements: ModelItems<FunctionDeclStatementData>,
    pub function_signatures: ModelItems<FunctionSignatureData>,
    pub function_decl_args: ModelItems<FunctionDeclArgData>,
    pub labeled_statements: ModelItems<LabeledStatementData>,
    pub try_statements: ModelItems<TryStatementData>,
    pub catch_clauses: ModelItems<CatchClauseData>,
    pub comma_expressions: ModelItems<CommaExpressionData>,
    pub ternary_expressions: ModelItems<TernaryExpressionData>,
    pub binary_expressions: ModelItems<BinaryExpressionData>,
    pub unary_expressions: ModelItems<UnaryExpressionData>,
    pub boolean_literals: ModelItems<BooleanLiteralData>,
    pub null_literals: ModelItems<NullLiteralData>,
    pub string_literals: ModelItems<StringLiteralData>,
    pub int_literals: ModelItems<IntLiteralData>,
    pub dot_access_expressions: ModelItems<DotAccessExpressionData>,
    pub index_access_expressions: ModelItems<IndexAccessExpressionData>,
    pub function_call_expressions: ModelItems<FunctionCallExpressionData>,
    pub non_null_assert_expressions: ModelItems<NonNullAssertExpressionData>,
    pub identifier_expressions: ModelItems<IdentifierExpressionData>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            source_files: SourceFiles::new(),
            span_table: HashMap::new(),

            files: ModelItems::new(ItemKind::File),
            type_decls: ModelItems::new(ItemKind::TypeDecl),
            import_decls: ModelItems::new(ItemKind::ImportDecl),
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
            switch_body_statements: ModelItems::new(ItemKind::SwitchBodyStatement),
            switch_cases: ModelItems::new(ItemKind::SwitchCase),
            switch_defaults: ModelItems::new(ItemKind::SwitchDefault),
            var_decl_statements: ModelItems::new(ItemKind::VarDeclStatement),
            function_decl_statements: ModelItems::new(ItemKind::FunctionDeclStatement),
            function_signatures: ModelItems::new(ItemKind::FunctionSignature),
            function_decl_args: ModelItems::new(ItemKind::FunctionDeclArg),
            labeled_statements: ModelItems::new(ItemKind::LabeledStatement),
            try_statements: ModelItems::new(ItemKind::TryStatement),
            catch_clauses: ModelItems::new(ItemKind::CatchClause),
            comma_expressions: ModelItems::new(ItemKind::CommaExpression),
            ternary_expressions: ModelItems::new(ItemKind::TernaryExpression),
            binary_expressions: ModelItems::new(ItemKind::BinaryExpression),
            unary_expressions: ModelItems::new(ItemKind::UnaryExpression),
            boolean_literals: ModelItems::new(ItemKind::BooleanLiteral),
            null_literals: ModelItems::new(ItemKind::NullLiteral),
            string_literals: ModelItems::new(ItemKind::StringLiteral),
            int_literals: ModelItems::new(ItemKind::IntLiteral),
            dot_access_expressions: ModelItems::new(ItemKind::DotAccessExpression),
            index_access_expressions: ModelItems::new(ItemKind::IndexAccessExpression),
            function_call_expressions: ModelItems::new(ItemKind::FunctionCallExpression),
            non_null_assert_expressions: ModelItems::new(ItemKind::NonNullAssertExpression),
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

    pub fn get(&self, id: usize) -> &T {
        self.items.get(id).unwrap()
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

//--------------------------------------------------
// Item pointers

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
    SwitchBodyStatement,
    SwitchCase,
    SwitchDefault,
    VarDeclStatement,
    FunctionDeclStatement,
    FunctionSignature,
    FunctionDeclArg,
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
    DotAccessExpression,
    IndexAccessExpression,
    FunctionCallExpression,
    NonNullAssertExpression,
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

// Type-erased version of a ptr, for use as a key in hash tables
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct ItemKey {
    kind: ItemKind,
    id: usize,
}

//--------------------------------------------------
// Item handles
//
// A handle is just (model, ptr). The stored data lives in `*Data` structs; the
// clean domain name (e.g. `BlockStatement`) is a type alias for the handle, so
// callers work with `BlockStatement<'a>` and never touch `BlockStatementData`
// directly.

pub struct Item<'a, T> {
    model: &'a Model,
    ptr: ItemPtr<T>,
}

// Handles are just (model, ptr) so they're cheap to copy around.
// Derived manually so we don't require T: Clone/Copy.
impl<'a, T> Clone for Item<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T> Copy for Item<'a, T> {}

/// Maps a marker type `T` (the `*Data` struct) to the dense array that stores
/// it on the `Model`. This is the one bit of glue that lets a handle find its
/// own data.
pub trait ModelItem: Sized {
    fn collection(model: &Model) -> &ModelItems<Self>;
}

impl Model {
    /// Wrap a ptr into a handle that can answer questions about the item.
    pub fn item<T: ModelItem>(&self, ptr: ItemPtr<T>) -> Item<'_, T> {
        Item { model: self, ptr }
    }
}

impl<'a, T: ModelItem> Item<'a, T> {
    /// The underlying stored data. Borrow is tied to the model (`'a`), not to
    /// `self`, so accessors can hand back `&'a` data and owned child handles.
    pub fn data(&self) -> &'a T {
        T::collection(self.model).get(self.ptr.id)
    }

    pub fn model(&self) -> &'a Model {
        self.model
    }

    pub fn ptr(&self) -> ItemPtr<T> {
        self.ptr
    }
}

//--------------------------------------------------
// Macros that generate the domain layer.

// (1) For each `Domain => DomainData => field`, generate:
//       * the domain-name type alias  (`pub type Domain<'a> = Item<'a, DomainData>`)
//       * the `ModelItem` impl that locates `DomainData`'s dense array.
//     `macro_rules!` can't build `DomainData` from `Domain`, so we spell both.
macro_rules! model_items {
    ($($domain:ident => $data:ident => $field:ident),* $(,)?) => {
        $(
            pub type $domain<'a> = Item<'a, $data>;

            impl ModelItem for $data {
                fn collection(model: &Model) -> &ModelItems<Self> {
                    &model.$field
                }
            }
        )*
    };
}

// (2) An enum-handle mirroring a *pure-ptr* dispatch enum whose variant names
// match the item type names (e.g. `StatementData::BlockStatement(ItemPtr<BlockStatementData>)`).
// `$src::$domain` reconstructs the source arm and `$domain<'a>` is the handle.
// Dispatch enums with renamed or non-ptr variants are written by hand instead.
macro_rules! enum_handle {
    ($handle:ident from $src:ident { $($variant:ident => $domain:ident),* $(,)? }) => {
        #[derive(Clone, Copy)]
        pub enum $handle<'a> {
            $($variant($domain<'a>),)*
        }

        impl<'a> $handle<'a> {
            pub fn new(model: &'a Model, value: $src) -> Self {
                match value {
                    $($src::$domain(p) => Self::$variant(model.item(p)),)*
                }
            }
        }
    };
}

//==================================================
// Section 2: Stored data (`*Data`)
//==================================================

#[derive(Debug)]
pub struct FileData {
    pub items: Vec<FileItemData>
}

#[derive(Debug, Copy, Clone)]
pub enum FileItemData {
    ImportDecl(ItemPtr<ImportDeclData>),
    TypeDecl(ItemPtr<TypeDeclData>),
    Statement(StatementData),
}

#[derive(Debug)]
pub struct TypeDeclData {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct ImportDeclData {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Copy, Clone)]
pub enum StatementData {
    EmptyStatement(ItemPtr<EmptyStatementData>),
    ExpressionStatement(ItemPtr<ExpressionStatementData>),
    IfStatement(ItemPtr<IfStatementData>),
    WhileStatement(ItemPtr<WhileStatementData>),
    ReturnStatement(ItemPtr<ReturnStatementData>),
    BreakStatement(ItemPtr<BreakStatementData>),
    ContinueStatement(ItemPtr<ContinueStatementData>),
    BlockStatement(ItemPtr<BlockStatementData>),
    ForStatement(ItemPtr<ForStatementData>),
    SwitchStatement(ItemPtr<SwitchStatementData>),
    VarDeclStatement(ItemPtr<VarDeclStatementData>),
    FunctionDeclStatement(ItemPtr<FunctionDeclStatementData>),
    LabeledStatement(ItemPtr<LabeledStatementData>),
    TryStatement(ItemPtr<TryStatementData>),
}

#[derive(Debug)]
pub struct EmptyStatementData {
}

#[derive(Debug)]
pub struct ExpressionStatementData {
    pub exp: ExpressionData,
}

#[derive(Debug)]
pub struct IfStatementData {
    pub test: ExpressionData,
    pub if_true: StatementData,
    pub if_false: Option<StatementData>,
}

#[derive(Debug)]
pub struct WhileStatementData {
    pub test: ExpressionData,
    pub stmt: StatementData,
}

#[derive(Debug)]
pub struct ReturnStatementData {
    pub exp: Option<ExpressionData>,
}

#[derive(Debug)]
pub struct BreakStatementData {
    pub label: Option<String>
}

#[derive(Debug)]
pub struct ContinueStatementData {
    pub label: Option<String>
}

#[derive(Debug)]
pub struct BlockStatementData {
    pub stmts: Vec<StatementData>
}

#[derive(Debug)]
pub struct ForStatementData {
    pub init: Option<ForInitData>,
    pub test: Option<ExpressionData>,
    pub advance: Option<ExpressionData>,
    pub stmt: StatementData
}

#[derive(Debug, Copy, Clone)]
pub enum ForInitData {
    Expression(ExpressionData),
    VarDecl(StatementData),
}

#[derive(Debug)]
pub struct SwitchStatementData {
    pub exp: ExpressionData,
    pub items: Vec<SwitchItemData>,
}

#[derive(Debug, Copy, Clone)]
pub enum SwitchItemData {
    Statement(ItemPtr<SwitchBodyStatementData>),
    Case(ItemPtr<SwitchCaseData>),
    Default(ItemPtr<SwitchDefaultData>),
}

#[derive(Debug)]
pub struct SwitchBodyStatementData {
    pub stmt: StatementData,
}

#[derive(Debug)]
pub struct SwitchCaseData {
    pub exp: ExpressionData,
}

#[derive(Debug)]
pub struct SwitchDefaultData {
}

#[derive(Debug)]
pub struct VarDeclStatementData {
    pub let_or_const: LetOrConst,
    pub name: String,
    pub init: Option<ExpressionData>,
}

#[derive(Debug, Copy, Clone)]
pub enum LetOrConst {
    Let,
    Const,
}

#[derive(Debug)]
pub struct FunctionDeclStatementData {
    pub name: String,
    pub signature: ItemPtr<FunctionSignatureData>,
    pub body: StatementData,
}

#[derive(Debug)]
pub struct FunctionSignatureData {
    pub args: Vec<ItemPtr<FunctionDeclArgData>>,
}

#[derive(Debug)]
pub struct FunctionDeclArgData {
    pub name: String
}

#[derive(Debug)]
pub struct LabeledStatementData {
    pub name: String,
    pub stmt: StatementData,
}

#[derive(Debug)]
pub struct TryStatementData {
    pub stmt: StatementData,
    pub catch_clause: Option<ItemPtr<CatchClauseData>>,
    pub finally_clause: Option<StatementData>,
}

#[derive(Debug)]
pub struct CatchClauseData {
    pub name: Option<String>,
    pub stmt: StatementData,
}

#[derive(Debug, Copy, Clone)]
pub enum ExpressionData {
    CommaExpression(ItemPtr<CommaExpressionData>),
    TernaryExpression(ItemPtr<TernaryExpressionData>),
    BinaryExpression(ItemPtr<BinaryExpressionData>),
    UnaryExpression(ItemPtr<UnaryExpressionData>),
    BooleanLiteral(ItemPtr<BooleanLiteralData>),
    NullLiteral(ItemPtr<NullLiteralData>),
    StringLiteral(ItemPtr<StringLiteralData>),
    IntLiteral(ItemPtr<IntLiteralData>),
    DotAccessExpression(ItemPtr<DotAccessExpressionData>),
    IndexAccessExpression(ItemPtr<IndexAccessExpressionData>),
    FunctionCallExpression(ItemPtr<FunctionCallExpressionData>),
    NonNullAssertExpression(ItemPtr<NonNullAssertExpressionData>),
    IdentifierExpression(ItemPtr<IdentifierExpressionData>),
}

#[derive(Debug)]
pub struct CommaExpressionData {
    pub exps: Vec<ExpressionData>
}

#[derive(Debug)]
pub struct TernaryExpressionData {
    pub test: ExpressionData,
    pub if_true: ExpressionData,
    pub if_false: ExpressionData,
}

#[derive(Debug)]
pub struct BinaryExpressionData {
    pub left: ExpressionData,
    pub op: BinaryOp,
    pub right: ExpressionData,
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

#[derive(Debug)]
pub struct UnaryExpressionData {
    pub op: UnaryOp,
    pub exp: ExpressionData
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
    LogicalNot,
    BitwiseNot,
}

#[derive(Debug)]
pub struct BooleanLiteralData {
    pub value: bool
}

#[derive(Debug)]
pub struct NullLiteralData {
}

#[derive(Debug)]
pub struct StringLiteralData {
    pub value: String
}

#[derive(Debug)]
pub struct IntLiteralData {
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
pub struct DotAccessExpressionData {
    pub exp: ExpressionData,
    pub name: String,
}

#[derive(Debug)]
pub struct IndexAccessExpressionData {
    pub exp: ExpressionData,
    pub access_exp: ExpressionData,
}

#[derive(Debug)]
pub struct FunctionCallExpressionData {
    pub exp: ExpressionData,
    pub args: Vec<ExpressionData>,
}

#[derive(Debug)]
pub struct NonNullAssertExpressionData {
    pub exp: ExpressionData,
}

#[derive(Debug)]
pub struct IdentifierExpressionData {
    pub name: String
}

//==================================================
// Section 3: Domain model objects
//==================================================

// Domain type aliases + `ModelItem` glue, one line per item type. Adding a new
// item type is an edit here plus its `*Data` struct in Section 2.
model_items! {
    File                    => FileData                    => files,
    TypeDecl                => TypeDeclData                => type_decls,
    ImportDecl              => ImportDeclData              => import_decls,
    EmptyStatement          => EmptyStatementData          => empty_statements,
    ExpressionStatement     => ExpressionStatementData     => expression_statements,
    IfStatement             => IfStatementData             => if_statements,
    WhileStatement          => WhileStatementData          => while_statements,
    ReturnStatement         => ReturnStatementData         => return_statements,
    BreakStatement          => BreakStatementData          => break_statements,
    ContinueStatement       => ContinueStatementData       => continue_statements,
    BlockStatement          => BlockStatementData          => block_statements,
    ForStatement            => ForStatementData            => for_statements,
    SwitchStatement         => SwitchStatementData         => switch_statements,
    SwitchBodyStatement     => SwitchBodyStatementData     => switch_body_statements,
    SwitchCase              => SwitchCaseData              => switch_cases,
    SwitchDefault           => SwitchDefaultData           => switch_defaults,
    VarDeclStatement        => VarDeclStatementData        => var_decl_statements,
    FunctionDeclStatement   => FunctionDeclStatementData   => function_decl_statements,
    FunctionSignature       => FunctionSignatureData       => function_signatures,
    FunctionDeclArg         => FunctionDeclArgData         => function_decl_args,
    LabeledStatement        => LabeledStatementData        => labeled_statements,
    TryStatement            => TryStatementData            => try_statements,
    CatchClause             => CatchClauseData             => catch_clauses,
    CommaExpression         => CommaExpressionData         => comma_expressions,
    TernaryExpression       => TernaryExpressionData       => ternary_expressions,
    BinaryExpression        => BinaryExpressionData        => binary_expressions,
    UnaryExpression         => UnaryExpressionData         => unary_expressions,
    BooleanLiteral          => BooleanLiteralData          => boolean_literals,
    NullLiteral             => NullLiteralData             => null_literals,
    StringLiteral           => StringLiteralData           => string_literals,
    IntLiteral              => IntLiteralData              => int_literals,
    DotAccessExpression     => DotAccessExpressionData     => dot_access_expressions,
    IndexAccessExpression   => IndexAccessExpressionData   => index_access_expressions,
    FunctionCallExpression  => FunctionCallExpressionData  => function_call_expressions,
    NonNullAssertExpression => NonNullAssertExpressionData => non_null_assert_expressions,
    IdentifierExpression    => IdentifierExpressionData    => identifier_expressions,
}

// Pure-ptr dispatch enums whose variant names match the item types.
enum_handle! {
    Statement from StatementData {
        Empty        => EmptyStatement,
        Expression   => ExpressionStatement,
        If           => IfStatement,
        While        => WhileStatement,
        Return       => ReturnStatement,
        Break        => BreakStatement,
        Continue     => ContinueStatement,
        Block        => BlockStatement,
        For          => ForStatement,
        Switch       => SwitchStatement,
        VarDecl      => VarDeclStatement,
        FunctionDecl => FunctionDeclStatement,
        Labeled      => LabeledStatement,
        Try          => TryStatement,
    }
}

enum_handle! {
    Expression from ExpressionData {
        Comma         => CommaExpression,
        Ternary       => TernaryExpression,
        Binary        => BinaryExpression,
        Unary         => UnaryExpression,
        Boolean       => BooleanLiteral,
        Null          => NullLiteral,
        String        => StringLiteral,
        Int           => IntLiteral,
        DotAccess     => DotAccessExpression,
        IndexAccess   => IndexAccessExpression,
        FunctionCall  => FunctionCallExpression,
        NonNullAssert => NonNullAssertExpression,
        Identifier    => IdentifierExpression,
    }
}

// Irregular dispatch enums: renamed variants and/or non-ptr payloads, so the
// macro doesn't fit — written by hand.

#[derive(Clone, Copy)]
pub enum FileItem<'a> {
    ImportDecl(ImportDecl<'a>),
    TypeDecl(TypeDecl<'a>),
    Statement(Statement<'a>),
}

impl<'a> FileItem<'a> {
    pub fn new(model: &'a Model, value: FileItemData) -> Self {
        match value {
            FileItemData::ImportDecl(p) => Self::ImportDecl(model.item(p)),
            FileItemData::TypeDecl(p) => Self::TypeDecl(model.item(p)),
            FileItemData::Statement(s) => Self::Statement(Statement::new(model, s)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum ForInit<'a> {
    Expression(Expression<'a>),
    VarDecl(Statement<'a>),
}

impl<'a> ForInit<'a> {
    pub fn new(model: &'a Model, value: ForInitData) -> Self {
        match value {
            ForInitData::Expression(e) => Self::Expression(Expression::new(model, e)),
            ForInitData::VarDecl(s) => Self::VarDecl(Statement::new(model, s)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum SwitchItem<'a> {
    Statement(SwitchBodyStatement<'a>),
    Case(SwitchCase<'a>),
    Default(SwitchDefault<'a>),
}

impl<'a> SwitchItem<'a> {
    pub fn new(model: &'a Model, value: SwitchItemData) -> Self {
        match value {
            SwitchItemData::Statement(p) => Self::Statement(model.item(p)),
            SwitchItemData::Case(p) => Self::Case(model.item(p)),
            SwitchItemData::Default(p) => Self::Default(model.item(p)),
        }
    }
}

//--------------------------------------------------
// Accessors — one impl per item type. Written on the domain alias.
// These are the hand-written, domain-shaped layer; add domain logic here.

impl<'a> File<'a> {
    pub fn items(&self) -> impl Iterator<Item = FileItem<'a>> + 'a {
        let model = self.model();
        self.data().items.iter().map(move |&i| FileItem::new(model, i))
    }
}

// TypeDecl: no fields yet (FIXME in TypeDeclData).

impl<'a> ImportDecl<'a> {
    pub fn name(&self) -> &'a str { &self.data().name }
    pub fn source(&self) -> &'a str { &self.data().source }
}

// EmptyStatement: no fields.

impl<'a> ExpressionStatement<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
}

impl<'a> IfStatement<'a> {
    pub fn test(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().test)
    }
    pub fn if_true(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().if_true)
    }
    pub fn if_false(&self) -> Option<Statement<'a>> {
        let model = self.model();
        self.data().if_false.map(|s| Statement::new(model, s))
    }
}

impl<'a> WhileStatement<'a> {
    pub fn test(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().test)
    }
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
}

impl<'a> ReturnStatement<'a> {
    pub fn exp(&self) -> Option<Expression<'a>> {
        let model = self.model();
        self.data().exp.map(|e| Expression::new(model, e))
    }
}

impl<'a> BreakStatement<'a> {
    pub fn label(&self) -> Option<&'a str> {
        self.data().label.as_deref()
    }
}

impl<'a> ContinueStatement<'a> {
    pub fn label(&self) -> Option<&'a str> {
        self.data().label.as_deref()
    }
}

impl<'a> BlockStatement<'a> {
    pub fn stmts(&self) -> impl Iterator<Item = Statement<'a>> + 'a {
        let model = self.model();
        self.data().stmts.iter().map(move |&s| Statement::new(model, s))
    }
}

impl<'a> ForStatement<'a> {
    pub fn init(&self) -> Option<ForInit<'a>> {
        let model = self.model();
        self.data().init.map(|i| ForInit::new(model, i))
    }
    pub fn test(&self) -> Option<Expression<'a>> {
        let model = self.model();
        self.data().test.map(|e| Expression::new(model, e))
    }
    pub fn advance(&self) -> Option<Expression<'a>> {
        let model = self.model();
        self.data().advance.map(|e| Expression::new(model, e))
    }
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
}

impl<'a> SwitchStatement<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
    pub fn items(&self) -> impl Iterator<Item = SwitchItem<'a>> + 'a {
        let model = self.model();
        self.data().items.iter().map(move |&i| SwitchItem::new(model, i))
    }
}

impl<'a> SwitchBodyStatement<'a> {
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
}

impl<'a> SwitchCase<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
}

// SwitchDefault: no fields.

impl<'a> VarDeclStatement<'a> {
    pub fn let_or_const(&self) -> LetOrConst { self.data().let_or_const }
    pub fn name(&self) -> &'a str { &self.data().name }
    pub fn init(&self) -> Option<Expression<'a>> {
        let model = self.model();
        self.data().init.map(|e| Expression::new(model, e))
    }
}

impl<'a> FunctionDeclStatement<'a> {
    pub fn name(&self) -> &'a str { &self.data().name }
    pub fn signature(&self) -> FunctionSignature<'a> {
        self.model().item(self.data().signature)
    }
    pub fn body(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().body)
    }
}

impl<'a> FunctionSignature<'a> {
    pub fn args(&self) -> impl Iterator<Item = FunctionDeclArg<'a>> + 'a {
        let model = self.model();
        self.data().args.iter().map(move |&ptr| model.item(ptr))
    }
}

impl<'a> FunctionDeclArg<'a> {
    pub fn name(&self) -> &'a str { &self.data().name }
}

impl<'a> LabeledStatement<'a> {
    pub fn name(&self) -> &'a str { &self.data().name }
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
}

impl<'a> TryStatement<'a> {
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
    pub fn catch_clause(&self) -> Option<CatchClause<'a>> {
        let model = self.model();
        self.data().catch_clause.map(|p| model.item(p))
    }
    pub fn finally_clause(&self) -> Option<Statement<'a>> {
        let model = self.model();
        self.data().finally_clause.map(|s| Statement::new(model, s))
    }
}

impl<'a> CatchClause<'a> {
    pub fn name(&self) -> Option<&'a str> {
        self.data().name.as_deref()
    }
    pub fn stmt(&self) -> Statement<'a> {
        Statement::new(self.model(), self.data().stmt)
    }
}

impl<'a> CommaExpression<'a> {
    pub fn exps(&self) -> impl Iterator<Item = Expression<'a>> + 'a {
        let model = self.model();
        self.data().exps.iter().map(move |&e| Expression::new(model, e))
    }
}

impl<'a> TernaryExpression<'a> {
    pub fn test(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().test)
    }
    pub fn if_true(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().if_true)
    }
    pub fn if_false(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().if_false)
    }
}

impl<'a> BinaryExpression<'a> {
    pub fn left(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().left)
    }
    pub fn op(&self) -> BinaryOp { self.data().op }
    pub fn right(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().right)
    }
}

impl<'a> UnaryExpression<'a> {
    pub fn op(&self) -> UnaryOp { self.data().op }
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
}

impl<'a> BooleanLiteral<'a> {
    pub fn value(&self) -> bool { self.data().value }
}

// NullLiteral: no fields.

impl<'a> StringLiteral<'a> {
    pub fn value(&self) -> &'a str { &self.data().value }
}

impl<'a> IntLiteral<'a> {
    pub fn value(&self) -> u64 { self.data().value }
    pub fn suffix(&self) -> Option<IntLiteralSuffix> { self.data().suffix }
}

impl<'a> DotAccessExpression<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
    pub fn name(&self) -> &'a str { &self.data().name }
}

impl<'a> IndexAccessExpression<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
    pub fn access_exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().access_exp)
    }
}

impl<'a> FunctionCallExpression<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
    pub fn args(&self) -> impl Iterator<Item = Expression<'a>> + 'a {
        let model = self.model();
        self.data().args.iter().map(move |&e| Expression::new(model, e))
    }
}

impl<'a> NonNullAssertExpression<'a> {
    pub fn exp(&self) -> Expression<'a> {
        Expression::new(self.model(), self.data().exp)
    }
}

impl<'a> IdentifierExpression<'a> {
    pub fn name(&self) -> &'a str { &self.data().name }
}
