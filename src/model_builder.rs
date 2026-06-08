use crate::ast;
use crate::model;
use crate::peg_parser::{Parsed, ParsedRange};


pub fn add_file_to_model(source_file: &String, src: &Parsed<ast::File>, model: &mut model::Model) -> model::ItemPtr<model::File> {
    let source_file = model.source_files.add(source_file.clone());
    let mut mb = ModelBuilder::new(model, source_file);
    mb.build_file(&src.value, &src.range)
}

struct ModelBuilder<'a> {
    model: &'a mut model::Model,
    source_file: model::SourceFilePtr,
}

impl<'a> ModelBuilder<'a> {
    pub fn new(model: &'a mut model::Model, source_file: model::SourceFilePtr) -> Self {
        Self {
            model,
            source_file,
        }
    }

    fn build_item<T, M, F, MIF>(&mut self, item: &T, range: &ParsedRange, model_items_f: MIF, f: F) -> model::ItemPtr<M>
    where
        F: Fn(&T, &mut Self) -> M,
        MIF: Fn(&mut model::Model) -> &mut model::ModelItems<M>,
    {
        let m = f(item, self);
        let model_items = model_items_f(self.model);
        let ptr = model_items.add(m);
        let source_location = model::SourceLocation {
            source_file: self.source_file,
            span: model:: Span {
                start: range.start,
                end: range.end,
            },
        };
        self.model.span_table.insert(ptr.key(), source_location);
        ptr
    }
    
    pub fn build_file(&mut self, src: &ast::File, range: &ParsedRange) -> model::ItemPtr<model::File> {
        self.build_item(src, range, |m| &mut m.files, |v, mb| {
            model::File {
                items: v.items.iter().map(|v| mb.build_file_item(&v.value, &v.range)).collect(),
            }
        })
    }
    
    pub fn build_file_item(&mut self, src: &ast::FileItem, range: &ParsedRange) -> model::ItemPtr<model::FileItem> {
        self.build_item(src, range, |m| &mut m.file_items, |v, mb| {
            match v {
                ast::FileItem::Statement(a) => model::FileItem::Statement(mb.build_statement(a, range)),
                ast::FileItem::TypeDecl(a) => model::FileItem::TypeDecl(mb.build_type_decl(a, range)),
                ast::FileItem::ImportDecl(a) => model::FileItem::ImportDecl(mb.build_import_decl(a, range)),
            }
        })
    }
    
    pub fn build_statement(&mut self, src: &ast::Statement, range: &ParsedRange) -> model::ItemPtr<model::Statement> {
        self.build_item(src, range, |m| &mut m.statements, |v, mb| {
            match v {
                ast::Statement::EmptyStatement => model::Statement::EmptyStatement(mb.build_empty_statement(range)),
                ast::Statement::ExpressionStatement(a) => model::Statement::ExpressionStatement(mb.build_expression_statement(a, range)),
                ast::Statement::IfStatement(a) => model::Statement::IfStatement(mb.build_if_statement(a, range)),
                ast::Statement::WhileStatement(a) => model::Statement::WhileStatement(mb.build_while_statement(a, range)),
                ast::Statement::ReturnStatement(a) => model::Statement::ReturnStatement(mb.build_return_statement(a, range)),
                ast::Statement::BreakStatement(a) => model::Statement::BreakStatement(mb.build_break_statement(a, range)),
                ast::Statement::ContinueStatement(a) => model::Statement::ContinueStatement(mb.build_continue_statement(a, range)),
                ast::Statement::BlockStatement(a) => model::Statement::BlockStatement(mb.build_block_statement(a, range)),
                ast::Statement::ForStatement(a) => model::Statement::ForStatement(mb.build_for_statement(a, range)),
                ast::Statement::SwitchStatement(a) => model::Statement::SwitchStatement(mb.build_switch_statement(a, range)),
                ast::Statement::VarDeclStatement(a) => model::Statement::VarDeclStatement(mb.build_var_decl_statement(a, range)),
                ast::Statement::FunctionDeclStatement(a) => model::Statement::FunctionDeclStatement(mb.build_function_decl_statement(a, range)),
                ast::Statement::LabeledStatement(a) => model::Statement::LabeledStatement(mb.build_labeled_statement(a, range)),
                ast::Statement::TryStatement(a) => model::Statement::TryStatement(mb.build_try_statement(a, range)),
            }
        })
    }
    
