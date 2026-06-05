// FIXME - to reduce noise during initial development
//#![allow(unused)]

use crate::ast;
use crate::peg_parser::{PegParser, CharClass, Parsed};

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::Statement, |p| {
            p.parse(|p| {
                if let Some(r) = self.empty_statement(p) {Some(r)}
                else if let Some(r) = self.expression_statement(p) {Some(r)}
                else if let Some(r) = self.if_statement(p) {Some(r)}
                else if let Some(r) = self.while_statement(p) {Some(r)}
                else if let Some(r) = self.return_statement(p) {Some(r)}
                else if let Some(r) = self.break_statement(p) {Some(r)}
                else if let Some(r) = self.continue_statement(p) {Some(r)}
                else if let Some(r) = self.block_statement(p) {Some(r)}
                else if let Some(r) = self.for_statement(p) {Some(r)}
                else if let Some(r) = self.switch_statement(p) {Some(r)}
                else if let Some(r) = self.var_decl_statement(p) {Some(r)}
                else if let Some(r) = self.function_decl_statement(p) {Some(r)}
                else {None}
            })
        })
    }

    pub fn empty_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::EmptyStatement, |p| {
            p.parse(|p| {
                Some(p.ch(';')?.with_value(ast::Statement::empty_statement()))
            })
        })
    }

    pub fn expression_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::ExpressionStatement, |p| {
            p.parse(|p| {
                let exp = self.expression(p)?;
                self.opt_sp(p)?;
                self.statement_end(p)?;
                Some(p.parsed(ast::Statement::expression_statement(exp)))
            })
        })
    }

    pub fn if_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::WhileStatement, |p| {
            p.parse(|p| {
                p.str("if")?;
                self.opt_sp(p)?;
                p.str("(")?;
                self.opt_sp(p)?;
                let test = self.expression(p)?;
                self.opt_sp(p)?;
                p.str(")")?;
                self.opt_sp(p)?;
                let if_true = self.statement(p)?;
                let if_false = p.parse(|p| {
                    self.opt_sp(p)?;
                    p.str("else")?;
                    self.opt_sp(p)?;
                    let stmt = self.statement(p)?.value;
                    Some(p.parsed(stmt))
                });
                Some(p.parsed(ast::Statement::if_statement(test, if_true, if_false)))
            })
        })
    }

    pub fn while_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::WhileStatement, |p| {
            p.parse(|p| {
                p.str("while")?;
                self.opt_sp(p)?;
                p.str("(")?;
                self.opt_sp(p)?;
                let test = self.expression(p)?;
                self.opt_sp(p)?;
                p.str(")")?;
                self.opt_sp(p)?;
                let stmt = self.statement(p)?;
                Some(p.parsed(ast::Statement::while_statement(test, stmt)))
            })
        })
    }

    pub fn return_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::ReturnStatement, |p| {
            p.parse(|p| {
                p.str("return")?;
                let exp = p.parse(|p| {
                    self.opt_sp(p)?;
                    self.expression(p)
                });
                self.statement_end(p)?;
                Some(p.parsed(ast::Statement::return_statement(exp)))
            })
        })
    }

    pub fn break_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::BreakStatement, |p| {
            p.parse(|p| {
                p.str("break")?;
                let label = p.parse(|p| {
                    self.opt_sp(p)?;
                    self.identifier(p)
                });
                self.statement_end(p)?;
                Some(p.parsed(ast::Statement::break_statement(label)))
            })
        })
    }

    pub fn continue_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::ContinueStatement, |p| {
            p.parse(|p| {
                p.str("continue")?;
                let label = p.parse(|p| {
                    self.opt_sp(p)?;
                    self.identifier(p)
                });
                self.statement_end(p)?;
                Some(p.parsed(ast::Statement::continue_statement(label)))
            })
        })
    }

    pub fn block_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::BlockStatement, |p| {
            p.parse(|p| {
                p.str("{")?;
                let stmts = p.star(|p| {
                    self.opt_sp(p)?;
                    self.statement(p)
                })?;
                self.opt_sp(p)?;
                p.str("}")?;
                Some(p.parsed(ast::Statement::block_statement(stmts)))
            })
        })
    }

    pub fn for_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::ForStatement, |p| {
            p.parse(|p| {
                p.str("for")?;
                self.opt_sp(p)?;
                p.str("(")?;
                self.opt_sp(p)?;
                let init = p.parse(|p| {
                    if let Some(r) = self.var_decl(p) {Some(p.parsed(ast::ForInit::VarDecl(r)))}
                    else if let Some(r) = self.expression(p) {Some(p.parsed(ast::ForInit::Expression(r)))}
                    else {None}
                });
                self.opt_sp(p)?;
                p.str(";")?;
                self.opt_sp(p)?;
                let test = self.expression(p);
                self.opt_sp(p)?;
                p.str(";")?;
                self.opt_sp(p)?;
                let advance = self.expression(p);
                self.opt_sp(p)?;
                p.str(")")?;
                self.opt_sp(p)?;
                let stmt = self.statement(p)?;
                Some(p.parsed(ast::Statement::for_statement(init, test, advance, stmt)))
            })
        })
    }
    
    pub fn switch_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::ForStatement, |p| {
            p.parse(|p| {
                p.str("switch")?;
                self.opt_sp(p)?;
                p.str("(")?;
                self.opt_sp(p)?;
                let exp = self.expression(p)?;
                self.opt_sp(p)?;
                p.str(")")?;
                self.opt_sp(p)?;
                p.str("{")?;
                let items = p.star(|p| {
                    self.opt_sp(p)?;
                    if let Some(r) = p.parse(|p| {
                        p.str("default")?;
                        self.opt_sp(p)?;
                        p.str(":")?;
                        Some(p.parsed(ast::SwitchItem::Default))
                    }) {Some(r)}
                    else if let Some(r) = p.parse(|p| {
                        p.str("case")?;
                        self.opt_sp(p)?;
                        let cexp = self.expression(p)?;
                        self.opt_sp(p)?;
                        p.str(":")?;
                        Some(p.parsed(ast::SwitchItem::Case(cexp)))
                    }) {Some(r)}
                    else if let Some(r) = p.parse(|p| {
                        self.opt_sp(p)?;
                        let stmt = self.statement(p)?;
                        Some(p.parsed(ast::SwitchItem::Statement(stmt)))
                    }) {Some(r)}
                    else {None}
                })?;
                self.opt_sp(p)?;
                p.str("}")?;
                Some(p.parsed(ast::Statement::switch_statement(exp, items)))
            })
        })
    }
    
    pub fn var_decl(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::VarDeclStatement, |p| {
            p.parse(|p| {
                p.str("var")?;
                self.opt_sp(p)?;
                let name = self.identifier(p)?;
                // FIXME - add type declaration
                let init = p.parse(|p| {
                    self.opt_sp(p)?;
                    p.str("=")?;
                    self.opt_sp(p)?;
                    let exp = self.expression(p)?.value;
                    Some(p.parsed(exp))
                });
                Some(p.parsed(ast::Statement::var_decl_statement(name, init)))
            })
        })
    }
    
    pub fn var_decl_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::VarDeclStatement, |p| {
            p.parse(|p| {
                let stmt = self.var_decl(p)?.value;
                self.opt_sp(p)?;
                self.statement_end(p)?;
                Some(p.parsed(stmt))
            })
        })
    }
    
    pub fn function_decl_statement(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Statement>> {
        p.for_rule(RuleName::FunctionDeclStatement, |p| {
            p.parse(|p| {
                p.str("function")?;
                self.opt_sp(p)?;
                let name = self.identifier(p)?;
                self.opt_sp(p)?;
                p.str("(")?;
                let args = self.function_decl_args(p)?;
                self.opt_sp(p)?;
                p.opt(|p| {
                    p.str(",")?;
                    self.opt_sp(p)?;
                    Some(p.parsed(()))
                })?;
                p.str(")")?;
                self.opt_sp(p)?;
                let stmt = self.block_statement(p)?;
                Some(p.parsed(ast::Statement::function_decl_statement(name, args, stmt)))
            })
        })
    }
    
    pub fn function_decl_arg(&self, p: &mut impl PegParser) -> Option<Parsed<ast::FunctionDeclArg>> {
        p.for_rule(RuleName::FunctionDeclArg, |p| {
            p.parse(|p| {
                self.opt_sp(p)?;
                let name = self.identifier(p)?;
                Some(p.parsed(ast::FunctionDeclArg {name}))
            })
        })
    }
    
    pub fn function_decl_args(&self, p: &mut impl PegParser) -> Option<Parsed<Vec<Parsed<ast::FunctionDeclArg>>>> {
        p.for_rule(RuleName::FunctionDeclArgs, |p| {
            p.parse(|p| {
                if let Some(first) = self.function_decl_arg(p) {
                    let rest = p.star(|p| {
                        self.opt_sp(p)?;
                        p.str(",")?;
                        self.opt_sp(p)?;
                        self.function_decl_arg(p)
                    })?;
                    Some(p.parsed(first_and_rest(first, rest)))
                }
                else {
                    Some(p.parsed(Vec::new()))
                }
            })
        })
    }
    
    pub fn expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        self.comma_expression(p)
    }

    pub fn comma_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::CommaExpression, |p| {
            p.parse(|p| {
                let first = self.assignment_expression(p)?;
                let rest = p.star(|p| p.try_parse(|p| {
                    self.opt_sp(p)?;
                    p.str(",")?;
                    self.opt_sp(p)?;
                    let exp = self.ternary_expression(p)?;
                    Some(exp)
                }))?;
                if rest.value.is_empty() { Some(first) }
                else {
                    let value = ast::Expression::comma_expression(first_and_rest(first, rest));
                    Some(p.parsed(value))
                }
            })
        })
    }

    fn binary_expression<SF, PP>(&self, p: &mut PP, subexp: SF, op_strs: &[(&'static str, ast::BinaryOp)]) -> Option<Parsed<ast::Expression>>
    where
        PP: PegParser,
        SF: Fn(&mut PP)->Option<Parsed<ast::Expression>>
    {
        p.parse(|p| {
            let first = subexp(p)?;
            self.opt_sp(p)?;
            let rest = p.star(|p| p.to_parsed(|p| {
                self.opt_sp(p)?;
                let op = self.op_str(p, op_strs)?;
                self.opt_sp(p)?;
                let exp = subexp(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            }))?.value;
            if rest.is_empty() {Some(first)}
            else {
                let value = ast::Expression::binary_expression(first, rest);
                Some(p.parsed(value))
            }
        })
    }

    fn op_str<R>(&self, p: &mut impl PegParser, op_strs: &[(&'static str, R)]) -> Option<Parsed<R>>
        where R:Copy
    {
        for (s, r) in op_strs {
            if let Some(s) = p.str(s) {return Some(s.with_value(*r))}
        }
        None
    }

    pub fn assignment_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::AssignmentExpression, |p| {
            self.binary_expression(p, |p| self.ternary_expression(p), &[
                ("=", ast::BinaryOp::Assign),
                ("+=", ast::BinaryOp::PlusAssign),
                ("-=", ast::BinaryOp::MinusAssign),
                ("*=", ast::BinaryOp::TimesAssign),
                ("/=", ast::BinaryOp::DivideAssign),
                ("%=", ast::BinaryOp::ModAssign),
                ("<<=", ast::BinaryOp::ShiftLeftAssign),
                (">>>=", ast::BinaryOp::LogicalShiftRightAssign),
                (">>=", ast::BinaryOp::ArithmeticShiftRightAssign),
                ("&=", ast::BinaryOp::BitwiseAndAssign),
                ("^=", ast::BinaryOp::BitwiseXorAssign),
                ("|=", ast::BinaryOp::BitwiseOrAssign),
                ("&&=", ast::BinaryOp::LogicalAndAssign),
                ("||=", ast::BinaryOp::LogicalOrAssign),
            ])
        })
    }

    pub fn ternary_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::TernaryExpression, |p| {
            p.parse(|p| {
                let terms = p.star(|p| self.ternary_expression_term(p))?.value;
                self.opt_sp(p)?;
                let if_false = self.logical_or_expression(p)?;

                if terms.is_empty() {Some(if_false)}
                else {
                    let value = ast::Expression::ternary_expression(terms, if_false);
                    Some(p.parsed(value))
                }
            })
        })
    }

    pub fn ternary_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::TernaryExpressionTerm>> {
        p.for_rule(RuleName::TernaryExpressionTerm, |p| {
            p.parse(|p| {
                let test = self.logical_or_expression(p)?;
                self.opt_sp(p)?;
                p.str("?")?;
                self.opt_sp(p)?;
                let if_true = self.logical_or_expression(p)?;
                self.opt_sp(p)?;
                p.str(":")?;
                self.opt_sp(p)?;

                let value = ast::TernaryExpressionTerm {
                    test: Box::new(test),
                    if_true: Box::new(if_true),
                };
                Some(p.parsed(value))
            })
        })
    }

    pub fn logical_or_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalOrExpression, |p| {
            self.binary_expression(p, |p| self.logical_and_expression(p), &[
                ("||", ast::BinaryOp::LogicalOr),
            ])
        })
    }

    pub fn logical_and_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalAndExpression, |p| {
            self.binary_expression(p, |p| self.bitwise_or_expression(p), &[
                ("&&", ast::BinaryOp::LogicalAnd),
            ])
        })
    }

    pub fn bitwise_or_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseOrExpression, |p| {
            self.binary_expression(p, |p| self.bitwise_xor_expression(p), &[
                ("|", ast::BinaryOp::BitwiseOr),
            ])
        })
    }

    pub fn bitwise_xor_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseXorExpression, |p| {
            self.binary_expression(p, |p| self.bitwise_and_expression(p), &[
                ("^", ast::BinaryOp::BitwiseXor),
            ])
        })
    }

    pub fn bitwise_and_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseAndExpression, |p| {
            self.binary_expression(p, |p| self.equality_expression(p), &[
                ("&", ast::BinaryOp::BitwiseAnd),
            ])
        })
    }

    pub fn equality_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::EqualityExpression, |p| {
            self.binary_expression(p, |p| self.relational_expression(p), &[
                ("==", ast::BinaryOp::Equals),
                ("!=", ast::BinaryOp::NotEquals),
            ])
        })
    }

    pub fn relational_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::RelationalExpression, |p| {
            self.binary_expression(p, |p| self.bitshift_expression(p), &[
                ("<=", ast::BinaryOp::LessThanOrEquals),
                ("<", ast::BinaryOp::LessThan),
                (">=", ast::BinaryOp::GreaterThanOrEquals),
                (">", ast::BinaryOp::GreaterThan),
            ])
        })
    }

    pub fn bitshift_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitshiftExpression, |p| {
            self.binary_expression(p, |p| self.add_expression(p), &[
                ("<<", ast::BinaryOp::ShiftLeft),
                (">>>", ast::BinaryOp::LogicalShiftRight),
                (">>", ast::BinaryOp::ArithmeticShiftRight),
            ])
        })
    }

    pub fn add_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::AddExpression, |p| {
            self.binary_expression(p, |p| self.mult_expression(p), &[
                ("+", ast::BinaryOp::Plus),
                ("-", ast::BinaryOp::Minus),
            ])
        })
    }

    pub fn mult_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::MultExpression, |p| {
            self.binary_expression(p, |p| self.unary_expression(p), &[
                ("*", ast::BinaryOp::Times),
                ("/", ast::BinaryOp::Divide),
                ("%", ast::BinaryOp::Mod)
            ])
        })
    }

    pub fn unary_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::UnaryExpression, |p| {
            p.parse(|p| {
                let ops = p.star(|p| {
                    self.opt_sp(p)?;
                    self.op_str(p, &[
                        ("+", ast::UnaryOp::Plus),
                        ("-", ast::UnaryOp::Minus),
                        ("!", ast::UnaryOp::LogicalNot),
                        ("~", ast::UnaryOp::BitwiseNot),
                    ])
                })?.value;
                self.opt_sp(p)?;
                let exp = self.member_expression(p)?;

                if ops.is_empty() {Some(exp)}
                else {
                    let value = ast::Expression::unary_expression(ops, exp);
                    Some(p.parsed(value))
                }
            })
        })
    }

    pub fn member_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::MemberExpression, |p| {
            p.parse(|p| {
                let first = self.grouping_expression(p)?;
                let rest = p.star(|p| {
                    if let Some(r) = self.dot_access(p) {Some(r)}
                    else if let Some(r) = self.function_call(p) {Some(r)}
                    else if let Some(r) = self.index_access(p) {Some(r)}
                    else if let Some(r) = self.non_null_assert(p) {Some(r)}
                    else {None}
                })?.value;
                if rest.is_empty() {Some(first)}
                else {
                    let value = ast::Expression::member_expression(first, rest);
                    Some(p.parsed(value))
                }
            })
        })
    }

    pub fn dot_access(&self, p: &mut impl PegParser) -> Option<Parsed<ast::MemberOp>> {
        p.for_rule(RuleName::DotAccess, |p| {
            p.parse(|p| {
                self.opt_sp(p)?;
                p.str(".")?;
                self.opt_sp(p)?;
                let name = self.identifier(p)?;
                Some(p.parsed(ast::MemberOp::dot_access(name)))
            })
        })
    }

    pub fn function_call(&self, p: &mut impl PegParser) -> Option<Parsed<ast::MemberOp>> {
        p.for_rule(RuleName::DotAccess, |p| {
            p.parse(|p| {
                self.opt_sp(p)?;
                p.str("(")?;
                self.opt_sp(p)?;
                let args = self.expression_list(p)?;
                p.str(")")?;
                Some(p.parsed(ast::MemberOp::function_call(args)))
            })
        })
    }

    // Parses a comma-separated list of expressions.  Note that the individual elements are ternary expressions rather than full expressions, since full expressions include comma expressions, which would "swallow" all of the list into a single expression
    fn expression_list(&self, p: &mut impl PegParser) -> Option<Parsed<Vec<Parsed<ast::Expression>>>> {
        p.parse(|p| {
            self.opt_sp(p)?;
            if let Some(first) = self.ternary_expression(p) {
                let rest = p.star(|p| {
                    self.opt_sp(p)?;
                    p.str(",")?;
                    self.opt_sp(p)?;
                    let exp = self.ternary_expression(p)?;
                    Some(exp)
                })?;
                // Allow trailing comma
                self.opt_sp(p)?;
                p.opt(|p| p.str(","))?;
                
                Some(p.parsed(first_and_rest(first, rest)))
            }
            else {
                Some(p.parsed(Vec::new()))
            }
        })
    }

    pub fn index_access(&self, p: &mut impl PegParser) -> Option<Parsed<ast::MemberOp>> {
        p.for_rule(RuleName::IndexAccess, |p| {
            p.parse(|p| {
                self.opt_sp(p)?;
                p.str("[")?;
                self.opt_sp(p)?;
                let exp = self.expression(p)?;
                self.opt_sp(p)?;
                p.str("]")?;
                self.opt_sp(p)?;
                Some(p.parsed(ast::MemberOp::index_access(exp)))
            })
        })
    }

    pub fn non_null_assert(&self, p: &mut impl PegParser) -> Option<Parsed<ast::MemberOp>> {
        p.for_rule(RuleName::NonNullAssert, |p| {
            p.parse(|p| {
                self.opt_sp(p)?;
                p.str("!")?;
                // Make sure it's not followed by "=", to keep it from eating the "!" of a "!=" which is at a higher precedence level
                p.not(|p| p.str("="))?;
                Some(p.parsed(ast::MemberOp::non_null_assert()))
            })
        })
    }


    pub fn grouping_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::GroupingExpression, |p| {
            if let Some(r) = self.grouped_expression(p) {Some(r)}
            else if let Some(r) = self.primary_expression(p) {Some(r)}
            else {None}
        })
    }

    pub fn grouped_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::GroupedExpression, |p| {
            p.parse(|p| {
                p.str("(")?;
                self.opt_sp(p)?;
                let exp = self.expression(p)?.value;
                self.opt_sp(p)?;
                p.str(")")?;
                Some(p.parsed(exp))
            })
        })
    }

    pub fn primary_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::IdentifierExpression, |p| {
            if let Some(r) = self.literal_expression(p) {Some(r)}
            else if let Some(r) = self.identifier_expression(p) {Some(r)}
            else {None}
        })
    }

    pub fn identifier_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::PrimaryExpression, |p| {
            let Parsed {range, value} = self.identifier(p)?;
            Some(Parsed {range, value: ast::Expression::identifier_expression(value)})
        })
    }
    
    pub fn identifier(&self, p: &mut impl PegParser) -> Option<Parsed<String>> {
        p.for_rule(RuleName::Identifier, |p| {
            p.to_parsed(|p| {
                // Make sure it's not a reserved word
                p.not(|p| self.reserved_word(p))?;
                
                // FIXME - make sure it's not a reserved word
                let first = p.char_class(&IDENTIFIER_START_CHARS)?;
                let rest = p.star(|p| p.char_class(&IDENTIFIER_REST_CHARS))?;
                // Collect the Vec<Parsed<char>> into a String
                Some(first_and_rest(first, rest).iter().map(|i| i.value).collect::<String>())
            })
        })
    }

    pub fn reserved_word(&self, p: &mut impl PegParser) -> Option<Parsed<&'static str>> {
        p.for_rule(RuleName::ReservedWord, |p| {
            let word = self.match_str(p, &RESERVED_WORDS)?;
            self.word_boundary(p)?;
            Some(word)
        })
    }

    fn match_str(&self, p: &mut impl PegParser, match_strs: &[&'static str]) -> Option<Parsed<&'static str>>
    {
        for s in match_strs {
            if let Some(r) = p.str(s) {return Some(r)}
        }
        None
    }

    pub fn literal_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LiteralExpression, |p| {
            if let Some(r) = self.boolean_literal(p) {Some(r)}
            else if let Some(r) = self.int_literal(p) {Some(r)}
            else if let Some(r) = self.null_literal(p) {Some(r)}
            else if let Some(r) = self.string_literal(p) {Some(r)}
            // FIXME - add string template
            else {None}
        })
    }

    pub fn boolean_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BooleanLiteral, |p| {
            if let Some(r) = p.parse(|p| {
                let value = p.str("true")?;
                // Must be followed by a non-identifier char or eof
                self.word_boundary(p)?;
                Some(p.parsed(value))
            }) {Some(r.with_value(ast::Expression::BooleanLiteral(true)))}
            else if let Some(r) = p.parse(|p| {
                let value = p.str("false")?;
                // Must be followed by a non-identifier char or eof
                self.word_boundary(p)?;
                Some(p.parsed(value))
            }) {Some(r.with_value(ast::Expression::BooleanLiteral(false)))}
            else {None}
        })
    }

    pub fn null_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::NullLiteral, |p| {
            if let Some(r) = p.parse(|p| {
                let value = p.str("null")?;
                // Must be followed by a non-identifier char or eof
                self.word_boundary(p)?;
                Some(p.parsed(value))
            }) {Some(r.with_value(ast::Expression::null_literal()))}
            else {None}
        })
    }

    // Typically follows reserved words to make sure they aren't recognized too quickly if they're actually the start of a longer identifier (e.g., "false" vs. "falsey")
    pub fn word_boundary(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::WordBoundary, |p| {
            p.not(|p| p.char_class(&IDENTIFIER_REST_CHARS))
        })
    }

    pub fn string_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::StringLiteral, |p| {
            if let Some(r) = self.string_literal_for_delimiter(p, '\'') {Some(r)}
            else if let Some(r) = self.string_literal_for_delimiter(p, '\"') {Some(r)}
            else {None}
        })
    }

    pub fn string_literal_for_delimiter(&self, p: &mut impl PegParser, delimiter: char) -> Option<Parsed<ast::Expression>> {
        p.ch(delimiter)?;
        let chars = p.star(|p| self.string_literal_char(p, delimiter))?.value;
        p.ch(delimiter)?;

        // Unwrap the chars, remove any that are line continuations ("\" followed by line break), collect to string
        let value = chars.into_iter().map(|v| v.value.to_char()).flatten().collect();
        Some(p.parsed(ast::Expression::string_literal(value)))
    }

    pub fn string_literal_char(&self, p: &mut impl PegParser, delimiter: char) -> Option<Parsed<CharOrLineBreak>> {
        p.for_rule(RuleName::StringLiteralChar, |p| {
            // Check for "regular" characters (anything but the delimiter or escape)
            if let Some(r) = p.parse(|p| {
                let singles = [delimiter, '\\'];
                let char_class = CharClass::new().except().chars(&singles);
                let ch = p.char_class(&char_class)?;
                Some(ch.map_value(|v| CharOrLineBreak::Char(*v)))
            }) {Some(r)}
            // Check for escape sequences ("\")
            else if let Some(r) = p.parse(|p| {
                p.ch('\\')?;
                if let Some(r) = self.string_literal_escape_single(p, &[
                    ('n', '\n'),
                    ('r', '\r'),
                    ('t', '\t'),
                    ('v', '\x0b'),
                    ('b', '\x08'),
                    ('f', '\x0c'),
                    ('\\', '\\'),
                    ('\'', '\''),
                    ('\"', '\"'),
                    ('0', '\0'),
                ]) {Some(p.parsed(r.value))}
                // Check for hex escape ("\xXX")
                else if let Some(r) = p.parse(|p| {
                    p.ch('x')?;
                    let d0 = self.hex_digit(p)?.value;
                    let d1 = self.hex_digit(p)?.value;
                    let char_code = (d0 << 4) | d1;
                    let value = CharOrLineBreak::Char(char::from_u32(char_code)?);
                    Some(p.parsed(value))
                }) {Some(p.parsed(r.value))}
                // Check for unicode escape ("\uXXXX")
                else if let Some(r) = p.parse(|p| {
                    p.ch('u')?;
                    let d0 = self.hex_digit(p)?.value;
                    let d1 = self.hex_digit(p)?.value;
                    let d2 = self.hex_digit(p)?.value;
                    let d3 = self.hex_digit(p)?.value;
                    let char_code = (d0 << 12) | (d1 << 8) | (d2 << 4) | d3;
                    // FIXME - from_u32 returning None might mess up error handling
                    let value = CharOrLineBreak::Char(char::from_u32(char_code)?);
                    Some(p.parsed(value))
                }) {Some(p.parsed(r.value))}
                // Check for unicode codepoint ("\u{X...}")
                else if let Some(r) = p.parse(|p| {
                    p.str("u{")?;
                    let digits = p.plus(|p| self.hex_digit(p))?.value;
                    p.str("}")?;
                    if digits.len() > 6 {
                        // FIXME - this might mess up error handling.  Better to have a version of p.plus that takes a maximum
                        None
                    }
                    else {
                        let char_code = digits.iter().fold(0, |acc, v| (acc << 4) + v.value);
                        let value = CharOrLineBreak::Char(char::from_u32(char_code)?);
                        Some(p.parsed(value))
                    }
                }) {Some(p.parsed(r.value))}
                // "\" followed by newline is just a continuation
                else if let Some(r) = p.parse(|p| {
                    p.char_class(&NEWLINE_CHARS)?;
                    Some(p.parsed(CharOrLineBreak::LineBreak))
                }) {Some(p.parsed(r.value))}
                // "\" followed by anything else (except the delimiter) is just that character
                else if let Some(r) = p.parse(|p| {
                    let singles = [delimiter];
                    let char_class = CharClass::new().except().chars(&singles);
                    let ch = p.char_class(&char_class)?;
                    Some(ch.map_value(|v| CharOrLineBreak::Char(*v)))
                }) {Some(p.parsed(r.value))}
                else {None}
            }) {Some(r)}
            else {None}
        })
    }

    fn hex_digit(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.parse(|p| {
            let ch = p.char_class(&HEX_DIGIT)?;
            Some(ch.map_value(|v| v.to_digit(16).unwrap()))
        })
    }

    fn string_literal_escape_single(&self, p: &mut impl PegParser, mapping: &[(char, char)]) -> Option<Parsed<CharOrLineBreak>> {
        for (src, dest) in mapping {
            if let Some(s) = p.ch(*src) {return Some(s.with_value(CharOrLineBreak::Char(*dest)))}
        }
        None
    }
    
    fn int_literal_radix(&self, p: &mut impl PegParser, prefix: &'static str, char_class: &CharClass, radix: u32, ast_radix: ast::Radix) -> Option<Parsed<ast::Expression>>
    {
        p.to_parsed(|p| {
            p.str(prefix)?;
            let first = p.char_class(char_class)?.map_value(|v| DigitOrUnderscore::Digit(*v));
            let rest = p.star(|p| {
                if let Some(r) = p.ch('_') {
                    Some(r.with_value(DigitOrUnderscore::Underscore))
                }
                else {
                    Some(p.char_class(char_class)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
                }
            })?;
            Some(ast::Expression::u32_literal(collect_digits(&first_and_rest(first, rest), radix), ast_radix))
        })
    }

    pub fn decimal_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::DecimalLiteral, |p| {
            self.int_literal_radix(p, "", &DECIMAL_DIGIT, 10, ast::Radix::Decimal)
        })
    }

    pub fn hex_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::HexLiteral, |p| {
            self.int_literal_radix(p, "0x", &HEX_DIGIT, 16, ast::Radix::Hex)
        })
    }
    
    pub fn octal_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::OctalLiteral, |p| {
            self.int_literal_radix(p, "0o", &OCTAL_DIGIT, 8, ast::Radix::Octal)
        })
    }

    pub fn binary_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BinaryLiteral, |p| {
            self.int_literal_radix(p, "0b", &BINARY_DIGIT, 2, ast::Radix::Binary)
        })
    }

    pub fn int_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::IntLiteral, |p| {
            if let Some(r) = self.hex_literal(p) {Some(r)}
            else if let Some(r) = self.octal_literal(p) {Some(r)}
            else if let Some(r) = self.binary_literal(p) {Some(r)}
            else if let Some(r) = self.decimal_literal(p) {Some(r)}
            else {None}
        })
    }

    // shorthand for optional space
    pub fn opt_sp(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::OptSp, |p| {
            if let Some(r) = p.opt(|p| self.sp(p)) {Some(r.with_value(()))}
            else {None}
        })
    }

    pub fn sp(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::Sp, |p| {
            if let Some(r) = p.star(|p| {
                if let Some(r) = self.ws(p) {Some(r)}
                else if let Some(r) = self.line_comment(p) {Some(r)}
                else if let Some(r) = self.block_comment(p) {Some(r)}
                else {None}
            }) {Some(r.with_value(()))}
            else {None}
        })
    }

    pub fn ws_char(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::WsChar, |p| {
            Some(p.char_class(&WS_CHARS)?.with_value(()))
        })
    }

    pub fn ws(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::Ws, |p| {
            Some(p.char_class(&WS_CHARS)?.with_value(()))
        })
    }

    pub fn line_comment(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::LineComment, |p| {
            p.parse(|p| {
                p.str("//")?;
                p.star(|p| {
                    let value = p.char_class(&NOT_NEWLINE_CHARS)?;
                    Some(p.parsed(value))
                    //Some(p.char_class(&NOT_NEWLINE_CHARS)?)
                })?;
                Some(p.parsed(()))
            })
        })
    }

    pub fn block_comment(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::BlockComment, |p| {
            p.parse(|p| {
                p.str("/*")?;
                p.star(|p| {
                    p.not(|p| p.str("*/"))?;
                    p.char_class(&ANY_CHAR)?;
                    Some(p.parsed(()))
                })?;
                p.str("*/")?;
                Some(p.parsed(()))
            })
        })
    }

    // Space that doesn't include newlines (used in finding statement ends)
    pub fn no_line_end_space(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::NoLineEndSpace, |p| {
            if let Some(r) = p.plus(|p| {
                if let Some(r) = p.char_class(&NO_LINE_END_WS_CHARS) {Some(r.with_value(()))}
                else if let Some(r) = self.block_comment(p) {Some(r.with_value(()))}
                else {None}
            }) {Some(r.with_value(()))}
            else {None}
        })
    }

    // Things that can terminate a statement
    pub fn statement_end(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::StatementEnd, |p| {
            p.parse(|p| {
                // Consume any space up to potential statement enders
                p.opt(|p| self.no_line_end_space(p))?;
                p.parse(|p| {
                    if let Some(r) = p.ch(';') {Some(r.with_value(()))}
                    else if let Some(r) = self.line_comment(p) {Some(r.with_value(()))}
                    else if let Some(r) = p.char_class(&NEWLINE_CHARS) {Some(r.with_value(()))}
                    // Covers the last statement in a file
                    else if let Some(r) = p.lookahead(|p| p.eof()) {Some(r.with_value(()))}
                    // Covers the last statement in a block
                    else if let Some(r) = p.lookahead(|p| p.ch('}')) {Some(r.with_value(()))}
                    // Covers the "advance" statement in a for(..;..;..) statement
                    else if let Some(r) = p.lookahead(|p| p.ch(')')) {Some(r.with_value(()))}
                    else {None}
                })?;
                Some(p.parsed(()))
            })
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum RuleName {
    Statement,
    EmptyStatement,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    BreakStatement,
    ContinueStatement,
    ReturnStatement,
    BlockStatement,
    ForStatement,
    SwitchStatement,
    VarDecl,
    VarDeclStatement,
    FunctionDeclStatement,
    FunctionDeclArg,
    FunctionDeclArgs,

    CommaExpression,
    AssignmentExpression,
    TernaryExpression,
    TernaryExpressionTerm,

    WsChar,
    Ws,
    Sp,
    OptSp,
    LineComment,
    BlockComment,
    WordBoundary,
    Identifier,
    ReservedWord,
    BooleanLiteral,
    NullLiteral,
    StringLiteral,
    StringLiteralChar,
    DecimalLiteral,
    HexLiteral,
    OctalLiteral,
    BinaryLiteral,
    IntLiteral,

    AddExpression,
    MultExpression,
    BitshiftExpression,
    RelationalExpression,
    EqualityExpression,
    BitwiseAndExpression,
    BitwiseXorExpression,
    BitwiseOrExpression,
    LogicalAndExpression,
    LogicalOrExpression,
    UnaryExpression,
    MemberExpression,
    DotAccess,
    FunctionCall,
    FunctionCallArgs,
    IndexAccess,
    NonNullAssert,
    GroupingExpression,
    GroupedExpression,
    PrimaryExpression,
    IdentifierExpression,
    LiteralExpression,

    NoLineEndSpace,
    StatementEnd,
}

const ANY_CHAR: CharClass = CharClass::new().except();
const WS_CHARS: CharClass = CharClass::new().chars(&[' ', '\n', '\r', '\t']);
const NO_LINE_END_WS_CHARS: CharClass = CharClass::new().chars(&[' ', '\t']);
const IDENTIFIER_START_CHARS: CharClass = CharClass::new()
    .ranges(&[('A', 'Z'), ('a', 'z')])
    .chars(&['_']);
const IDENTIFIER_REST_CHARS: CharClass = CharClass::new()
    .ranges(&[('A', 'Z'), ('a', 'z'), ('0', '9')])
    .chars(&['_']);
const DECIMAL_DIGIT: CharClass = CharClass::new()
    .ranges(&[('0', '9')]);
const HEX_DIGIT: CharClass = CharClass::new()
    .ranges(&[('0', '9'), ('a', 'f'), ('A', 'F')]);
const OCTAL_DIGIT: CharClass = CharClass::new()
    .ranges(&[('0', '7')]);
const BINARY_DIGIT: CharClass = CharClass::new()
    .ranges(&[('0', '1')]);
const NEWLINE_CHARS: CharClass = CharClass::new().chars(&['\r', '\n']);
const NOT_NEWLINE_CHARS: CharClass = NEWLINE_CHARS.except();

const RESERVED_WORDS: &[&str] = &[
    "true",
    "false",
    "null",
    "if",
    "else",
    "while",
    "return",
    "break",
    "continue",
    "for",
    "switch",
    "case",
    "default",
    "function",
    "var",
];

pub enum DigitOrUnderscore {
    Digit(char),
    Underscore,
}

// Combines the given first and rest into a single Vec with a range spanning both
fn first_and_rest<R>(first: Parsed<R>, rest: Parsed<Vec<Parsed<R>>>) -> Vec<Parsed<R>> {
    std::iter::once(first).chain(rest.value.into_iter()).collect()
}

// Collect digit characters into a single u32 parsed with the given radix, ignoring underscores
fn collect_digits(digits: &Vec<Parsed<DigitOrUnderscore>>, radix: u32) -> u32 {
    digits.iter().fold(0, |acc, v| {
        match v.value {
            DigitOrUnderscore::Digit(d) => (acc * radix) + d.to_digit(radix).unwrap(),
            _ => acc
        }
    })
}

pub enum CharOrLineBreak {
    Char(char),
    // Represents a "\" followed by a newline, representing a line continuation in a string literal that doesn't actually contribute any characters
    LineBreak,
}

impl CharOrLineBreak {
    pub fn to_char(&self) -> Option<char> {
        match self {
            Self::Char(ch) => Some(*ch),
            Self::LineBreak => None,
        }
    }
}
