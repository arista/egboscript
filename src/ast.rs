#[derive(Debug)]
pub enum Expression {
    BooleanLiteral(bool),
    U32Literal(U32Literal),
}

impl Expression {
    pub fn u32_literal(val: u32, radix: Radix) -> Self {
        Self::U32Literal(U32Literal {val, radix})
    }
}

#[derive(Debug)]
pub struct U32Literal {
    pub val: u32,
    pub radix: Radix,
}

// The source radix of an int literal
#[derive(Debug)]
pub enum Radix {
    Decimal,
    Hex,
    Octal,
    Binary,
}
