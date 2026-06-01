// FIXME - to reduce noise during initial development
#![allow(unused)]

pub struct RuleCtx<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> RuleCtx<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
        }
    }
    
    // Return the character at the current position, None if EOF
    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    // Return the character at the current position and advance the position, None if EOF
    fn next_char(&mut self) -> Option<char> {
        let ret = self.peek();
        if let Some(ch) = ret {
            self.pos += ch.len_utf8();
        }
        ret
    }

    // Executes the given function.  If Some is returned, then the result is wrapped with the start and end character positions.  Otherwise, the position is reset to its starting point and None is returned
    pub fn to_parsed<R>(&mut self, f: impl FnOnce(&mut Self)->Option<R>) -> Option<Parsed<R>> {
        let start = self.pos;
        if let Some(value) = f(self) {
            Some(Parsed {range: ParsedRange {start, end: self.pos}, value})
        }
        else {
            self.pos = start;
            None
        }
    }

    pub fn eof(&mut self) -> Option<Parsed<()>> {
        self.to_parsed(|p| {
            if let Some(ch) = p.peek() {None}
            else {Some(())}
        })
    }

    pub fn ch(&mut self, match_ch: char) -> Option<Parsed<char>> {
        self.to_parsed(|p| {
            if let Some(ch) = p.next_char() && ch == match_ch {Some(ch)}
            else {None}
        })
    }

    pub fn char_class(&mut self, char_class: CharClass) -> Option<Parsed<char>> {
        self.to_parsed(|p| {
            if let Some(ch) = p.next_char() && char_class.matches(ch) {Some(ch)}
            else {None}
        })
    }

    pub fn str(&mut self, match_str: &'static str) -> Option<Parsed<&'static str>> {
        self.to_parsed(|p| {
            for match_ch in match_str.chars() {
                if let Some(ch) = p.next_char() && match_ch != ch {}
                else {return None}
            }
            Some(match_str)
        })
    }

    pub fn opt<R>(&mut self, f: impl FnOnce(&mut Self)->Option<Parsed<R>>) -> Option<Parsed<Option<R>>> {
        self.to_parsed(|p| {
            if let Some(Parsed {range, value}) = f(p) {
                Some(Some(value))
            }
            else {
                Some(None)
            }
        })
    }

    pub fn star<R>(&mut self, f: impl Fn(&mut Self)->Option<Parsed<R>>) -> Option<Parsed<Vec<Parsed<R>>>> {
        self.to_parsed(|p| {
            let mut ret = Vec::<Parsed<R>>::new();
            while let Some(parsed) = f(p) {
                ret.push(parsed)
            }
            Some(ret)
        })
    }

    pub fn plus<R>(&mut self, f: impl Fn(&mut Self)->Option<Parsed<R>>) -> Option<Parsed<Vec<Parsed<R>>>> {
        self.to_parsed(|p| {
            let first_parsed = f(p)?;
            let mut ret = Vec::<Parsed<R>>::new();
            ret.push(first_parsed);
            while let Some(parsed) = f(p) {
                ret.push(parsed)
            }
            Some(ret)
        })
    }

    pub fn parse<R>(&mut self, f: impl FnOnce(&mut Self)->Option<Parsed<R>>) -> Option<Parsed<R>> {
        f(self)
    }
}

#[derive(Debug)]
pub struct Parsed<R> {
    range: ParsedRange,
    value: R,
}

impl<R> Parsed<R> {
    // Replaces the value of the Parsed.  Typically used when a Parsed is generated from a primitive ("true"), but is then converted to its final value (boolean true)
    pub fn with_value<R2>(&self, value: R2) -> Parsed<R2> {
        Parsed {
            range: self.range,
            value
        }
    }

    // Convert the value of the Parsed.  Typically used when a Parsed is generated from a primitive resulting in a Vec<Parsed<...>>, which is then accumulated into its final form (String, number, etc.)
    pub fn map_value<R2>(&self, f: impl FnOnce(&R) -> R2) -> Parsed<R2> {
        let Parsed {range, value} = self;
        Parsed {
            range: *range,
            value: f(value),
        }
    }
}

// The range of character positions of a parsed construct
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ParsedRange {
    // The position of the first character
    start: usize,
    // The position *after* the last character
    end: usize,
}

impl ParsedRange {
    // Combine two ranges
    pub fn union(&self, r: &ParsedRange) -> ParsedRange {
        ParsedRange {
            start: *[self.start, r.start].iter().min().unwrap(),
            end: *[self.end, r.end].iter().max().unwrap(),
        }
    }
}