    pub fn build_type_decl(&mut self, src: &ast::TypeDecl, range: &ParsedRange) -> model::ItemPtr<model::TypeDecl> {
        self.build_item(src, range, |m| &mut m.type_decls, |_v, _mb| {
            model::TypeDecl {
                // FIXME - implement this
            }
        })
    }
    
    pub fn build_import_decl(&mut self, src: &ast::ImportDecl, range: &ParsedRange) -> model::ItemPtr<model::ImportDecl> {
        self.build_item(src, range, |m| &mut m.import_decls, |_v, _mb| {
            model::ImportDecl {
                // FIXME - implement this
            }
        })
    }

    pub fn build_empty_statement(&mut self, range: &ParsedRange) -> model::ItemPtr<model::EmptyStatement> {
        self.build_item(&(), range, |m| &mut m.empty_statements, |_v, _mb| {
            model::EmptyStatement {
            }
        })
    }
            
    pub fn build_expression_statement(&mut self, src: &ast::ExpressionStatement, range: &ParsedRange) -> model::ItemPtr<model::ExpressionStatement> {
        self.build_item(src, range, |m| &mut m.expression_statements, |v, mb| {
            model::ExpressionStatement {
                exp: mb.build_expression(&v.exp.value, &v.exp.range)
            }
        })
    }
            
    pub fn build_if_statement(&mut self, src: &ast::IfStatement, range: &ParsedRange) -> model::ItemPtr<model::IfStatement> {
        self.build_item(src, range, |m| &mut m.if_statements, |v, mb| {
            model::IfStatement {
                test: mb.build_expression(&v.test.value, &v.test.range),
                if_true: mb.build_statement(&v.if_true.value, &v.if_true.range),
                if_false: v.if_false.as_ref().map(|v| mb.build_statement(&v.value, &v.range)),
            }
        })
    }
            
    pub fn build_while_statement(&mut self, src: &ast::WhileStatement, range: &ParsedRange) -> model::ItemPtr<model::WhileStatement> {
        self.build_item(src, range, |m| &mut m.while_statements, |v, mb| {
            model::WhileStatement {
                test: mb.build_expression(&v.test.value, &v.test.range),
                stmt: mb.build_statement(&v.stmt.value, &v.stmt.range),
            }
        })
    }
            
    pub fn build_return_statement(&mut self, src: &ast::ReturnStatement, range: &ParsedRange) -> model::ItemPtr<model::ReturnStatement> {
        self.build_item(src, range, |m| &mut m.return_statements, |v, mb| {
            model::ReturnStatement {
                exp: v.exp.as_ref().map(|v| mb.build_expression(&v.value, &v.range)),
            }
        })
    }
            
    pub fn build_break_statement(&mut self, src: &ast::BreakStatement, range: &ParsedRange) -> model::ItemPtr<model::BreakStatement> {
        self.build_item(src, range, |m| &mut m.break_statements, |v, _mb| {
            model::BreakStatement {
                label: v.label.as_ref().map(|v| v.value.clone()),
            }
        })
    }
            
    pub fn build_continue_statement(&mut self, src: &ast::ContinueStatement, range: &ParsedRange) -> model::ItemPtr<model::ContinueStatement> {
        self.build_item(src, range, |m| &mut m.continue_statements, |v, _mb| {
            model::ContinueStatement {
                label: v.label.as_ref().map(|v| v.value.clone()),
            }
        })
    }
            
    pub fn build_block_statement(&mut self, src: &ast::BlockStatement, range: &ParsedRange) -> model::ItemPtr<model::BlockStatement> {
        self.build_item(src, range, |m| &mut m.block_statements, |v, mb| {
            model::BlockStatement {
                stmts: v.stmts.value.iter().map(|v| mb.build_statement(&v.value, &v.range)).collect()
            }
        })
    }
            
