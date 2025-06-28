#[derive(logos::Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    /* math operators */
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    /* unary operators */
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,
    #[token("!")]
    Bang,

    /* assignment operators */
    #[token("=")]
    Equal,

    /* boolean operators */
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    /* misc tokens */
    #[token("->")]
    ArrowSmall,
    #[token("=>")]
    ArrowBig,

    /* broad types */
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),
    #[regex(r#""([^"\\]|\\[nrt"\\])*""#, |lex| {
        let slice = lex.slice();

        let content = &slice[1..slice.len()-1];
        Some(content.replace("\\n", "\n")
                   .replace("\\r", "\r")
                   .replace("\\t", "\t")
                   .replace("\\\"", "\"")
                   .replace("\\\\", "\\"))
    })]
    String(String),
    #[regex(r"true|false", |lex| match lex.slice() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    })]
    Boolean(bool),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),

    /* types */
    #[token("i32")]
    KeywordTypeI32,
    #[token("i64")]
    KeywordTypeI64,
    #[token("u32")]
    KeywordTypeU32,
    #[token("u64")]
    KeywordTypeU64,
    #[token("f32")]
    KeywordTypeF32,
    #[token("f64")]
    KeywordTypeF64,
    #[token("bool")]
    KeywordTypeBool,
    #[token("String")]
    KeywordTypeString,

    /* delimiters */
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("...")]
    Ellipsis,

    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("let")]
    KeywordLet,
}
