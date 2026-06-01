use crate::parser::RuleName;
use std::collections::HashSet;

pub struct PegParser<'a> {
    source: &'a str,
    pos: usize,
    rule_names: HashSet<RuleName>
}

#[derive(Debug)]
pub enum ParseError {
    RuleNameUsedMultipleTimes(RuleName),
    ParseFailed,
}

impl<'a> PegParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            rule_names: HashSet::new(),
        }
    }

    // Create and register a Rule
    pub fn add_rule<R, F>(&mut self, rule_name: RuleName, rule: F) -> Result<Rule<R, F>, ParseError>
    where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
        if self.rule_names.contains(&rule_name) {
            Err(ParseError::RuleNameUsedMultipleTimes(rule_name))
        }
        else {
            self.rule_names.insert(rule_name);
            Ok(Rule { _rule_name: rule_name, rule })
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
    pub fn to_parsed<R>(&mut self, f: impl Fn(&mut Self)->Option<R>) -> Option<Parsed<R>> {
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
            if let Some(_) = p.peek() {None}
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
                if let Some(ch) = p.next_char() && match_ch == ch {}
                else {return None}
            }
            Some(match_str)
        })
    }

    pub fn opt<R, F>(&mut self, rule: &Rule<R, F>) -> Option<Parsed<Option<R>>>
    where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
        self.to_parsed(|p| {
            if let Some(Parsed {range: _, value}) = (rule.rule)(p) {
                Some(Some(value))
            }
            else {
                Some(None)
            }
        })
    }

    pub fn star<R, F>(&mut self, rule: &Rule<R, F>) -> Option<Parsed<Vec<Parsed<R>>>>
    where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
        self.to_parsed(|p| {
            let mut ret = Vec::<Parsed<R>>::new();
            while let Some(parsed) = (rule.rule)(p) {
                ret.push(parsed)
            }
            Some(ret)
        })
    }

    pub fn plus<R, F>(&mut self, rule: &Rule<R, F>) -> Option<Parsed<Vec<Parsed<R>>>>
    where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
        self.to_parsed(|p| {
            let first_parsed = (rule.rule)(p)?;
            let mut ret = Vec::<Parsed<R>>::new();
            ret.push(first_parsed);
            while let Some(parsed) = (rule.rule)(p) {
                ret.push(parsed)
            }
            Some(ret)
        })
    }

    pub fn rule<R, F>(&mut self, rule: &Rule<R, F>) -> Option<Parsed<R>>
    where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
        self.to_parsed(|p| Some((rule.rule)(p)?.value))
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

pub struct Rule<R, F>
where F: Fn(&mut PegParser) -> Option<Parsed<R>> {
    _rule_name: RuleName,
    rule: F,
}