    pub fn build_for_statement(&mut self, src: &ast::ForStatement, range: &ParsedRange) -> model::ItemPtr<model::ForStatement> {
        self.build_item(src, range, |m| &mut m.for_statements, |_v, _mb| {
            model::ForStatement {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_switch_statement(&mut self, src: &ast::SwitchStatement, range: &ParsedRange) -> model::ItemPtr<model::SwitchStatement> {
        self.build_item(src, range, |m| &mut m.switch_statements, |_v, _mb| {
            model::SwitchStatement {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_var_decl_statement(&mut self, src: &ast::VarDeclStatement, range: &ParsedRange) -> model::ItemPtr<model::VarDeclStatement> {
        self.build_item(src, range, |m| &mut m.var_decl_statements, |_v, _mb| {
            model::VarDeclStatement {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_function_decl_statement(&mut self, src: &ast::FunctionDeclStatement, range: &ParsedRange) -> model::ItemPtr<model::FunctionDeclStatement> {
        self.build_item(src, range, |m| &mut m.function_decl_statements, |_v, _mb| {
            model::FunctionDeclStatement {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_labeled_statement(&mut self, src: &ast::LabeledStatement, range: &ParsedRange) -> model::ItemPtr<model::LabeledStatement> {
        self.build_item(src, range, |m| &mut m.labeled_statements, |v, mb| {
            model::LabeledStatement {
                name: src.name.value.clone(),
                stmt: mb.build_statement(&v.stmt.value, &v.stmt.range),
            }
        })
    }
            
    pub fn build_try_statement(&mut self, src: &ast::TryStatement, range: &ParsedRange) -> model::ItemPtr<model::TryStatement> {
        self.build_item(src, range, |m| &mut m.try_statements, |v, mb| {
            model::TryStatement {
                stmt: mb.build_statement(&v.stmt.value, &v.stmt.range),
                catch_clause: v.catch_clause.as_ref().map(|v| mb.build_catch_clause(&v.value, &v.range)),
                finally_clause: v.finally_clause.as_ref().map(|v| mb.build_statement(&v.value, &v.range)),
            }
        })
    }
            
    pub fn build_catch_clause(&mut self, src: &ast::CatchClause, range: &ParsedRange) -> model::ItemPtr<model::CatchClause> {
        self.build_item(src, range, |m| &mut m.catch_clauses, |v, mb| {
            model::CatchClause {
                name: src.name.as_ref().map(|v| v.value.clone()),
                stmt: mb.build_statement(&v.stmt.value, &v.stmt.range),
            }
        })
    }
            
    pub fn build_expression(&mut self, src: &ast::Expression, range: &ParsedRange) -> model::ItemPtr<model::Expression> {
        self.build_item(src, range, |m| &mut m.expressions, |v, mb| {
            match v {
                ast::Expression::CommaExpression(a) => model::Expression::CommaExpression(mb.build_comma_expression(a, range)),
                ast::Expression::TernaryExpression(a) => model::Expression::TernaryExpression(mb.build_ternary_expression(a, range)),
                ast::Expression::BinaryExpression(a) => model::Expression::BinaryExpression(mb.build_binary_expression(a, range)),
                ast::Expression::UnaryExpression(a) => model::Expression::UnaryExpression(mb.build_unary_expression(a, range)),
                ast::Expression::BooleanLiteral(a) => model::Expression::BooleanLiteral(mb.build_boolean_literal(a, range)),
                ast::Expression::NullLiteral => model::Expression::NullLiteral(mb.build_null_literal(range)),
                ast::Expression::StringLiteral(a) => model::Expression::StringLiteral(mb.build_string_literal(a, range)),
                ast::Expression::IntLiteral(a) => model::Expression::IntLiteral(mb.build_int_literal(a, range)),
                ast::Expression::MemberExpression(a) => model::Expression::MemberExpression(mb.build_member_expression(a, range)),
                ast::Expression::IdentifierExpression(a) => model::Expression::IdentifierExpression(mb.build_identifier_expression(a, range)),
            }
        })
    }
            
    pub fn build_comma_expression(&mut self, src: &ast::CommaExpression, range: &ParsedRange) -> model::ItemPtr<model::CommaExpression> {
        self.build_item(src, range, |m| &mut m.comma_expressions, |v, mb| {
            model::CommaExpression {
                exps: v.exps.iter().map(|v| mb.build_expression(&v.value, &v.range)).collect()
            }
        })
    }
            
    pub fn build_ternary_expression(&mut self, src: &ast::TernaryExpression, range: &ParsedRange) -> model::ItemPtr<model::TernaryExpression> {
        self.build_item(src, range, |m| &mut m.ternary_expressions, |_v, _mb| {
            model::TernaryExpression {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_binary_expression(&mut self, src: &ast::BinaryExpression, range: &ParsedRange) -> model::ItemPtr<model::BinaryExpression> {
        self.build_item(src, range, |m| &mut m.binary_expressions, |_v, _mb| {
            model::BinaryExpression {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_unary_expression(&mut self, src: &ast::UnaryExpression, _range: &ParsedRange) -> model::ItemPtr<model::UnaryExpression> {
        let exp = self.build_expression(&src.exp.value, &src.exp.range);
        self.build_one_unary_expression(&src.ops, 0, &exp)
    }
            
    fn build_one_unary_expression(&mut self, ops: &Vec<Parsed<ast::UnaryOp>>, ix: usize, exp: &model::ItemPtr<model::Expression>) -> model::ItemPtr<model::UnaryExpression> {
        let op = ops.get(ix).unwrap();
        let model_op = self.build_unary_op(&op.value);
        self.build_item(&(), &op.range, |m| &mut m.unary_expressions, |_v, mb| {
            if ix == ops.len() - 1 {
                model::UnaryExpression {
                    op: model_op,
                    exp: *exp,
                }
            }
            else {
                let e = mb.build_one_unary_expression(ops, ix + 1, exp);
                // Build an Expression around the UnaryExpression
                let eexp = mb.build_item(&e, &op.range, |m| &mut m.expressions, |v, _mb| {
                    model::Expression::UnaryExpression(v.clone())
                });
                model::UnaryExpression {
                    op: model_op,
                    exp: eexp,
                }
            }
        })
    }

    pub fn build_unary_op(&mut self, src: &ast::UnaryOp) -> model::UnaryOp {
        match src {
            ast::UnaryOp::Plus => model::UnaryOp::Plus,
            ast::UnaryOp::Minus => model::UnaryOp::Minus,
            ast::UnaryOp::LogicalNot => model::UnaryOp::LogicalNot,
            ast::UnaryOp::BitwiseNot => model::UnaryOp::BitwiseNot,
        }
    }
    
    pub fn build_boolean_literal(&mut self, src: &bool, range: &ParsedRange) -> model::ItemPtr<model::BooleanLiteral> {
        self.build_item(src, range, |m| &mut m.boolean_literals, |v, _mb| {
            model::BooleanLiteral {
                value: *v,
            }
        })
    }
            
    pub fn build_null_literal(&mut self, range: &ParsedRange) -> model::ItemPtr<model::NullLiteral> {
        self.build_item(&(), range, |m| &mut m.null_literals, |_v, _mb| {
            model::NullLiteral {
            }
        })
    }
            
    pub fn build_string_literal(&mut self, src: &String, range: &ParsedRange) -> model::ItemPtr<model::StringLiteral> {
        self.build_item(src, range, |m| &mut m.string_literals, |v, _mb| {
            model::StringLiteral {
                value: v.clone(),
            }
        })
    }
            
    pub fn build_int_literal(&mut self, src: &ast::IntLiteral, range: &ParsedRange) -> model::ItemPtr<model::IntLiteral> {
        self.build_item(src, range, |m| &mut m.int_literals, |v, mb| {
            model::IntLiteral {
                value: v.value,
                suffix: v.suffix.as_ref().map(|v| mb.build_int_literal_suffix(&v.value)),
            }
        })
    }
            
    pub fn build_int_literal_suffix(&mut self, src: &ast::IntLiteralSuffix) -> model::IntLiteralSuffix {
        match src {
            ast::IntLiteralSuffix::U8 => model::IntLiteralSuffix::U8,
            ast::IntLiteralSuffix::U16 => model::IntLiteralSuffix::U16,
            ast::IntLiteralSuffix::U32 => model::IntLiteralSuffix::U32,
            ast::IntLiteralSuffix::U64 => model::IntLiteralSuffix::U64,
            ast::IntLiteralSuffix::I8 => model::IntLiteralSuffix::I8,
            ast::IntLiteralSuffix::I16 => model::IntLiteralSuffix::I16,
            ast::IntLiteralSuffix::I32 => model::IntLiteralSuffix::I32,
            ast::IntLiteralSuffix::I64 => model::IntLiteralSuffix::I64,
        }
    }
            
    pub fn build_member_expression(&mut self, src: &ast::MemberExpression, range: &ParsedRange) -> model::ItemPtr<model::MemberExpression> {
        self.build_item(src, range, |m| &mut m.member_expressions, |_v, _mb| {
            model::MemberExpression {
                // FIXME - implement this
            }
        })
    }
            
    pub fn build_identifier_expression(&mut self, src: &ast::IdentifierExpression, range: &ParsedRange) -> model::ItemPtr<model::IdentifierExpression> {
        self.build_item(src, range, |m| &mut m.identifier_expressions, |v, _mb| {
            model::IdentifierExpression {
                name: v.name.clone(),
            }
        })
    }
}
