//! Dump the model as JSON by walking the domain handles. No external deps.
//!
//! Each handle implements `ToJson`, producing a `Json` value tagged with its
//! `"kind"`. The enum handles (`Statement`, `Expression`, …) just delegate to
//! whichever concrete handle they wrap. `Json` implements `Display`, so:
//!
//! ```ignore
//! let opts = model_json::DumpOpts::with_spans();
//! println!("{}", model_json::file_to_json(&model, file_ptr, &opts));
//! ```

use crate::model::*;

//--------------------------------------------------
// Dump options.

#[derive(Clone, Copy, Default)]
pub struct DumpOpts {
    /// Include each node's `SourceLocation` as a `"span"` field.
    pub spans: bool,
}

impl DumpOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_spans() -> Self {
        Self { spans: true }
    }
}

//--------------------------------------------------
// A minimal JSON value + pretty-printer.

pub enum Json {
    Null,
    Bool(bool),
    Num(i64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(&'static str, Json)>),
}

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.write(f, 0)
    }
}

impl Json {
    fn write(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        fn pad(f: &mut std::fmt::Formatter, n: usize) -> std::fmt::Result {
            write!(f, "{:1$}", "", n * 2)
        }
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{b}"),
            Json::Num(n) => write!(f, "{n}"),
            Json::Str(s) => write_str(f, s),
            Json::Array(xs) if xs.is_empty() => write!(f, "[]"),
            Json::Array(xs) => {
                writeln!(f, "[")?;
                for (i, x) in xs.iter().enumerate() {
                    pad(f, depth + 1)?;
                    x.write(f, depth + 1)?;
                    writeln!(f, "{}", if i + 1 < xs.len() { "," } else { "" })?;
                }
                pad(f, depth)?;
                write!(f, "]")
            }
            Json::Object(kv) if kv.is_empty() => write!(f, "{{}}"),
            Json::Object(kv) => {
                writeln!(f, "{{")?;
                for (i, (k, v)) in kv.iter().enumerate() {
                    pad(f, depth + 1)?;
                    write!(f, "\"{k}\": ")?;
                    v.write(f, depth + 1)?;
                    writeln!(f, "{}", if i + 1 < kv.len() { "," } else { "" })?;
                }
                pad(f, depth)?;
                write!(f, "}}")
            }
        }
    }
}

fn write_str(f: &mut std::fmt::Formatter, s: &str) -> std::fmt::Result {
    write!(f, "\"")?;
    for c in s.chars() {
        match c {
            '"' => write!(f, "\\\"")?,
            '\\' => write!(f, "\\\\")?,
            '\n' => write!(f, "\\n")?,
            '\r' => write!(f, "\\r")?,
            '\t' => write!(f, "\\t")?,
            c => write!(f, "{c}")?,
        }
    }
    write!(f, "\"")
}

//--------------------------------------------------
// ToJson trait + helpers.

pub trait ToJson {
    fn to_json(&self, opts: &DumpOpts) -> Json;
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            Some(v) => v.to_json(opts),
            None => Json::Null,
        }
    }
}

impl ToJson for &str {
    fn to_json(&self, _opts: &DumpOpts) -> Json {
        Json::Str((*self).into())
    }
}

