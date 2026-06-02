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
    
    pub fn identifier(&self, p: &mut impl PegParser) -> Option<Parsed<String>> {
        p.for_rule(RuleName::Identifier, |p| {
            p.to_parsed(|p| {
                let first = p.char_class(&IDENTIFIER_START_CHARS)?;
                let rest = p.star(|p| p.char_class(&IDENTIFIER_REST_CHARS))?;
                // Collect the Vec<Parsed<char>> into a String
                Some(first_and_rest(first, rest).iter().map(|i| i.value).collect::<String>())
            })
        })
    }

    pub fn boolean_literal(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BooleanLiteral, |p| {
            // FIXME - make sure these are followed by a WordBoundary
            if let Some(r) = p.str("true") {Some(r.with_value(ast::Expression::BooleanLiteral(true)))}
            else if let Some(r) = p.str("false") {Some(r.with_value(ast::Expression::BooleanLiteral(false)))}
            else {None}
        })
    }
    
    fn int_literal_radix(&self, p: &mut impl PegParser, prefix: &'static str, char_class: &CharClass, radix: u32, ast_radix: ast::Radix) -> Option<Parsed<ast::Expression>>
    {
        p.to_parsed(|p| {
            let _ = p.str(prefix)?;
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

    pub fn unary_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::UnaryExpression, |p| {
            p.to_parsed(|p| {
                let ops = p.star(|p| {
                    let _ = self.opt_sp(p)?;
                    self.op_str(p, &[
                        ("+", ast::UnaryOp::Plus),
                        ("-", ast::UnaryOp::Minus),
                        ("!", ast::UnaryOp::LogicalNot),
                        ("~", ast::UnaryOp::BitwiseNot),
                    ])
                })?;
                let exp = self.int_literal(p)?;
                Some(ast::Expression::unary_expression(ops.value, exp))
            })
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

    fn binary_expression<SF, PP>(&self, p: &mut PP, subexp: SF, op_strs: &[(&'static str, ast::BinaryOp)]) -> Option<Parsed<ast::Expression>>
    where
        PP: PegParser,
        SF: Fn(&mut PP)->Option<Parsed<ast::Expression>>
    {
        p.to_parsed(|p| {
            let first = subexp(p)?;
            let _ = self.opt_sp(p)?;
            let rest = p.star(|p| p.to_parsed(|p| {
                let _ = self.opt_sp(p)?;
                let op = self.op_str(p, op_strs)?;
                let _ = self.opt_sp(p)?;
                let exp = subexp(p)?;
                Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
            }))?.value;
            Some(ast::Expression::binary_expression(first, rest))
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

    pub fn add_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::AddExpression, |p| {
            self.binary_expression(p, |p| self.mult_expression(p), &[
                ("+", ast::BinaryOp::Plus),
                ("-", ast::BinaryOp::Minus),
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

    pub fn equality_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::EqualityExpression, |p| {
            self.binary_expression(p, |p| self.relational_expression(p), &[
                ("==", ast::BinaryOp::Equals),
                ("!=", ast::BinaryOp::NotEquals),
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

    pub fn bitwise_xor_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::BitwiseXorExpression, |p| {
            self.binary_expression(p, |p| self.bitwise_and_expression(p), &[
                ("^", ast::BinaryOp::BitwiseXor),
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

    pub fn logical_and_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalAndExpression, |p| {
            self.binary_expression(p, |p| self.bitwise_or_expression(p), &[
                ("&&", ast::BinaryOp::LogicalAnd),
            ])
        })
    }

    pub fn logical_or_expression(&self, p: &mut impl PegParser) -> Option<Parsed<ast::Expression>> {
        p.for_rule(RuleName::LogicalOrExpression, |p| {
            self.binary_expression(p, |p| self.logical_and_expression(p), &[
                ("||", ast::BinaryOp::LogicalOr),
            ])
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum RuleName {
    WsChar,
    Ws,
    Sp,
    OptSp,
    Identifier,
    BooleanLiteral,
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
pub fn first_and_rest<R>(first: Parsed<R>, rest: Parsed<Vec<Parsed<R>>>) -> Vec<Parsed<R>> {
    std::iter::once(first).chain(rest.value.into_iter()).collect()
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