// Represents a set of characters and character ranges to be matched (or not matched if negated is true)
pub struct CharClass {
    negated: bool,
    singles: &'static [char],
    ranges: &'static [(char, char)],
}

impl CharClass {
    pub const fn new() -> Self {
        Self {
            negated: false,
            singles: &[],
            ranges: &[],
        }
    }

    pub const fn except(&self) -> Self {
        Self {
            negated: true,
            singles: self.singles,
            ranges: self.ranges,
        }
    }

    pub const fn chars(&self, singles: &'static [char]) -> Self {
        Self {
            negated: self.negated,
            singles,
            ranges: self.ranges,
        }
    }

    pub const fn ranges(&self, ranges: &'static [(char, char)]) -> Self {
        Self {
            negated: self.negated,
            singles: self.singles,
            ranges,
        }
    }
    
    pub fn matches(&self, ch: char) -> bool {
        if self.negated {
            !self.matches_non_negated(ch)
        }
        else {
            self.matches_non_negated(ch)
        }
    }

    fn matches_non_negated(&self, ch: char) -> bool {
        for sch in self.singles {
            if *sch == ch {return true}
        }
        for (start, end) in self.ranges {
            if ch >= *start && ch <= *end {return true}
        }
        false
    }
}

// Shorthand to create a rule, allowing it to be assigned to a variable with a type
pub fn rule<R, F>(rule_name: RuleName, rule: F) -> F
where F: Fn(&mut RuleCtx) -> Option<Parsed<R>> {
    rule
}

pub fn first_and_rest<R>(first: Parsed<R>, rest: Parsed<Vec<Parsed<R>>>) -> Parsed<Vec<Parsed<R>>> {
    let range = first.range.union(&rest.range);
    let value = std::iter::once(first).chain(rest.value.into_iter()).collect();
    Parsed {range, value}
}

pub fn collect_digits(digits: &Vec<Parsed<DigitOrUnderscore>>, radix: u32) -> u32 {
    digits.iter().fold(0, |acc, v| {
        match v.value {
            DigitOrUnderscore::Digit(d) => (acc * radix) + d,
            _ => acc
        }
    })
}

pub fn parse(ctx: &mut RuleCtx) -> Option<Parsed<u32>> {

    let ws_char = rule::<(), _>(RuleName::WsChar, |p| Some(p.char_class(WS_CHARS)?.with_value(())));
    let ws = rule::<(), _>(RuleName::Ws, |p| Some(p.plus(ws_char)?.with_value(())));

    let identifier = rule::<String, _>(RuleName::Identifier, |p| {
        let first = p.char_class(IDENTIFIER_START_CHARS)?;
        let rest = p.star(|p| p.char_class(IDENTIFIER_REST_CHARS))?;
        // Collect the Vec<Parsed<char>> into a String
        Some(first_and_rest(first, rest).map_value(|v| v.iter().map(|i| i.value).collect::<String>()))
    });

    let boolean_literal = rule::<bool, _>(RuleName::BooleanLiteral, |p| {
        if let Some(r) = p.str("true") {Some(r.with_value(true))}
        else if let Some(r) = p.str("false") {Some(r.with_value(false))}
        else {None}
    });

    let underscore_digit = rule::<DigitOrUnderscore, _>(RuleName::UnderscorDigit, |p| {
        Some(p.ch('_')?.with_value(DigitOrUnderscore::Underscore))
    });
    
    let decimal_digit = rule::<DigitOrUnderscore, _>(RuleName::DecimalDigit, |p| {
        Some(p.char_class(DECIMAL_DIGIT)?.map_value(|v| DigitOrUnderscore::Digit(v.to_digit(10).unwrap())))
    });
    let decimal_digit_or_underscore = rule::<DigitOrUnderscore, _>(RuleName::DecimalDigitOrUnderscore, |p| {
        p.parse(|p| {
            if let Some(r) = p.parse(underscore_digit) {Some(r)}
            else {p.parse(decimal_digit)}
        })
    });
    let decimal_digits = rule::<u32, _>(RuleName::DecimalDigits, |p| {
        let first = p.parse(decimal_digit)?;
        let rest = p.star(|p| p.parse(decimal_digit_or_underscore))?;
        Some(first_and_rest(first, rest).map_value(|v| collect_digits(v, 10)))
    });

    // Main parse target
    ctx.parse(decimal_digits)
}

pub enum RuleName {
    WsChar,
    Ws,
    Identifier,
    BooleanLiteral,
    UnderscorDigit,
    DecimalDigit,
    DecimalDigitOrUnderscore,
    DecimalDigits,
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

pub enum DigitOrUnderscore {
    Digit(u32),
    Underscore,
}
