use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Keyword {
    LET,
    CONST,
    STATIC,
    FN,
    IF,
    ELSE,
    FOR,
    LOOP,
    BREAK,
    CONTINUE,
}
#[derive(Debug, Clone)]
pub enum Literal {
    STRING(Rc<str>),
    SIGNEDINT(isize),
    USIGNEDINT(usize),
    FLOAT(f64),
    BOOL(bool),
}

#[derive(Debug, Clone)]
pub enum Operator {
    EQUAL,
    NOTEQUAL,
    NOT,

    AND,
    BITAND,

    OR,
    BITOR,

    ASSIGNMENT,

    INCREMENT,
    INCASSIGN,
    ADD,

    DECREMENT,
    DECASSIGN,
    SUBTRACT,

    MULTASSIGN,
    MULTIPLY,

    DIVASSIGN,
    DIVIDE,

    DOT,
}

#[derive(Debug, Clone)]
pub enum Separator {
    LCURLY,
    RCURLY,
    LPARAN,
    RPARAN,
    SEMICOLON,
    COMMA,
}

#[derive(Debug, Clone)]
pub enum Token {
    KEYWORD(Keyword),
    IDENTIFIER(Rc<str>),
    SEPARATOR(Separator),
    OPERATOR(Operator),
    LITERAL(Literal),
    COMMENT(Rc<str>),
}
