pub fn parse() -> () {
}

// Immutable inputs to the parser, including the string to be parsed, parse options, etc.
pub struct Env<'a> {
    source: &'a str,
}

// Mutable state during parse
pub struct State {
}

// Immutable cursor
#[derive(Clone, Copy)]
pub struct Cursor {
    // The index of the next char to read
    pos: usize,
}

impl Cursor {
    pub fn peek(&self, env: &Env) -> NextChar {
        match env.source[self.pos..].chars().next() {
            Some(ch) => NextChar::Char(ch),
            None => NextChar::Eof,
        }
    }
    
    pub fn next(&self, env: &Env)->NextResult {
        let next_char = self.peek(env);
        match next_char {
            NextChar::Char(ch) => NextResult {
                ch: next_char,
                cursor: Cursor {
                    pos: self.pos + ch.len_utf8(),
                }
            },
            NextChar::Eof => NextResult {
                ch: next_char,
                cursor: *self,
            },
        }
    }
}

pub enum NextChar {
    Char(char),
    Eof,
}

pub struct NextResult {
    ch: NextChar,
    cursor: Cursor,
}

pub struct CharClass {
    negated: bool,
    singles: &'static [char],
    ranges: &'static [(char, char)],
}

impl CharClass {
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

pub struct ParseResult<R> {
    // The cursor to use for resuming parsing after this parsed construct
    next: Cursor,
    parsed: Parsed<R>
}

impl<R> ParseResult<R> {
    pub fn new(next: Cursor, start: Cursor, end: Cursor, value: R) -> Self {
        Self {
            next,
            parsed: Parsed::new(start, end, value),
        }
    }
}

pub struct Parsed<R> {
    range: ParsedRange,
    value: R,
}

impl<R> Parsed<R> {
    pub fn new(start: Cursor, end: Cursor, value: R) -> Self {
        Self {
            range: ParsedRange {start, end},
            value,
        }
    }
}

// The range of character positions of a parsed construct
pub struct ParsedRange {
    // The position of the first character
    start: Cursor,
    // The position *after* the last character
    end: Cursor,
}

pub fn parse_char_class(char_class: &CharClass, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<char>> {
    let next = cursor.next(env);
    match next.ch {
        NextChar::Char(ch) => {
            if char_class.matches(ch) {
                Some(ParseResult::new(next.cursor, cursor, cursor, ch))
            }
            else {None}
        }
        _ => {None}
    }        
}

pub fn parse_char(chr: char, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<char>> {
    let next = cursor.next(env);
    match next.ch {
        NextChar::Char(ch) => {
            if chr == ch {
                Some(ParseResult::new(next.cursor, cursor, cursor, ch))
            }
            else {None}
        }
        _ => {None}
    }        
}

pub fn parse_str(str: &'static str, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<&'static str>> {
    let mut c = cursor;
    for ch in str.chars() {
        c = parse_char(ch, c, env, state)?.next;
    }
    Some(ParseResult::new(c, cursor, c, str))
}

trait ParserFn<R>: Fn(Cursor, &Env, &mut State) -> Option<ParseResult<R>> {}

pub fn parse_optional<F, R>(f: F, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<Option<Parsed<R>>>>
where
    F: ParserFn<R>
{
    match f(cursor, env, state) {
        Some(ParseResult {next, parsed}) => {
            let ParsedRange {start, end} = parsed.range;
            Some(ParseResult::new(next, start, end, Some(parsed)))
        }
        None => {
            Some(ParseResult::new(cursor, cursor, cursor, None))
        }
    }
}

pub fn parse_star<F, R>(f: F, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<Vec<Parsed<R>>>>
where
    F: ParserFn<R>
{
    let mut ret = Vec::<Parsed<R>>::new();
    let mut c = cursor;
    while let Some(ParseResult {next, parsed}) = f(cursor, env, state) {
        c = next;
        ret.push(parsed)
    }
    Some(ParseResult::new(c, cursor, c, ret))
}

pub fn parse_plus<F, R>(f: F, cursor: Cursor, env: &Env, state: &mut State) -> Option<ParseResult<Vec<Parsed<R>>>>
where
    F: ParserFn<R>
{
    let mut ret = Vec::<Parsed<R>>::new();
    let mut c = cursor;
    if let Some(ParseResult {next, parsed}) = f(cursor, env, state) {
        c = next;
        ret.push(parsed);
        while let Some(ParseResult {next, parsed}) = f(cursor, env, state) {
            c = next;
            ret.push(parsed)
        }
        Some(ParseResult::new(c, cursor, c, ret))
    }
    else {
        None
    }
}
