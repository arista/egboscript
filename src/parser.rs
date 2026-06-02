// FIXME - to reduce noise during initial development
#![allow(unused)]

use crate::ast;
use crate::peg_parser::{PegParser, CharClass, Rule, Parsed, ParseError};

pub fn parse(p: &mut PegParser) -> Result<Parsed<ast::Expression>, ParseError> {

    //--------------------------------------------------
    // Ignorable space

    let ws_char: Rule<(),_> = p.add_rule(RuleName::WsChar, |p| Some(p.char_class(WS_CHARS)?.with_value(())))?;
    let ws: Rule<(),_> = p.add_rule(RuleName::Ws, |p| Some(p.plus(&ws_char)?.with_value(())))?;

    let sp: Rule<(),_> = p.add_rule(RuleName::Sp, |p| {
        if let Some(r) = p.rule(&ws) {Some(r)}
        else {None}
    })?;

    let opt_sp: Rule<(),_> = p.add_rule(RuleName::OptSp, |p| {
        if let Some(r) = p.opt(&sp) {Some(r.with_value(()))}
        else {None}
    })?;

    //--------------------------------------------------
    // Identifiers

    let identifier_start_char: Rule<char,_> = p.add_rule(RuleName::IdentifierStartChar, |p| {
        p.char_class(IDENTIFIER_START_CHARS)
    })?;
    let identifier_rest_char: Rule<char,_> = p.add_rule(RuleName::IdentifierRestChar, |p| {
        p.char_class(IDENTIFIER_REST_CHARS)
    })?;
    
    let identifier: Rule<String,_> = p.add_rule(RuleName::Identifier, |p| {
        let first = p.rule(&identifier_start_char)?;
        let rest = p.star(&identifier_rest_char)?;
        // Collect the Vec<Parsed<char>> into a String
        Some(first_and_rest(first, rest).map_value(|v| v.iter().map(|i| i.value).collect::<String>()))
    })?;

    //--------------------------------------------------
    // Boolean Literal
    
    let boolean_literal: Rule<ast::Expression,_> = p.add_rule(RuleName::BooleanLiteral, |p| {
        if let Some(r) = p.str("true") {Some(r.with_value(ast::Expression::BooleanLiteral(true)))}
        else if let Some(r) = p.str("false") {Some(r.with_value(ast::Expression::BooleanLiteral(false)))}
        else {None}
    })?;

    //--------------------------------------------------
    // Int Literal
    
    let underscore_digit: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::UnderscorDigit, |p| {
        Some(p.ch('_')?.with_value(DigitOrUnderscore::Underscore))
    })?;
    
    let decimal_digit: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::DecimalDigit, |p| {
        Some(p.char_class(DECIMAL_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
    })?;
    let decimal_digit_or_underscore: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::DecimalDigitOrUnderscore, |p| {
        if let Some(r) = p.rule(&underscore_digit) {Some(r)}
        else {p.rule(&decimal_digit)}
    })?;
    let decimal_digits: Rule<u32,_> = p.add_rule(RuleName::DecimalDigits, |p| {
        let first = p.rule(&decimal_digit)?;
        let rest = p.star(&decimal_digit_or_underscore)?;
        Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 10)))
    })?;

    
    let hex_digit: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::HexDigit, |p| {
        Some(p.char_class(HEX_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
    })?;
    let hex_digit_or_underscore: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::HexDigitOrUnderscore, |p| {
        if let Some(r) = p.rule(&underscore_digit) {Some(r)}
        else {p.rule(&hex_digit)}
    })?;
    let hex_digits: Rule<u32,_> = p.add_rule(RuleName::HexDigits, |p| {
        let first = p.rule(&hex_digit)?;
        let rest = p.star(&hex_digit_or_underscore)?;
        Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 16)))
    })?;
    let hex_literal: Rule<u32,_> = p.add_rule(RuleName::HexLiteral, |p| {
        let _ = p.str("0x")?;
        Some(p.rule(&hex_digits)?)
    })?;
    
    let octal_digit: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::OctalDigit, |p| {
        Some(p.char_class(OCTAL_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
    })?;
    let octal_digit_or_underscore: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::OctalDigitOrUnderscore, |p| {
        if let Some(r) = p.rule(&underscore_digit) {Some(r)}
        else {p.rule(&octal_digit)}
    })?;
    let octal_digits: Rule<u32,_> = p.add_rule(RuleName::OctalDigits, |p| {
        let first = p.rule(&octal_digit)?;
        let rest = p.star(&octal_digit_or_underscore)?;
        Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 8)))
    })?;
    let octal_literal: Rule<u32,_> = p.add_rule(RuleName::OctalLiteral, |p| {
        let _ = p.str("0o")?;
        Some(p.rule(&octal_digits)?)
    })?;
    
    let binary_digit: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::BinaryDigit, |p| {
        Some(p.char_class(BINARY_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(*v)))
    })?;
    let binary_digit_or_underscore: Rule<DigitOrUnderscore,_> = p.add_rule(RuleName::BinaryDigitOrUnderscore, |p| {
        if let Some(r) = p.rule(&underscore_digit) {Some(r)}
        else {p.rule(&binary_digit)}
    })?;
    let binary_digits: Rule<u32,_> = p.add_rule(RuleName::BinaryDigits, |p| {
        let first = p.rule(&binary_digit)?;
        let rest = p.star(&binary_digit_or_underscore)?;
        Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 2)))
    })?;
    let binary_literal: Rule<u32,_> = p.add_rule(RuleName::BinaryLiteral, |p| {
        let _ = p.str("0b")?;
        Some(p.rule(&binary_digits)?)
    })?;

    let int_literal: Rule<ast::Expression,_> = p.add_rule(RuleName::IntLiteral, |p| {
        if let Some(r) = p.rule(&hex_literal) {
            Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Hex)))
        }
        else if let Some(r) = p.rule(&octal_literal) {
            Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Octal)))
        }
        else if let Some(r) = p.rule(&binary_literal) {
            Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Binary)))
        }
        else if let Some(r) = p.rule(&decimal_digits) {
            Some(r.map_value(|v| ast::Expression::u32_literal(*v, ast::Radix::Decimal)))
        }
        else {
            None
        }
    })?;

    //--------------------------------------------------
    // UnaryExpression

    let unary_expression_op: Rule<ast::UnaryOp,_> = p.add_rule(RuleName::UnaryExpressionOp, |p| {
        if let Some(r) = p.ch('+') {Some(r.with_value(ast::UnaryOp::Plus))}
        else if let Some(r) = p.ch('-') {Some(r.with_value(ast::UnaryOp::Minus))}
        else if let Some(r) = p.ch('!') {Some(r.with_value(ast::UnaryOp::LogicalNot))}
        else if let Some(r) = p.ch('~') {Some(r.with_value(ast::UnaryOp::BitwiseNot))}
        else {None}
    })?;

    let unary_expression_term: Rule<ast::UnaryOp,_> = p.add_rule(RuleName::UnaryExpressionTerm, |p| {
        let _ = p.rule(&opt_sp)?;
        p.rule(&unary_expression_op)
    })?;

    let unary_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::UnaryExpression, |p| {
        p.to_parsed(|p| {
            let ops = p.star(&unary_expression_term)?;
            let exp = p.rule(&int_literal)?;
            Some(ast::Expression::unary_expression(ops.value, exp))
        })
    })?;

    //--------------------------------------------------
    // MultExpression

    let mult_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::MultExpressionOp, |p| {
        if let Some(r) = p.ch('*') {Some(r.with_value(ast::BinaryOp::Times))}
        else if let Some(r) = p.ch('/') {Some(r.with_value(ast::BinaryOp::Divide))}
        else if let Some(r) = p.ch('%') {Some(r.with_value(ast::BinaryOp::Mod))}
        else {None}
    })?;

    let mult_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::MultExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&mult_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&unary_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let mult_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::MultExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&unary_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&mult_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // AddExpression

    let add_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::AddExpressionOp, |p| {
        if let Some(r) = p.ch('+') {Some(r.with_value(ast::BinaryOp::Plus))}
        else if let Some(r) = p.ch('-') {Some(r.with_value(ast::BinaryOp::Minus))}
        else {None}
    })?;

    let add_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::AddExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&add_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&mult_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let add_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::AddExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&mult_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&add_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // BitshiftExpression

    let bitshift_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::BitshiftExpressionOp, |p| {
        if let Some(r) = p.str("<=") {Some(r.with_value(ast::BinaryOp::LessThanOrEquals))}
        else if let Some(r) = p.str("<") {Some(r.with_value(ast::BinaryOp::LessThan))}
        else if let Some(r) = p.str(">=") {Some(r.with_value(ast::BinaryOp::GreaterThanOrEquals))}
        else if let Some(r) = p.str(">") {Some(r.with_value(ast::BinaryOp::GreaterThan))}
        else {None}
    })?;

    let bitshift_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::BitshiftExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&bitshift_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&add_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let bitshift_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::BitshiftExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&add_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&bitshift_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // RelationalExpression

    let relational_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::RelationalExpressionOp, |p| {
        if let Some(r) = p.str("<<") {Some(r.with_value(ast::BinaryOp::ShiftLeft))}
        else if let Some(r) = p.str(">>>") {Some(r.with_value(ast::BinaryOp::LogicalShiftRight))}
        else if let Some(r) = p.str(">>") {Some(r.with_value(ast::BinaryOp::ArithmeticShiftRight))}
        else {None}
    })?;

    let relational_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::RelationalExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&relational_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&bitshift_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let relational_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::RelationalExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&bitshift_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&relational_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // EqualityExpression

    let equality_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::EqualityExpressionOp, |p| {
        if let Some(r) = p.str("==") {Some(r.with_value(ast::BinaryOp::Equals))}
        else if let Some(r) = p.str("!=") {Some(r.with_value(ast::BinaryOp::NotEquals))}
        else {None}
    })?;

    let equality_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::EqualityExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&equality_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&relational_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let equality_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::EqualityExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&relational_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&equality_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // BitwiseAndExpression

    let bitwise_and_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::BitwiseAndExpressionOp, |p| {
        if let Some(r) = p.str("&") {Some(r.with_value(ast::BinaryOp::BitwiseAnd))}
        else {None}
    })?;

    let bitwise_and_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::BitwiseAndExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&bitwise_and_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&equality_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let bitwise_and_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::BitwiseAndExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&equality_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&bitwise_and_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // BitwiseXorExpression

    let bitwise_xor_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::BitwiseXorExpressionOp, |p| {
        if let Some(r) = p.str("^") {Some(r.with_value(ast::BinaryOp::BitwiseXor))}
        else {None}
    })?;

    let bitwise_xor_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::BitwiseXorExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&bitwise_xor_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&bitwise_and_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let bitwise_xor_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::BitwiseXorExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&bitwise_and_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&bitwise_xor_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // BitwiseOrExpression

    let bitwise_or_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::BitwiseOrExpressionOp, |p| {
        if let Some(r) = p.str("|") {Some(r.with_value(ast::BinaryOp::BitwiseOr))}
        else {None}
    })?;

    let bitwise_or_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::BitwiseOrExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&bitwise_or_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&bitwise_xor_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let bitwise_or_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::BitwiseOrExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&bitwise_xor_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&bitwise_or_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // LogicalAndExpression

    let logical_and_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::LogicalAndExpressionOp, |p| {
        if let Some(r) = p.str("&&") {Some(r.with_value(ast::BinaryOp::LogicalAnd))}
        else {None}
    })?;

    let logical_and_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::LogicalAndExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&logical_and_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&bitwise_or_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let logical_and_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::LogicalAndExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&bitwise_or_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&logical_and_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // LogicalOrExpression

    let logical_or_expression_op: Rule<ast::BinaryOp,_> = p.add_rule(RuleName::LogicalOrExpressionOp, |p| {
        if let Some(r) = p.str("&&") {Some(r.with_value(ast::BinaryOp::LogicalOr))}
        else {None}
    })?;

    let logical_or_expression_term: Rule<ast::BinaryExpressionTerm,_> = p.add_rule(RuleName::LogicalOrExpressionTerm, |p| {
        p.to_parsed(|p| {
            let _ = p.rule(&opt_sp)?;
            let op = p.rule(&logical_or_expression_op)?;
            let _ = p.rule(&opt_sp)?;
            let exp = p.rule(&logical_and_expression)?;
            Some(ast::BinaryExpressionTerm {op, exp: Box::new(exp)})
        })
    })?;

    let logical_or_expression: Rule<ast::Expression,_> = p.add_rule(RuleName::LogicalOrExpression, |p| {
        p.to_parsed(|p| {
            let first = p.rule(&logical_and_expression)?;
            let _ = p.rule(&opt_sp)?;
            let rest = p.star(&logical_or_expression_term)?.value;
            Some(ast::Expression::binary_expression(first, rest))
        })
    })?;

    //--------------------------------------------------
    // Expressions

    let expression: Rule<ast::Expression,_> = p.add_rule(RuleName::Expression, |p| {
        if let Some(r) = p.rule(&boolean_literal) {Some(r)}
        else if let Some(r) = p.rule(&int_literal) {Some(r)}
        else {None}
    })?;

    // Main parse target
//    if let Some(r) = p.rule(&expression) {
    if let Some(r) = p.rule(&logical_or_expression) {
        Ok(r)
    }
    else {
        // FIXME - let the PegParser generate the error based on its state
        Err(ParseError::ParseFailed)
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
