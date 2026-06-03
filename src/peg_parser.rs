use crate::parser::RuleName;

pub struct PegParserImpl<'a> {
    source: &'a str,
    // The position of the next character to be read
    pos: usize,
    // The start position of the current parse() call
    parse_start_pos: usize,
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

    fn not<R, F>(&mut self, f: F) -> Option<Parsed<()>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn to_parsed<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<R>;

    fn try_parse<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn parse<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>;

    fn parsed<R>(&self, value: R) -> Parsed<R>;
}

impl<'a> PegParserImpl<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            parse_start_pos: 0,
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
    // Runs for the given named rule.  FIXME - for future, use this to cache parse results (aka "packrat" parsing)
    fn for_rule<R, RN, F>(&mut self, _rule_name: RN, f: F)->Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        f(self)
    }

    // Executes the given function and returns its result.  If None is returned, then the cursor position is returned to where it was at the beginning of the parse() call.  Within the parse() call, the function can call parsed(value), which will wrap the value in a Parsed whose range spans the parse() call.
    fn parse<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>>,
    {
        let prev_parse_start_pos = self.parse_start_pos;

        let start = self.pos;
        self.parse_start_pos = start;
        if let Some(value) = f(self) {
            self.parse_start_pos = prev_parse_start_pos;
            Some(value)
        }
        else {
            self.parse_start_pos = prev_parse_start_pos;
            self.pos = start;
            None
        }
    }

    // When called within a parse() call, wraps the given value with a range that starts at the beginning of the parse() call and ends at the current position
    fn parsed<R>(&self, value: R) -> Parsed<R> {
        Parsed {
            range: ParsedRange {
                start: self.parse_start_pos,
                end: self.pos,
            },
            value,
        }
    }

    fn char_class(&mut self, char_class: &CharClass)->Option<Parsed<char>> {
        self.parse(|p| {
            if let Some(ch) = p.next_char() && char_class.matches(ch) {Some(p.parsed(ch))}
            else {None}
        })
    }
    
    fn ch(&mut self, match_ch: char)->Option<Parsed<char>> {
        self.parse(|p| {
            if let Some(ch) = p.next_char() && ch == match_ch {Some(p.parsed(ch))}
            else {None}
        })
    }
    
    fn str(&mut self, match_str: &'static str)->Option<Parsed<&'static str>> {
        self.parse(|p| {
            for match_ch in match_str.chars() {
                if let Some(ch) = p.next_char() && match_ch == ch {}
                else {return None}
            }
            Some(p.parsed(match_str))
        })
    }

    fn eof(&mut self) -> Option<Parsed<()>> {
        self.parse(|p| {
            if let Some(_) = p.peek() {None}
            else {Some(p.parsed(()))}
        })
    }

    fn opt<R, F>(&mut self, f: F) -> Option<Parsed<Option<R>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.parse(|p| {
            if let Some(Parsed {range, value}) = f(p) {
                Some(Parsed {range, value: Some(value)})
            }
            else {
                Some(p.parsed(None))
            }
        })
    }

    fn star<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.parse(|p| {
            let mut ret = Vec::<Parsed<R>>::new();
            while let Some(parsed) = f(p) {
                ret.push(parsed)
            }
            Some(p.parsed(ret))
        })
    }

    fn plus<R, F>(&mut self, f: F) -> Option<Parsed<Vec<Parsed<R>>>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.parse(|p| {
            let first_parsed = f(p)?;
            let mut ret = Vec::<Parsed<R>>::new();
            ret.push(first_parsed);
            while let Some(parsed) = f(p) {
                ret.push(parsed)
            }
            Some(p.parsed(ret))
        })
    }

    // Returns None if the function returns Some, or Some(()) if the function returns None
    fn not<R, F>(&mut self, f: F) -> Option<Parsed<()>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        self.parse(|p| {
            match f(p) {
                Some(_) => None,
                None => Some(p.parsed(()))
            }
        })
    }

    // FIXME - this should eventually go away
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

    // FIXME - this should eventually go away
    // Executes the given function.  If Some is returned, then the result is wrapped with the start and end character positions.  Otherwise, the position is reset to its starting point and None is returned
    fn try_parse<R, F>(&mut self, f: F) -> Option<Parsed<R>>
    where
        F: Fn(&mut Self)->Option<Parsed<R>> {
        let start = self.pos;
        if let Some(value) = f(self) {
            Some(value)
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

// Represents a set of characters and character ranges to be matched (or not matched if negated is true)
pub struct CharClass<'a> {
    negated: bool,
    singles: &'a [char],
    ranges: &'a [(char, char)],
}

impl<'a> CharClass<'a> {
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

    pub const fn chars(&self, singles: &'a [char]) -> Self {
        Self {
            negated: self.negated,
            singles,
            ranges: self.ranges,
        }
    }

    pub const fn ranges(&self, ranges: &'a [(char, char)]) -> Self {
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
