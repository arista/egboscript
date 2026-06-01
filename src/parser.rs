pub struct Parser<'a> {
    source: &'a str,
}

impl<'a> Parser<'a> {
    // Return the character at the cursor, None if EOF
    pub fn peek(&self, cursor: Cursor) -> Option<char> {
        self.source[cursor.pos..].chars().next()
    }

    // Return the character, and the new position of the Cursor after consuming that character
    pub fn next_char(&self, cursor: Cursor) -> (Option<char>, Cursor) {
        match self.peek(cursor) {
            Some(ch) => (Some(ch), cursor.advance(ch)),
            None => (None, cursor),
        }
    }

    pub fn char_class(&mut self, cursor: Cursor, char_class: &CharClass) -> Option<ParseResult<char>> {
        let (ch_opt, next_cursor) = self.next_char(cursor);
        if let Some(ch) = ch_opt && char_class.matches(ch) {
            Some(ParseResult::new(next_cursor, cursor, cursor, ch))
        }
        else {
            None
        }
    }

    pub fn ch(&mut self, cursor: Cursor, match_ch: char) -> Option<ParseResult<char>> {
        let (ch_opt, next_cursor) = self.next_char(cursor);
        if let Some(ch) = ch_opt && match_ch == ch {
            Some(ParseResult::new(next_cursor, cursor, cursor, ch))
        }
        else {
            None
        }
    }

    pub fn string(&mut self, cursor: Cursor, str: &'static str) -> Option<ParseResult<&'static str>> {
        let mut c = cursor;
        for ch in str.chars() {
            c = self.ch(c, ch)?.next;
        }
        Some(ParseResult::new(c, cursor, c, str))
    }

    pub fn optional<F, R>(&mut self, cursor: Cursor, f: F) -> Option<ParseResult<Option<Parsed<R>>>>
    where
        F: ParserFn<R>
    {
        if let Some(ParseResult {next, parsed}) = f(self, cursor) {
            let ParsedRange {start, end} = parsed.range;
            Some(ParseResult::new(next, start, end, Some(parsed)))
        }
        else {
            Some(ParseResult::new(cursor, cursor, cursor, None))
        }
    }

    pub fn star<F, R>(&mut self, cursor: Cursor, f: F) -> Option<ParseResult<Vec<Parsed<R>>>>
    where
        F: ParserFn<R>
    {
        let mut v = Vec::<Parsed<R>>::new();
        let mut c = cursor;
        while let Some(ParseResult {next, parsed}) = f(self, c) {
            c = next;
            let ParsedRange {start, end} = parsed.range;
            v.push(parsed);
        }
        Some(ParseResult::new(c, cursor, c, v))
    }

    pub fn plus<F, R>(&mut self, cursor: Cursor, f: F) -> Option<ParseResult<Vec<Parsed<R>>>>
    where
        F: ParserFn<R>
    {
        if let Some(ParseResult {next, parsed}) = f(self, cursor) {
            let mut v = Vec::<Parsed<R>>::new();
            v.push(parsed);
            let mut c = next;
            while let Some(ParseResult {next, parsed}) = f(self, c) {
                c = next;
                let ParsedRange {start, end} = parsed.range;
                v.push(parsed);
            }
            Some(ParseResult::new(c, cursor, c, v))
        }
        else {
            None
        }
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

// Immutable cursor
#[derive(Clone, Copy)]
pub struct Cursor {
    // The index of the next char to read
    pos: usize,
}

impl Cursor {
    pub fn advance(&self, ch: char) -> Cursor {
        Cursor {pos: self.pos + ch.len_utf8()}
    }
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

trait ParserFn<R>: Fn(&mut Parser, Cursor) -> Option<ParseResult<R>> {}
