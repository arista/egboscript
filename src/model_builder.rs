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
    
    pub fn build_file_item(&mut self, src: &ast::FileItem, range: &ParsedRange) -> model::FileItem {
        match src {
            ast::FileItem::Statement(a) => model::FileItem::Statement(self.build_statement(&a, range)),
            ast::FileItem::TypeDecl(a) => model::FileItem::TypeDecl(self.build_type_decl(&a, range)),
            ast::FileItem::ImportDecl(a) => model::FileItem::ImportDecl(self.build_import_decl(&a, range)),
        }
    }
    
    pub fn build_statement(&mut self, src: &ast::Statement, range: &ParsedRange) -> model::Statement {
        match src {
            ast::Statement::EmptyStatement => model::Statement::EmptyStatement(self.build_empty_statement(range)),
            ast::Statement::ExpressionStatement(a) => model::Statement::ExpressionStatement(self.build_expression_statement(&a, range)),
            ast::Statement::IfStatement(a) => model::Statement::IfStatement(self.build_if_statement(&a, range)),
            ast::Statement::WhileStatement(a) => model::Statement::WhileStatement(self.build_while_statement(&a, range)),
            ast::Statement::ReturnStatement(a) => model::Statement::ReturnStatement(self.build_return_statement(&a, range)),
            ast::Statement::BreakStatement(a) => model::Statement::BreakStatement(self.build_break_statement(&a, range)),
            ast::Statement::ContinueStatement(a) => model::Statement::ContinueStatement(self.build_continue_statement(&a, range)),
            ast::Statement::BlockStatement(a) => model::Statement::BlockStatement(self.build_block_statement(&a, range)),
            ast::Statement::ForStatement(a) => model::Statement::ForStatement(self.build_for_statement(&a, range)),
            ast::Statement::SwitchStatement(a) => model::Statement::SwitchStatement(self.build_switch_statement(&a, range)),
            ast::Statement::VarDeclStatement(a) => model::Statement::VarDeclStatement(self.build_var_decl_statement(&a, range)),
            ast::Statement::FunctionDeclStatement(a) => model::Statement::FunctionDeclStatement(self.build_function_decl_statement(&a, range)),
            ast::Statement::LabeledStatement(a) => model::Statement::LabeledStatement(self.build_labeled_statement(&a, range)),
            ast::Statement::TryStatement(a) => model::Statement::TryStatement(self.build_try_statement(&a, range)),
        }
    }
    
    pub fn build_type_decl(&mut self, src: &ast::TypeDecl, range: &ParsedRange) -> model::ItemPtr<model::TypeDecl> {
        self.build_item(src, range, |m| &mut m.type_decls, |_v, _mb| {
            model::TypeDecl {
                // FIXME - implement this
            }
        })
    }
    
    pub fn build_import_decl(&mut self, src: &ast::ImportDecl, range: &ParsedRange) -> model::ItemPtr<model::ImportDecl> {
        self.build_item(src, range, |m| &mut m.import_decls, |v, _mb| {
            model::ImportDecl {
                name: v.name.value.clone(),
                source: v.source.value.clone(),
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
        self.build_item(src, range, |m| &mut m.for_statements, |v, mb| {
            model::ForStatement {
                init: v.init.as_ref().map(|v| mb.build_for_init(&v.value, &v.range)),
                test: v.test.as_ref().map(|v| mb.build_expression(&v.value, &v.range)),
                advance: v.advance.as_ref().map(|v| mb.build_expression(&v.value, &v.range)),
                stmt: mb.build_statement(&v.stmt.value, &v.stmt.range),
            }
        })
    }
            
    pub fn build_for_init(&mut self, src: &ast::ForInit, _range: &ParsedRange) -> model::ForInit {
        match src {
            ast::ForInit::Expression(v) => model::ForInit::Expression(self.build_expression(&v.value, &v.range)),
            ast::ForInit::VarDecl(v) => model::ForInit::VarDecl(self.build_statement(&v.value, &v.range)),
        }
    }
            
    pub fn build_switch_statement(&mut self, src: &ast::SwitchStatement, range: &ParsedRange) -> model::ItemPtr<model::SwitchStatement> {
        self.build_item(src, range, |m| &mut m.switch_statements, |v, mb| {
            model::SwitchStatement {
                exp: mb.build_expression(&v.exp.value, &v.exp.range),
                items: v.items.value.iter().map(|v| mb.build_switch_item(&v.value, &v.range)).collect(),
            }
        })
    }
            
    pub fn build_switch_item(&mut self, src: &ast::SwitchItem, range: &ParsedRange) -> model::SwitchItem {
        match src {
            ast::SwitchItem::Statement(v) => model::SwitchItem::Statement(self.build_switch_body_statement(&v.value, &v.range)),
            ast::SwitchItem::Case(v) => model::SwitchItem::Case(self.build_switch_case(&v.value, &v.range)),
            ast::SwitchItem::Default => model::SwitchItem::Default(self.build_switch_default(range)),
        }
    }
            
    pub fn build_switch_body_statement(&mut self, src: &ast::Statement, range: &ParsedRange) -> model::ItemPtr<model::SwitchBodyStatement> {
        self.build_item(src, range, |m| &mut m.switch_body_statements, |_v, mb| {
            model::SwitchBodyStatement {
                stmt: mb.build_statement(src, range),
            }
        })
    }
            
    pub fn build_switch_case(&mut self, src: &ast::Expression, range: &ParsedRange) -> model::ItemPtr<model::SwitchCase> {
        self.build_item(src, range, |m| &mut m.switch_cases, |_v, mb| {
            model::SwitchCase {
                exp: mb.build_expression(src, range),
            }
        })
    }
            
    pub fn build_switch_default(&mut self, range: &ParsedRange) -> model::ItemPtr<model::SwitchDefault> {
        self.build_item(&(), range, |m| &mut m.switch_defaults, |_v, _mb| {
            model::SwitchDefault {
            }
        })
    }
            
    pub fn build_var_decl_statement(&mut self, src: &ast::VarDeclStatement, range: &ParsedRange) -> model::ItemPtr<model::VarDeclStatement> {
        self.build_item(src, range, |m| &mut m.var_decl_statements, |v, mb| {
            model::VarDeclStatement {
                let_or_const: mb.build_let_or_const(&v.let_or_const.value),
                name: v.name.value.clone(),
                init: v.init.as_ref().map(|v| mb.build_expression(&v.value, &v.range)),
            }
        })
    }

    pub fn build_let_or_const(&mut self, src: &ast::LetOrConst) -> model::LetOrConst {
        match src {
            ast::LetOrConst::Let => model::LetOrConst::Let,
            ast::LetOrConst::Const => model::LetOrConst::Const,
        }
    }
            
    pub fn build_function_decl_statement(&mut self, src: &ast::FunctionDeclStatement, range: &ParsedRange) -> model::ItemPtr<model::FunctionDeclStatement> {
        self.build_item(src, range, |m| &mut m.function_decl_statements, |v, mb| {
            model::FunctionDeclStatement {
                name: v.name.value.clone(),
                signature: mb.build_function_signature(&v.signature.value, &v.signature.range),
                body: mb.build_statement(&v.body.value, &v.body.range),
            }
        })
    }
            
    pub fn build_function_signature(&mut self, src: &ast::FunctionSignature, range: &ParsedRange) -> model::ItemPtr<model::FunctionSignature> {
        self.build_item(src, range, |m| &mut m.function_signatures, |v, mb| {
            model::FunctionSignature {
                args: v.args.value.iter().map(|v| mb.build_function_decl_arg(&v.value, &v.range)).collect(),
            }
        })
    }
            
    pub fn build_function_decl_arg(&mut self, src: &ast::FunctionDeclArg, range: &ParsedRange) -> model::ItemPtr<model::FunctionDeclArg> {
        self.build_item(src, range, |m| &mut m.function_decl_args, |v, _mb| {
            model::FunctionDeclArg {
                name: v.name.value.clone(),
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
            
    pub fn build_expression(&mut self, src: &ast::Expression, range: &ParsedRange) -> model::Expression {
            match src {
                ast::Expression::CommaExpression(a) => model::Expression::CommaExpression(self.build_comma_expression(&a, range)),
                ast::Expression::TernaryExpression(a) => model::Expression::TernaryExpression(self.build_ternary_expression(&a, range)),
                ast::Expression::BinaryExpression(a) => model::Expression::BinaryExpression(self.build_binary_expression(&a, range)),
                ast::Expression::UnaryExpression(a) => model::Expression::UnaryExpression(self.build_unary_expression(&a, range)),
                ast::Expression::BooleanLiteral(a) => model::Expression::BooleanLiteral(self.build_boolean_literal(&a, range)),
                ast::Expression::NullLiteral => model::Expression::NullLiteral(self.build_null_literal(range)),
                ast::Expression::StringLiteral(a) => model::Expression::StringLiteral(self.build_string_literal(&a, range)),
                ast::Expression::IntLiteral(a) => model::Expression::IntLiteral(self.build_int_literal(&a, range)),
                ast::Expression::MemberExpression(a) => self.build_member_expression(&a, range),
                ast::Expression::IdentifierExpression(a) => model::Expression::IdentifierExpression(self.build_identifier_expression(&a, range)),
            }
    }
            
    pub fn build_comma_expression(&mut self, src: &ast::CommaExpression, range: &ParsedRange) -> model::ItemPtr<model::CommaExpression> {
        self.build_item(src, range, |m| &mut m.comma_expressions, |v, mb| {
            model::CommaExpression {
                exps: v.exps.iter().map(|v| mb.build_expression(&v.value, &v.range)).collect()
            }
        })
    }
            
    pub fn build_ternary_expression(&mut self, src: &ast::TernaryExpression, _range: &ParsedRange) -> model::ItemPtr<model::TernaryExpression> {
        let if_false = self.build_expression(&src.if_false.value, &src.if_false.range);
        self.build_one_ternary_expression(&src.terms, 0, &if_false)
    }
            
    pub fn build_one_ternary_expression(&mut self, terms: &Vec<Parsed<ast::TernaryExpressionTerm>>, ix: usize, if_false: &model::Expression) -> model::ItemPtr<model::TernaryExpression> {
        let term = &terms.get(ix).unwrap().value;
        let test = self.build_expression(&term.test.value, &term.test.range);
        let if_true = self.build_expression(&term.if_true.value, &term.if_true.range);
        
        self.build_item(&(), &term.test.range, |m| &mut m.ternary_expressions, |_v, mb| {
            if ix == terms.len() - 1 {
                model::TernaryExpression {
                    test: test,
                    if_true: if_true,
                    if_false: *if_false,
                }
            }
            else {
                let e = mb.build_one_ternary_expression(terms, ix + 1, if_false);
                let eexp = model::Expression::TernaryExpression(e);
                model::TernaryExpression {
                    test: test,
                    if_true: if_true,
                    if_false: eexp,
                }
            }
        })
    }
            
    pub fn build_binary_expression(&mut self, src: &ast::BinaryExpression, _range: &ParsedRange) -> model::ItemPtr<model::BinaryExpression> {
        let first = self.build_expression(&src.first.value, &src.first.range);
        self.build_one_binary_expression(&src.rest, src.rest.len() - 1, &first)
    }
            
    fn build_one_binary_expression(&mut self, terms: &Vec<Parsed<ast::BinaryExpressionTerm>>, ix: usize, first: &model::Expression) -> model::ItemPtr<model::BinaryExpression> {
        let term = &terms.get(ix).unwrap().value;
        let op = &term.op;
        let model_op = self.build_binary_op(&op.value);
        let right = self.build_expression(&term.exp.value, &term.exp.range);
        
        self.build_item(&(), &op.range, |m| &mut m.binary_expressions, |_v, mb| {
            if ix == 0 {
                model::BinaryExpression {
                    left: *first,
                    op: model_op,
                    right,
                }
            }
            else {
                let e = mb.build_one_binary_expression(terms, ix - 1, first);
                // Build an Expression around the UnaryExpression
                let eexp = model::Expression::BinaryExpression(e);
                model::BinaryExpression {
                    left: eexp,
                    op: model_op,
                    right: right,
                }
            }
        })
    }

    pub fn build_binary_op(&mut self, src: &ast::BinaryOp) -> model::BinaryOp {
        match src {
            ast::BinaryOp::Plus => model::BinaryOp::Plus,
            ast::BinaryOp::Minus => model::BinaryOp::Minus,
            ast::BinaryOp::Times => model::BinaryOp::Times,
            ast::BinaryOp::Divide => model::BinaryOp::Divide,
            ast::BinaryOp::Mod => model::BinaryOp::Mod,
            ast::BinaryOp::ShiftLeft => model::BinaryOp::ShiftLeft,
            ast::BinaryOp::LogicalShiftRight => model::BinaryOp::LogicalShiftRight,
            ast::BinaryOp::ArithmeticShiftRight => model::BinaryOp::ArithmeticShiftRight,
            ast::BinaryOp::LessThan => model::BinaryOp::LessThan,
            ast::BinaryOp::LessThanOrEquals => model::BinaryOp::LessThanOrEquals,
            ast::BinaryOp::GreaterThan => model::BinaryOp::GreaterThan,
            ast::BinaryOp::GreaterThanOrEquals => model::BinaryOp::GreaterThanOrEquals,
            ast::BinaryOp::Equals => model::BinaryOp::Equals,
            ast::BinaryOp::NotEquals => model::BinaryOp::NotEquals,
            ast::BinaryOp::BitwiseAnd => model::BinaryOp::BitwiseAnd,
            ast::BinaryOp::BitwiseXor => model::BinaryOp::BitwiseXor,
            ast::BinaryOp::BitwiseOr => model::BinaryOp::BitwiseOr,
            ast::BinaryOp::LogicalAnd => model::BinaryOp::LogicalAnd,
            ast::BinaryOp::LogicalOr => model::BinaryOp::LogicalOr,
            ast::BinaryOp::Assign => model::BinaryOp::Assign,
            ast::BinaryOp::PlusAssign => model::BinaryOp::PlusAssign,
            ast::BinaryOp::MinusAssign => model::BinaryOp::MinusAssign,
            ast::BinaryOp::TimesAssign => model::BinaryOp::TimesAssign,
            ast::BinaryOp::DivideAssign => model::BinaryOp::DivideAssign,
            ast::BinaryOp::ModAssign => model::BinaryOp::ModAssign,
            ast::BinaryOp::ShiftLeftAssign => model::BinaryOp::ShiftLeftAssign,
            ast::BinaryOp::LogicalShiftRightAssign => model::BinaryOp::LogicalShiftRightAssign,
            ast::BinaryOp::ArithmeticShiftRightAssign => model::BinaryOp::ArithmeticShiftRightAssign,
            ast::BinaryOp::BitwiseAndAssign => model::BinaryOp::BitwiseAndAssign,
            ast::BinaryOp::BitwiseXorAssign => model::BinaryOp::BitwiseXorAssign,
            ast::BinaryOp::BitwiseOrAssign => model::BinaryOp::BitwiseOrAssign,
            ast::BinaryOp::LogicalAndAssign => model::BinaryOp::LogicalAndAssign,
            ast::BinaryOp::LogicalOrAssign => model::BinaryOp::LogicalOrAssign,
        }
    }
            
    pub fn build_unary_expression(&mut self, src: &ast::UnaryExpression, _range: &ParsedRange) -> model::ItemPtr<model::UnaryExpression> {
        let exp = self.build_expression(&src.exp.value, &src.exp.range);
        self.build_one_unary_expression(&src.ops, 0, &exp)
    }
            
    fn build_one_unary_expression(&mut self, ops: &Vec<Parsed<ast::UnaryOp>>, ix: usize, exp: &model::Expression) -> model::ItemPtr<model::UnaryExpression> {
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
                let eexp = model::Expression::UnaryExpression(e);
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
            
    pub fn build_member_expression(&mut self, src: &ast::MemberExpression, _range: &ParsedRange) -> model::Expression {
        let first = self.build_expression(&src.first.value, &src.first.range);
        self.build_one_member_expression(&src.rest, src.rest.len() - 1, &first)
    }
            
    pub fn build_one_member_expression(&mut self, rest: &Vec<Parsed<ast::MemberOp>>, ix: usize, exp: &model::Expression) -> model::Expression {
        let op = &rest.get(ix).unwrap();

        if ix == 0 {
            self.build_member_expression_op(op, exp)
        }
        else {
            let e = self.build_one_member_expression(rest, ix - 1, exp);
            self.build_member_expression_op(op, &e)
        }
    }

    pub fn build_member_expression_op(&mut self, op: &Parsed<ast::MemberOp>, exp: &model::Expression) -> model::Expression {
        match &op.value {
            ast::MemberOp::DotAccess(v) => model::Expression::DotAccessExpression(self.build_dot_access_expression(v, &op.range, exp)),
            ast::MemberOp::IndexAccess(v) => model::Expression::IndexAccessExpression(self.build_index_access_expression(v, &op.range, exp)),
            ast::MemberOp::FunctionCall(v) => model::Expression::FunctionCallExpression(self.build_function_call_expression(v, &op.range, exp)),
            ast::MemberOp::NonNullAssert => model::Expression::NonNullAssertExpression(self.build_non_null_assert_expression(&op.range, exp)),
        }
    }

    pub fn build_dot_access_expression(&mut self, src: &ast::DotAccess, range: &ParsedRange, exp: &model::Expression) -> model::ItemPtr<model::DotAccessExpression> {
        self.build_item(src, range, |m| &mut m.dot_access_expressions, |v, _mb| {
            model::DotAccessExpression {
                exp: *exp,
                name: v.name.value.clone(),
            }
        })
    }

    pub fn build_index_access_expression(&mut self, src: &ast::IndexAccess, range: &ParsedRange, exp: &model::Expression) -> model::ItemPtr<model::IndexAccessExpression> {
        self.build_item(src, range, |m| &mut m.index_access_expressions, |v, mb| {
            model::IndexAccessExpression {
                exp: *exp,
                access_exp: mb.build_expression(&v.exp.value, &v.exp.range),
            }
        })
    }

    pub fn build_function_call_expression(&mut self, src: &ast::FunctionCall, range: &ParsedRange, exp: &model::Expression) -> model::ItemPtr<model::FunctionCallExpression> {
        self.build_item(src, range, |m| &mut m.function_call_expressions, |v, mb| {
            model::FunctionCallExpression {
                exp: *exp,
                args: v.args.value.iter().map(|v| mb.build_expression(&v.value, &v.range)).collect(),
            }
        })
    }

    pub fn build_non_null_assert_expression(&mut self, range: &ParsedRange, exp: &model::Expression) -> model::ItemPtr<model::NonNullAssertExpression> {
        self.build_item(&(), range, |m| &mut m.non_null_assert_expressions, |_v, _mb| {
            model::NonNullAssertExpression {
                exp: *exp,
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