/// A tagged object: `{ "kind": <kind>, ["span": ...,] ...fields }`. The span is
/// injected here (once) when `opts.spans` is set, so individual `to_json` impls
/// just hand over their kind, span, and fields.
fn node(
    kind: &'static str,
    span: Option<&SourceLocation>,
    opts: &DumpOpts,
    mut fields: Vec<(&'static str, Json)>,
) -> Json {
    let mut v = Vec::with_capacity(fields.len() + 2);
    v.push(("kind", Json::Str(kind.into())));
    if opts.spans {
        v.push(("span", span.map(source_location_to_json).unwrap_or(Json::Null)));
    }
    v.append(&mut fields);
    Json::Object(v)
}

/// A source location. Just the file id + char range for now; line/column info
/// can be folded in here later without touching any of the node impls.
fn source_location_to_json(loc: &SourceLocation) -> Json {
    Json::Object(vec![
        ("source_file", Json::Num(loc.source_file.id() as i64)),
        ("start", Json::Num(loc.span.start as i64)),
        ("end", Json::Num(loc.span.end as i64)),
    ])
}

/// An array built from an iterator of nodes.
fn array<T: ToJson>(it: impl Iterator<Item = T>, opts: &DumpOpts) -> Json {
    Json::Array(it.map(|x| x.to_json(opts)).collect())
}

/// A value enum (`BinaryOp`, `UnaryOp`, …) rendered via its derived `Debug`.
fn tag<T: std::fmt::Debug>(v: T) -> Json {
    Json::Str(format!("{v:?}"))
}

/// Entry point: dump a whole file.
pub fn file_to_json(model: &Model, file: ItemPtr<FileData>, opts: &DumpOpts) -> Json {
    model.item(file).to_json(opts)
}

//--------------------------------------------------
// Enum handles: delegate to the wrapped concrete handle (it carries the tag
// and, when requested, the span).

impl<'a> ToJson for FileItem<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            FileItem::ImportDecl(x) => x.to_json(opts),
            FileItem::TypeDecl(x) => x.to_json(opts),
            FileItem::Statement(x) => x.to_json(opts),
        }
    }
}

impl<'a> ToJson for Statement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            Statement::Empty(x) => x.to_json(opts),
            Statement::Expression(x) => x.to_json(opts),
            Statement::If(x) => x.to_json(opts),
            Statement::While(x) => x.to_json(opts),
            Statement::Return(x) => x.to_json(opts),
            Statement::Break(x) => x.to_json(opts),
            Statement::Continue(x) => x.to_json(opts),
            Statement::Block(x) => x.to_json(opts),
            Statement::For(x) => x.to_json(opts),
            Statement::Switch(x) => x.to_json(opts),
            Statement::VarDecl(x) => x.to_json(opts),
            Statement::FunctionDecl(x) => x.to_json(opts),
            Statement::Labeled(x) => x.to_json(opts),
            Statement::Try(x) => x.to_json(opts),
        }
    }
}

impl<'a> ToJson for ForInit<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            ForInit::Expression(x) => x.to_json(opts),
            ForInit::VarDecl(x) => x.to_json(opts),
        }
    }
}

impl<'a> ToJson for SwitchItem<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            SwitchItem::Statement(x) => x.to_json(opts),
            SwitchItem::Case(x) => x.to_json(opts),
            SwitchItem::Default(x) => x.to_json(opts),
        }
    }
}

impl<'a> ToJson for Expression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        match self {
            Expression::Comma(x) => x.to_json(opts),
            Expression::Ternary(x) => x.to_json(opts),
            Expression::Binary(x) => x.to_json(opts),
            Expression::Unary(x) => x.to_json(opts),
            Expression::Boolean(x) => x.to_json(opts),
            Expression::Null(x) => x.to_json(opts),
            Expression::String(x) => x.to_json(opts),
            Expression::Int(x) => x.to_json(opts),
            Expression::DotAccess(x) => x.to_json(opts),
            Expression::IndexAccess(x) => x.to_json(opts),
            Expression::FunctionCall(x) => x.to_json(opts),
            Expression::NonNullAssert(x) => x.to_json(opts),
            Expression::Identifier(x) => x.to_json(opts),
        }
    }
}

//--------------------------------------------------
// File-level items.

impl<'a> ToJson for File<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("File", self.span(), opts, vec![
            ("items", array(self.items(), opts)),
        ])
    }
}

impl<'a> ToJson for TypeDecl<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("TypeDecl", self.span(), opts, vec![])
    }
}

impl<'a> ToJson for ImportDecl<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("ImportDecl", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
            ("source", self.source().to_json(opts)),
        ])
    }
}

//--------------------------------------------------
// Statements.

impl<'a> ToJson for EmptyStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("EmptyStatement", self.span(), opts, vec![])
    }
}

