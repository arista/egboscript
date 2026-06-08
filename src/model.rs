use std::{marker::PhantomData};

pub enum ModelError {
    TypeMismatch(&'static str)
}

pub type ModelResult<T> = Result<T, ModelError>;

pub struct Model {
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

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct ItemPtr<T> {
    kind: ItemKind,
    id: usize,
    _marker: PhantomData<T>,
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
    // FIXME - implement this
}

#[derive(Debug)]
pub struct IfStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct WhileStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct ReturnStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct BreakStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct ContinueStatement {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct BlockStatement {
    // FIXME - implement this
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
    // FIXME - implement this
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
    // FIXME - implement this
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
    // FIXME - implement this
}

#[derive(Debug)]
pub struct BooleanLiteral {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct NullLiteral {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct StringLiteral {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct IntLiteral {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct MemberExpression {
    // FIXME - implement this
}

#[derive(Debug)]
pub struct IdentifierExpression {
    // FIXME - implement this
}











// #[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
// pub struct IntLiteral {
//     value: u32,
// }

// #[derive(Eq, Hash, PartialEq, Clone, Copy)]
// pub struct Id {
//     pub id: u32
// }

// pub struct Handle<'a, T> {
//     pub model: &'a Model,
//     pub id: Id,

//     // To allow the <T> even if it hasn't been used yet in a field
//     _marker: PhantomData<T>,
// }

// pub enum Item {
//     IntLiteral(IntLiteral),
//     BooleanLiteral(BooleanLiteral),
//     StringLiteral(StringLiteral),
//     NullLiteral(NullLiteral),
// }

// pub enum Expression {
//     IntLiteral(IntLiteral),
//     BooleanLiteral(BooleanLiteral),
//     StringLiteral(StringLiteral),
//     NullLiteral(NullLiteral),
// }

// impl Item {
//     pub fn int_literal(id: Id, value: u32) -> Self {
//         Self::IntLiteral(IntLiteral {id, value})
//     }
    
//     pub fn as_int_literal(&self) -> ModelResult<&IntLiteral> {
//         match self {
//             Self::IntLiteral(r) => Ok(r),
//             _ => Err(ModelError::TypeMismatch("Item is not an IntLiteral"))
//         }
//     }

//     pub fn string_literal(id: Id, value: String) -> Self {
//         Self::StringLiteral(StringLiteral {id, value})
//     }
    
//     pub fn as_string_literal(&self) -> ModelResult<&StringLiteral> {
//         match self {
//             Self::StringLiteral(r) => Ok(r),
//             _ => Err(ModelError::TypeMismatch("Item is not a StringLiteral"))
//         }
//     }

//     pub fn boolean_literal(id: Id, value: bool) -> Self {
//         Self::BooleanLiteral(BooleanLiteral {id, value})
//     }
    
//     pub fn as_boolean_literal(&self) -> ModelResult<&BooleanLiteral> {
//         match self {
//             Self::BooleanLiteral(r) => Ok(r),
//             _ => Err(ModelError::TypeMismatch("Item is not a BooleanLiteral"))
//         }
//     }

//     pub fn null_literal(id: Id) -> Self {
//         Self::NullLiteral(NullLiteral {id})
//     }
    
//     pub fn as_null_literal(&self) -> ModelResult<&NullLiteral> {
//         match self {
//             Self::NullLiteral(r) => Ok(r),
//             _ => Err(ModelError::TypeMismatch("Item is not a NullLiteral"))
//         }
//     }
// }

// //----------------------------------------
// // NullLiteral

// pub struct NullLiteral {
//     pub id: Id,
// }

// impl<'a> Handle<'a, NullLiteral> {
//     pub fn item(&self) -> ModelResult<&NullLiteral> {
//         self.model.get_item(&self.id)?.as_null_literal()
//     }
// }

// //----------------------------------------
// // IntLiteral

// pub struct IntLiteral {
//     pub id: Id,
//     pub value: u32,
// }

// impl<'a> Handle<'a, IntLiteral> {
//     pub fn item(&self) -> ModelResult<&IntLiteral> {
//         self.model.get_item(&self.id)?.as_int_literal()
//     }

//     pub fn value(&self) -> ModelResult<u32> {
//         Ok(self.item()?.value)
//     }
// }

// //----------------------------------------
// // BooleanLiteral

// pub struct BooleanLiteral {
//     pub id: Id,
//     pub value: bool,
// }

// impl<'a> Handle<'a, BooleanLiteral> {
//     pub fn item(&self) -> ModelResult<&BooleanLiteral> {
//         self.model.get_item(&self.id)?.as_boolean_literal()
//     }

//     pub fn value(&self) -> ModelResult<bool> {
//         Ok(self.item()?.value)
//     }
// }

// //----------------------------------------
// // StringLiteral

// pub struct StringLiteral {
//     pub id: Id,
//     pub value: String,
// }

// impl<'a> Handle<'a, StringLiteral> {
//     pub fn item(&self) -> ModelResult<&StringLiteral> {
//         self.model.get_item(&self.id)?.as_string_literal()
//     }

//     pub fn value(&self) -> ModelResult<&String> {
//         Ok(&self.item()?.value)
//     }
// }
