use crate::tokens::{Literal, Operator, Separator, Token, Keyword};

enum LexerState {
    DEFAULT,

    // Literals
    LITSTRING,
    LITNUMERIC,
    LITBOOL,
    //
}

pub struct Lexer {
    source: String,
    state: LexerState,
    pos: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            state: LexerState::DEFAULT,
            pos: 0usize,
        }
    }
    fn consume<T>(&mut self, val: T) -> T {
        self.pos += 1;
        val
    }
    fn lookahead_token<'a>(
        &mut self,
        first: Option<char>,
        second: Option<char>,
    ) -> Option<Token<'a>> {
        match first {
            Some('{') => Some(Token::SEPARATOR(Separator::LCURLY)),
            Some('}') => Some(Token::SEPARATOR(Separator::RCURLY)),

            Some('(') => Some(Token::SEPARATOR(Separator::LPARAN)),
            Some(')') => Some(Token::SEPARATOR(Separator::RPARAN)),

            Some(';') => Some(Token::SEPARATOR(Separator::SEMICOLON)),
            Some(',') => Some(Token::SEPARATOR(Separator::COMMA)),
            Some('.') => Some(Token::OPERATOR(Operator::DOT)),

            Some('=') => match second {
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::EQUAL))),
                None => Some(Token::OPERATOR(Operator::ASSIGNMENT)),
                _ => None,
            },
            Some('!') => match second {
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::NOTEQUAL))),
                _ => Some(Token::OPERATOR(Operator::NOT)),
            },
            Some('+') => match second {
                Some('+') => self.consume(Some(Token::OPERATOR(Operator::INCREMENT))),
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::INCASSIGN))),
                None => Some(Token::OPERATOR(Operator::ADD)),
                _ => None,
            },
            Some('-') => match second {
                Some('-') => self.consume(Some(Token::OPERATOR(Operator::DECREMENT))),
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::DECASSIGN))),
                None => Some(Token::OPERATOR(Operator::SUBTRACT)),
                _ => None,
            },
            Some('*') => match second {
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::MULTASSIGN))),
                None => Some(Token::OPERATOR(Operator::MULTIPLY)),
                _ => None,
            },
            Some('/') => match second {
                Some('=') => self.consume(Some(Token::OPERATOR(Operator::DIVASSIGN))),
                None => Some(Token::OPERATOR(Operator::DIVIDE)),
                _ => None,
            },
            Some('&') => match second {
                Some('&') => self.consume(Some(Token::OPERATOR(Operator::AND))),
                None => Some(Token::OPERATOR(Operator::BITAND)),
                _ => None,
            },
            Some('|') => match second {
                Some('|') => self.consume(Some(Token::OPERATOR(Operator::OR))),
                None => Some(Token::OPERATOR(Operator::BITOR)),
                _ => None,
            },
            _ => None,
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        let (first, second) = self.get_filtered_chars();
        let token = self.lookahead_token(first, second);
        if token.is_some() {
            return token;
        }
        match first {
            Some('"') => self.process_string_literal(),
            Some('0'..='9' | '-') => self.process_numeric_literal(),

            None => None,
            _ => {
                let maybe_bool = self.process_boolean_literal();
                if maybe_bool.is_some() {
                    return maybe_bool;
                }
                self.process_identifier()
            }
        }
    }

    fn process_numeric_literal(&mut self) -> Option<Token> {
        self.state = LexerState::LITSTRING;
        let start = self.pos;
        loop {
            if let Some(next_char) = self.get_char() {
                match next_char {
                    '0'..='9' | '-' | '.' => (),
                    'u' => {
                        let raw = &self.source[start - 1..self.pos - 1];
                        return Some(Token::LITERAL(Literal::USIGNEDINT(raw.parse().unwrap())));
                    }
                    'i' => {
                        let raw = &self.source[start - 1..self.pos - 1];
                        return Some(Token::LITERAL(Literal::SIGNEDINT(raw.parse().unwrap())));
                    }
                    'f' => {
                        let raw = &self.source[start - 1..self.pos - 1];
                        return Some(Token::LITERAL(Literal::FLOAT(raw.parse().unwrap())));
                    }
                    _ => (),
                }
                continue;
            }
            return None;
        }
    }
    fn process_boolean_literal<'a>(&mut self) -> Option<Token<'a>> {
        self.state = LexerState::LITSTRING;
        let start = self.pos;
        if self.pos + 3 > self.source.len() {
            return None;
        }
        let maybe_bool = &self.source[start - 1..self.pos + 3];
        if maybe_bool.eq("true") {
            self.pos += 3;
            return Some(Token::LITERAL(Literal::BOOL(true)));
        }
        if self.pos + 4 > self.source.len() {
            return None;
        }
        let maybe_bool = &self.source[start - 1..self.pos + 4];
        if maybe_bool.eq("false") {
            self.pos += 4;
            return Some(Token::LITERAL(Literal::BOOL(false)));
        }
        None
    }
    fn process_string_literal(&mut self) -> Option<Token> {
        self.state = LexerState::LITNUMERIC;
        let start = self.pos;
        loop {
            if let Some(next_char) = self.get_char() {
                if next_char == '"' {
                    return Some(Token::LITERAL(Literal::STRING(
                        &self.source[start..self.pos - 1],
                    )));
                }
                continue;
            }
            return None;
        }
    }

    fn process_identifier(&mut self) -> Option<Token> {
        let start = self.pos;
        let token = loop {
            let (first, second) = self.get_chars();
            if let Some(c) = first {
                let pos = self.pos;
                if self.lookahead_token(first, second).is_some() || Self::is_whitespace(c) {
                    self.pos = pos - 1;
                    break Some(self.source[start-1..self.pos].trim());
                }
                continue;
            }
            break None;
        };

        match token {
            Some("let") => Some(Token::KEYWORD(Keyword::LET)),
            Some("const") => Some(Token::KEYWORD(Keyword::CONST)),
            Some("static") => Some(Token::KEYWORD(Keyword::STATIC)),
            Some("fn") => Some(Token::KEYWORD(Keyword::FN)),
            Some("if") => Some(Token::KEYWORD(Keyword::IF)),
            Some("else") => Some(Token::KEYWORD(Keyword::ELSE)),
            Some("for") => Some(Token::KEYWORD(Keyword::FOR)),
            Some("loop") => Some(Token::KEYWORD(Keyword::LOOP)),
            Some("break") => Some(Token::KEYWORD(Keyword::BREAK)),
            Some("continue") => Some(Token::KEYWORD(Keyword::CONTINUE)),
            Some(identifier) => Some(Token::IDENTIFIER(identifier)),
            None => None
        }
    }

    fn get_char(&mut self) -> Option<char> {
        self.pos += 1;
        self.source.chars().skip(self.pos - 1).next()
    }
    fn get_chars(&mut self) -> (Option<char>, Option<char>) {
        self.pos += 1;
        let mut it = self.source.chars().skip(self.pos - 1);
        (it.next(), it.next())
    }
    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false
        }
    }
    fn get_filtered_chars(&mut self) -> (Option<char>, Option<char>) {
        let first = loop {
            let potential = self.get_char();
            match potential {
                Some(' ' | '\t' | '\n' | '\r') => {
                    continue;
                }
                None => {
                    break None;
                }
                _ => {
                    break potential;
                }
            }
        };
        let second = {
            let potential = self.get_char();
            self.pos -= 1;
            match potential {
                Some(' ' | '\t' | '\n' | '\r') => None,
                None => None,
                _ => potential,
            }
        };
        (first, second)
    }
}
