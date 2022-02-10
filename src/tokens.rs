#[derive(Debug)]
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
    CONTINUE
}
#[derive(Debug)]
pub enum Literal<'a> {
    STRING(&'a str), 
    SIGNEDINT(isize), 
    USIGNEDINT(usize), 
    FLOAT(f64),
    BOOL(bool),
}

#[derive(Debug)]
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

    DOT
}

#[derive(Debug)]
pub enum Separator {
    LCURLY,
    RCURLY,
    LPARAN,
    RPARAN,
    SEMICOLON,
    COMMA
}

#[derive(Debug)]
pub enum Token<'a> {
    KEYWORD(Keyword),
    IDENTIFIER(&'a str),
    SEPARATOR(Separator),
    OPERATOR(Operator),
    LITERAL(Literal<'a>),
    COMMENT(&'a str)
}