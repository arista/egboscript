// FIXME - to reduce noise during initial development
#![allow(unused)]

use crate::ast;
use crate::peg_parser::{PegParser, CharClass, Rule, Parsed, ParseError};

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

pub fn parse(p: &mut PegParser) -> Result<Parsed<ast::Expression>, ParseError> {

    let ws_char: Rule<(),_> = p.add_rule(RuleName::WsChar, |p| Some(p.char_class(WS_CHARS)?.with_value(())))?;
    let ws: Rule<(),_> = p.add_rule(RuleName::Ws, |p| Some(p.plus(&ws_char)?.with_value(())))?;

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
    // Expressions

    let expression: Rule<ast::Expression,_> = p.add_rule(RuleName::Expression, |p| {
        if let Some(r) = p.rule(&boolean_literal) {Some(r)}
        else if let Some(r) = p.rule(&int_literal) {Some(r)}
        else {None}
    })?;

    // Main parse target
    if let Some(r) = p.rule(&expression) {
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
