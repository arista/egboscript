// FIXME - to reduce noise during initial development
#![allow(unused)]

use crate::ast;
use crate::peg_parser::{PegParser, CharClass, Parsed, ParseError};

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
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

    pub fn sp(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::Sp, |p| {
            if let Some(r) = self.ws(p) {Some(r)}
            else {None}
        })
    }

    pub fn opt_sp(&self, p: &mut impl PegParser) -> Option<Parsed<()>> {
        p.for_rule(RuleName::OptSp, |p| {
            if let Some(r) = p.opt(|p| self.sp(p)) {Some(r.with_value(()))}
            else {None}
        })
    }

    pub fn identifier_start_char(&self, p: &mut impl PegParser) -> Option<Parsed<char>> {
        p.for_rule(RuleName::IdentifierStartChar, |p| {
            p.char_class(&IDENTIFIER_START_CHARS)
        })
    }

    pub fn identifier_rest_char(&self, p: &mut impl PegParser) -> Option<Parsed<char>> {
        p.for_rule(RuleName::IdentifierRestChar, |p| {
            p.char_class(&IDENTIFIER_REST_CHARS)
        })
    }
    
    pub fn identifier(&self, p: &mut impl PegParser) -> Option<Parsed<String>> {
        p.for_rule(RuleName::Identifier, |p| {
            let first = self.identifier_start_char(p)?;
            let rest = p.star(|p| self.identifier_rest_char(p))?;
            // Collect the Vec<Parsed<char>> into a String
            Some(first_and_rest(first, rest).map_value(|v| v.iter().map(|i| i.value).collect::<String>()))
        })
    }

    pub fn boolean_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BooleanLiteral, |p| {
            if let Some(r) = p.str("true") {Some(r.with_value(ast::Expression::BooleanLiteral(true)))}
            else if let Some(r) = p.str("false") {Some(r.with_value(ast::Expression::BooleanLiteral(false)))}
            else {None}
        })
    }

    pub fn underscore_digit(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::UnderscorDigit, |p| {
            Some(p.ch('_')?.with_value(DigitOrUnderscore::Underscore))
        })
    }
    
    pub fn decimal_digit(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::DecimalDigit, |p| {
            Some(p.char_class(&DECIMAL_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
        })
    }

    pub fn decimal_digit_or_underscore(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::DecimalDigitOrUnderscore, |p| {
            if let Some(r) = self.underscore_digit(p) {Some(r)}
            else {self.decimal_digit(p)}
        })
    }

    pub fn decimal_digits(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::DecimalDigits, |p| {
            let first = self.decimal_digit(p)?;
            let rest = p.star(|p| self.decimal_digit_or_underscore(p))?;
            Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 10)))
        })
    }

    
    pub fn hex_digit(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::HexDigit, |p| {
            Some(p.char_class(&HEX_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
        })
    }

    pub fn hex_digit_or_underscore(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::HexDigitOrUnderscore, |p| {
            if let Some(r) = self.underscore_digit(p) {Some(r)}
            else {self.hex_digit(p)}
        })
    }

    pub fn hex_digits(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::HexDigits, |p| {
            let first = self.hex_digit(p)?;
            let rest = p.star(|p| self.hex_digit_or_underscore(p))?;
            Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 16)))
        })
    }

    pub fn hex_literal(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::HexLiteral, |p| {
            let _ = p.str("0x")?;
            Some(self.hex_digits(p)?)
        })
    }
    
    pub fn octal_digit(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::OctalDigit, |p| {
            Some(p.char_class(&OCTAL_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
        })
    }

    pub fn octal_digit_or_underscore(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::OctalDigitOrUnderscore, |p| {
            if let Some(r) = self.underscore_digit(p) {Some(r)}
            else {self.octal_digit(p)}
        })
    }

    pub fn octal_digits(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::OctalDigits, |p| {
            let first = self.octal_digit(p)?;
            let rest = p.star(|p| self.octal_digit_or_underscore(p))?;
            Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 8)))
        })
    }

    pub fn octal_literal(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::OctalLiteral, |p| {
            let _ = p.str("0o")?;
            Some(self.octal_digits(p)?)
        })
    }
    
    pub fn binary_digit(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::BinaryDigit, |p| {
            Some(p.char_class(&BINARY_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
        })
    }

    pub fn binary_digit_or_underscore(&self, p: &mut impl PegParser) -> Option<Parsed<DigitOrUnderscore>> {
        p.for_rule(RuleName::BinaryDigitOrUnderscore, |p| {
            if let Some(r) = self.underscore_digit(p) {Some(r)}
            else {self.binary_digit(p)}
        })
    }

    pub fn binary_digits(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::BinaryDigits, |p| {
            let first = self.binary_digit(p)?;
            let rest = p.star(|p| self.binary_digit_or_underscore(p))?;
            Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 2)))
        })
    }

    pub fn binary_literal(&self, p: &mut impl PegParser) -> Option<Parsed<u32>> {
        p.for_rule(RuleName::BinaryLiteral, |p| {
            let _ = p.str("0b")?;
            Some(self.binary_digits(p)?)
        })
    }

    pub fn int_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::IntLiteral, |p| {
            if let Some(r) = self.hex_literal(p) {
                Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Hex)))
            }
            else if let Some(r) = self.octal_literal(p) {
                Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Octal)))
            }
            else if let Some(r) = self.binary_literal(p) {
                Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Binary)))
            }
            else if let Some(r) = self.decimal_digits(p) {
                Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Decimal)))
            }
            else {
                None
            }
        })
    }

    pub fn unary_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::UnaryOp>> {
        p.for_rule(RuleName::UnaryExpressionOp, |p| {
            if let Some(r) = p.ch('+') {Some(r.with_value(ast::UnaryOp::Plus))}
            else if let Some(r) = p.ch('-') {Some(r.with_value(ast::UnaryOp::Minus))}
            else if let Some(r) = p.ch('!') {Some(r.with_value(ast::UnaryOp::LogicalNot))}
            else if let Some(r) = p.ch('~') {Some(r.with_value(ast::UnaryOp::BitwiseNot))}
            else {None}
        })
    }

    pub fn unary_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::UnaryOp>> {
        p.for_rule(RuleName::UnaryExpressionTerm, |p| {
            let _ = self.opt_sp(p)?;
            self.unary_expression_op(p)
        })
    }

    pub fn unary_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::UnaryExpression, |p| {
            p.to_parsed(|p| {
                let ops = p.star(|p| self.unary_expression_term(p))?;
                let exp = self.int_literal(p)?;
                Some(ast::Expression::unary_expression(ops.value, exp))
            })
        })
    }

    pub fn mult_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::MultExpressionOp, |p| {
            if let Some(r) = p.ch('*') {Some(r.with_value(ast::BinaryOp::Times))}
            else if let Some(r) = p.ch('/') {Some(r.with_value(ast::BinaryOp::Divide))}
            else if let Some(r) = p.ch('%') {Some(r.with_value(ast::BinaryOp::Mod))}
            else {None}
        })
    }

    pub fn mult_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::MultExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.mult_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.unary_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn mult_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::MultExpression, |p| {
            p.to_parsed(|p| {
                let first = self.unary_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.mult_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn add_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::AddExpressionOp, |p| {
            if let Some(r) = p.ch('+') {Some(r.with_value(ast::BinaryOp::Plus))}
            else if let Some(r) = p.ch('-') {Some(r.with_value(ast::BinaryOp::Minus))}
            else {None}
        })
    }

    pub fn add_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::AddExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.add_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.mult_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn add_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::AddExpression, |p| {
            p.to_parsed(|p| {
                let first = self.mult_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.add_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn bitshift_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::BitshiftExpressionOp, |p| {
            if let Some(r) = p.str("<=") {Some(r.with_value(ast::BinaryOp::LessThanOrEquals))}
            else if let Some(r) = p.str("<") {Some(r.with_value(ast::BinaryOp::LessThan))}
            else if let Some(r) = p.str(">=") {Some(r.with_value(ast::BinaryOp::GreaterThanOrEquals))}
            else if let Some(r) = p.str(">") {Some(r.with_value(ast::BinaryOp::GreaterThan))}
            else {None}
        })
    }

    pub fn bitshift_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::BitshiftExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.bitshift_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.add_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn bitshift_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitshiftExpression, |p| {
            p.to_parsed(|p| {
                let first = self.add_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.bitshift_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn relational_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::RelationalExpressionOp, |p| {
            if let Some(r) = p.str("<<") {Some(r.with_value(ast::BinaryOp::ShiftLeft))}
            else if let Some(r) = p.str(">>>") {Some(r.with_value(ast::BinaryOp::LogicalShiftRight))}
            else if let Some(r) = p.str(">>") {Some(r.with_value(ast::BinaryOp::ArithmeticShiftRight))}
            else {None}
        })
    }

    pub fn relational_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::RelationalExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.relational_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.bitshift_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn relational_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::RelationalExpression, |p| {
            p.to_parsed(|p| {
                let first = self.bitshift_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.relational_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn equality_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::EqualityExpressionOp, |p| {
            if let Some(r) = p.str("==") {Some(r.with_value(ast::BinaryOp::Equals))}
            else if let Some(r) = p.str("!=") {Some(r.with_value(ast::BinaryOp::NotEquals))}
            else {None}
        })
    }

    pub fn equality_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::EqualityExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.equality_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.relational_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn equality_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::EqualityExpression, |p| {
            p.to_parsed(|p| {
                let first = self.relational_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.equality_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn bitwise_and_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::BitwiseAndExpressionOp, |p| {
            if let Some(r) = p.str("&") {Some(r.with_value(ast::BinaryOp::BitwiseAnd))}
            else {None}
        })
    }

    pub fn bitwise_and_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::BitwiseAndExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.bitwise_and_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.equality_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn bitwise_and_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseAndExpression, |p| {
            p.to_parsed(|p| {
                let first = self.equality_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.bitwise_and_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn bitwise_xor_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::BitwiseXorExpressionOp, |p| {
            if let Some(r) = p.str("^") {Some(r.with_value(ast::BinaryOp::BitwiseXor))}
            else {None}
        })
    }

    pub fn bitwise_xor_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::BitwiseXorExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.bitwise_xor_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.bitwise_and_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn bitwise_xor_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseXorExpression, |p| {
            p.to_parsed(|p| {
                let first = self.bitwise_and_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.bitwise_xor_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn bitwise_or_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::BitwiseOrExpressionOp, |p| {
            if let Some(r) = p.str("|") {Some(r.with_value(ast::BinaryOp::BitwiseOr))}
            else {None}
        })
    }

    pub fn bitwise_or_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::BitwiseOrExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.bitwise_or_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.bitwise_xor_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn bitwise_or_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseOrExpression, |p| {
            p.to_parsed(|p| {
                let first = self.bitwise_xor_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.bitwise_or_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn logical_and_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::LogicalAndExpressionOp, |p| {
            if let Some(r) = p.str("&&") {Some(r.with_value(ast::BinaryOp::LogicalAnd))}
            else {None}
        })
    }

    pub fn logical_and_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::LogicalAndExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.logical_and_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.bitwise_or_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn logical_and_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalAndExpression, |p| {
            p.to_parsed(|p| {
                let first = self.bitwise_or_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.logical_and_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }

    pub fn logical_or_expression_op(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryOp>> {
        p.for_rule(RuleName::LogicalOrExpressionOp, |p| {
            if let Some(r) = p.str("&&") {Some(r.with_value(ast::BinaryOp::LogicalOr))}
            else {None}
        })
    }

    pub fn logical_or_expression_term(&self, p: &mut impl PegParser) -> Option<Parsed<ast::BinaryExpressionTerm>> {
        p.for_rule(RuleName::LogicalOrExpressionTerm, |p| {
            p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.logical_or_expression_op(p)?;
                let _ = self.opt_sp(p)?;
                let exp = self.logical_and_expression(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            })
        })
    }

    pub fn logical_or_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalOrExpression, |p| {
            p.to_parsed(|p| {
                let first = self.logical_and_expression(p)?;
                let _ = self.opt_sp(p)?;
                let rest = p.star(|p| self.logical_or_expression_term(p))?.value;
                Some(ast::Expression::binary_expression(first, rest))
            })
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum RuleName {
    WsChar,
    Ws,
    Sp,
    OptSp,
    IdentifierStartChar,
    IdentifierRestChar,
    Identifier,
    BooleanLiteral,
    UnderscorDigit,
    DecimalDigit,
    DecimalDigitOrUnderscore,
    DecimalDigits,
    HexDigit,
    HexDigitOrUnderscore,
    HexDigits,
    HexLiteral,
    OctalDigit,
    OctalDigitOrUnderscore,
    OctalDigits,
    OctalLiteral,
    BinaryDigit,
    BinaryDigitOrUnderscore,
    BinaryDigits,
    BinaryLiteral,
    IntLiteral,
    Expression,

    AddExpressionOp,
    AddExpression,
    AddExpressionTerm,
    MultExpressionOp,
    MultExpression,
    MultExpressionTerm,
    BitshiftExpressionOp,
    BitshiftExpression,
    BitshiftExpressionTerm,
    RelationalExpressionOp,
    RelationalExpression,
    RelationalExpressionTerm,
    EqualityExpressionOp,
    EqualityExpression,
    EqualityExpressionTerm,
    BitwiseAndExpressionOp,
    BitwiseAndExpression,
    BitwiseAndExpressionTerm,
    BitwiseXorExpressionOp,
    BitwiseXorExpression,
    BitwiseXorExpressionTerm,
    BitwiseOrExpressionOp,
    BitwiseOrExpression,
    BitwiseOrExpressionTerm,
    LogicalAndExpressionOp,
    LogicalAndExpression,
    LogicalAndExpressionTerm,
    LogicalOrExpressionOp,
    LogicalOrExpression,
    LogicalOrExpressionTerm,

    UnaryExpressionOp,
    UnaryExpression,
    UnaryExpressionTerm,
}

const WS_CHARS: CharClass = CharClass::new().chars(&[' ', '\n', '\r', '\t']);
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

pub enum DigitOrUnderscore {
    Digit(char),
    Underscore,
}

// Combines the given first and rest into a single Vec with a range spanning both
pub fn first_and_rest<R>(first: Parsed<R>, rest: Parsed<Vec<Parsed<R>>>) -> Parsed<Vec<Parsed<R>>> {
    let range = first.range.union(&rest.range);
    let value = std::iter::once(first).chain(rest.value.into_iter()).collect();
    Parsed {range, value}
}

// Collect digit characters into a single u32 parsed with the given radix, ignoring underscores
pub fn collect_digits(digits: &Vec<Parsed<DigitOrUnderscore>>, radix: u32) -> u32 {
    digits.iter().fold(0, |acc, v| {
        match v.value {
            DigitOrUnderscore::Digit(d) => (acc * radix) + d.to_digit(radix).unwrap(),
            _ => acc
        }
    })
}