impl<'a> ToJson for ExpressionStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("ExpressionStatement", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for IfStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("IfStatement", self.span(), opts, vec![
            ("test", self.test().to_json(opts)),
            ("if_true", self.if_true().to_json(opts)),
            ("if_false", self.if_false().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for WhileStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("WhileStatement", self.span(), opts, vec![
            ("test", self.test().to_json(opts)),
            ("stmt", self.stmt().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for ReturnStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("ReturnStatement", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for BreakStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("BreakStatement", self.span(), opts, vec![
            ("label", self.label().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for ContinueStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("ContinueStatement", self.span(), opts, vec![
            ("label", self.label().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for BlockStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("BlockStatement", self.span(), opts, vec![
            ("stmts", array(self.stmts(), opts)),
        ])
    }
}

impl<'a> ToJson for ForStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("ForStatement", self.span(), opts, vec![
            ("init", self.init().to_json(opts)),
            ("test", self.test().to_json(opts)),
            ("advance", self.advance().to_json(opts)),
            ("stmt", self.stmt().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for SwitchStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("SwitchStatement", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
            ("items", array(self.items(), opts)),
        ])
    }
}

impl<'a> ToJson for SwitchBodyStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("SwitchBodyStatement", self.span(), opts, vec![
            ("stmt", self.stmt().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for SwitchCase<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("SwitchCase", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for SwitchDefault<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("SwitchDefault", self.span(), opts, vec![])
    }
}

impl<'a> ToJson for VarDeclStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("VarDeclStatement", self.span(), opts, vec![
            ("let_or_const", tag(self.let_or_const())),
            ("name", self.name().to_json(opts)),
            ("init", self.init().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for FunctionDeclStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("FunctionDeclStatement", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
            ("signature", self.signature().to_json(opts)),
            ("body", self.body().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for FunctionSignature<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("FunctionSignature", self.span(), opts, vec![
            ("args", array(self.args(), opts)),
        ])
    }
}

impl<'a> ToJson for FunctionDeclArg<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("FunctionDeclArg", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for LabeledStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("LabeledStatement", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
            ("stmt", self.stmt().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for TryStatement<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("TryStatement", self.span(), opts, vec![
            ("stmt", self.stmt().to_json(opts)),
            ("catch_clause", self.catch_clause().to_json(opts)),
            ("finally_clause", self.finally_clause().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for CatchClause<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("CatchClause", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
            ("stmt", self.stmt().to_json(opts)),
        ])
    }
}

//--------------------------------------------------
// Expressions.

impl<'a> ToJson for CommaExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("CommaExpression", self.span(), opts, vec![
            ("exps", array(self.exps(), opts)),
        ])
    }
}

impl<'a> ToJson for TernaryExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("TernaryExpression", self.span(), opts, vec![
            ("test", self.test().to_json(opts)),
            ("if_true", self.if_true().to_json(opts)),
            ("if_false", self.if_false().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for BinaryExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("BinaryExpression", self.span(), opts, vec![
            ("left", self.left().to_json(opts)),
            ("op", tag(self.op())),
            ("right", self.right().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for UnaryExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("UnaryExpression", self.span(), opts, vec![
            ("op", tag(self.op())),
            ("exp", self.exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for BooleanLiteral<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("BooleanLiteral", self.span(), opts, vec![
            ("value", Json::Bool(self.value())),
        ])
    }
}

impl<'a> ToJson for NullLiteral<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("NullLiteral", self.span(), opts, vec![])
    }
}

impl<'a> ToJson for StringLiteral<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("StringLiteral", self.span(), opts, vec![
            ("value", self.value().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for IntLiteral<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("IntLiteral", self.span(), opts, vec![
            ("value", Json::Num(self.value() as i64)),
            ("suffix", self.suffix().map(tag).unwrap_or(Json::Null)),
        ])
    }
}

impl<'a> ToJson for DotAccessExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("DotAccessExpression", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
            ("name", self.name().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for IndexAccessExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("IndexAccessExpression", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
            ("access_exp", self.access_exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for FunctionCallExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("FunctionCallExpression", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
            ("args", array(self.args(), opts)),
        ])
    }
}

impl<'a> ToJson for NonNullAssertExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("NonNullAssertExpression", self.span(), opts, vec![
            ("exp", self.exp().to_json(opts)),
        ])
    }
}

impl<'a> ToJson for IdentifierExpression<'a> {
    fn to_json(&self, opts: &DumpOpts) -> Json {
        node("IdentifierExpression", self.span(), opts, vec![
            ("name", self.name().to_json(opts)),
        ])
    }
}
