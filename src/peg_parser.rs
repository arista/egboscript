use crate::parser::RuleName;

pub struct PegParserImpl<'a> {
    source: &'a str,
    pos: usize,
}

#[derive(Debug)]
pub enum ParseError {
    RuleNameUsedMultipleTimes(RuleName),
    ParseFailed,
}

pub trait PegParser {
    fn for_rule<R, RN, F>(&mut self, rule_name: RN, f: F)->Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn char_class(&mut self, char_class: &CharClass)->Option<Parsed<char>>;
    fn ch(&mut self, match_ch: char)->Option<Parsed<char>>;
    fn str(&mut self, match_str: &'static str)->Option<Parsed<&'static str>>;
    fn eof(&mut self)->Option<Parsed<()>>;

    fn opt<R, F>(&mut self, f: F) -> Option<Parsed<Option<R>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn star<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn plus<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn to_parsed<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<R>;
}

impl<'a> PegParserImpl<'a> {
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
}

impl<'a> PegParser for PegParserImpl<'a> {
    fn for_rule<R, RN, F>(&mut self, _rule_name: RN, f: F)->Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        f(self)
    }

    fn char_class(&mut self, char_class: &CharClass)->Option<Parsed<char>> {
        self.to_parsed(|p| {
            if let Some(ch) = p.next_char() && char_class.matches(ch) {Some(ch)}
            else {None}
        })
    }
    
    fn ch(&mut self, match_ch: char)->Option<Parsed<char>> {
        self.to_parsed(|p| {
            if let Some(ch) = p.next_char() && ch == match_ch {Some(ch)}
            else {None}
        })
    }
    
    fn str(&mut self, match_str: &'static str)->Option<Parsed<&'static str>> {
        self.to_parsed(|p| {
            for match_ch in match_str.chars() {
                if let Some(ch) = p.next_char() && match_ch == ch {}
                else {return None}
            }
            Some(match_str)
        })
    }

    fn eof(&mut self) -> Option<Parsed<()>> {
        self.to_parsed(|p| {
            if let Some(_) = p.peek() {None}
            else {Some(())}
        })
    }

    fn opt<R, F>(&mut self, f: F) -> Option<Parsed<Option<R>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.to_parsed(|p| {
            if let Some(Parsed {range: _, value}) = f(p) {
                Some(Some(value))
            }
            else {
                Some(None)
            }
        })
    }

    fn star<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.to_parsed(|p| {
            let mut ret = Vec::<Parsed<R>>::new();
            while let Some(parsed) = f(p) {
                ret.push(parsed)
            }
            Some(ret)
        })
    }

    fn plus<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
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

    // Executes the given function.  If Some is returned, then the result is wrapped with the start and end character positions.  Otherwise, the position is reset to its starting point and None is returned
    fn to_parsed<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<R> {
        let start = self.pos;
        if let Some(value) = f(self) {
            Some(Parsed {range: ParsedRange {start, end: self.pos}, value})
        }
        else {
            self.pos = start;
            None
        }
    }
}

#[derive(Debug)]
pub struct Parsed<R> {
    pub range: ParsedRange,
    pub value: R,
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
