mod tests;
use lifering::FloatingPointComponents;
use logos::{Filter, Lexer, Logos};

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token<'a> {
    /// The "`+`" operator.
    #[token("+")]
    Add,

    /// The "`and`" operator.
    #[token("and")]
    And,

    /// The "`=`" operator.
    #[token("=")]
    Assign,

    /// The "`break`" keyword.
    #[token("break")]
    Break,

    /// The "`]`" token.
    #[token("]")]
    CloseBracket,

    /// The "`}`" token.
    #[token("}")]
    CloseCurly,

    /// The "`)`" token.
    #[token(")")]
    CloseParen,

    /// The "`:`" operator.
    #[token(":")]
    Colon,

    /// The "`,`" token.
    #[token(",")]
    Comma,

    /// Line and block comments, line comments are eaten by
    /// the lexer, but block comments are left for the parser
    /// to handle.
    #[token("--", comment)]
    CommentStart,

    /// The "`..`" operator.
    #[token("..")]
    Concat,

    /// The "`/`" operator.
    #[token("/")]
    Divide,

    /// The "`do`" keyword.
    #[token("do")]
    Do,

    /// The "`else`" keyword.
    #[token("else")]
    Else,

    /// The "`elseif`" keyword.
    #[token("elseif")]
    ElseIf,

    /// The "`end`" keyword.
    #[token("end")]
    End,

    /// The "`==`" operator.
    #[token("==")]
    Eq,

    /// Error token, also responsible for eating whitespace.
    #[error]
    #[regex(r"\s", logos::skip)]
    Error,

    /// The "`^`" operator.
    #[token("^")]
    Exponentiate,

    /// The "`false`" keyword.
    #[token("false")]
    False,

    /// The "`for`" keyword.
    #[token("for")]
    For,

    /// The "`function`" keyword.
    #[token("function")]
    Function,

    /// The "`>=`" operator.
    #[token(">=")]
    Ge,

    /// The "`>`" operator.
    #[token(">")]
    Gt,

    /// All identifiers.
    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier(&'a str),

    /// The "`if`" keyword.
    #[token("if")]
    If,

    /// The "`in`" keyword.
    #[token("in")]
    In,

    /// The "`<=`" operator.
    #[token("<=")]
    Le,

    /// The "`local`" keyword.
    #[token("local")]
    Local,

    /// The "`<`" operator.
    #[token("<")]
    Lt,

    /// The "`*`" operator.
    #[token("*")]
    Multiply,

    /// The "`~=`" operator.
    #[token("~=")]
    Ne,

    /// Newlines, this variant serves an *internal purpose only*
    /// and should never be used as it is filtered out.
    #[token("\n")]
    Newline,

    /// The "`nil`" keyword.
    #[token("nil")]
    Nil,

    /// The "`not`" operator.
    #[token("not")]
    Not,

    /// Lua numeric literals, stored as components to allow the type
    /// to be [`Hash`].
    #[regex(r#"(-?([0-9]*[.])?[0-9]+)((e|E)(-?)[0-9]+)?"#, |lex| FloatingPointComponents::new(lex.slice().parse::<f64>().expect("Invalid float literal.")).unwrap())]
    Number(FloatingPointComponents),

    /// The "`[`" token.
    #[token("[")]
    OpenBracket,

    /// The "`{`" token.
    #[token("{")]
    OpenCurly,

    /// The "`(`" token.
    #[token("(")]
    OpenParen,

    /// The "`or`" operator.
    #[token("or")]
    Or,

    /// The "`.`" operator.
    #[token(".")]
    Period,

    /// The "`repeat`" keyword.
    #[token("repeat")]
    Repeat,

    /// The "`return`" keyword.
    #[token("return")]
    Return,

    /// The "`;`" token.
    #[token(";")]
    Semicolon,

    /// Lua string literals.
    #[regex(r#""([^\\"]|\\.)*""#, string_literal)]
    StringLiteral(&'a str),

    /// The "`-`" operator.
    #[token("-")]
    Subtract,

    /// The "`then`" keyword.
    #[token("then")]
    Then,

    /// The "`true`" keyword.
    #[token("true")]
    True,

    /// The "`until`" keyword.
    #[token("until")]
    Until,

    /// The "`...`" token.
    #[token("...")]
    Vararg,

    /// The "`while`" keyword.
    #[token("while")]
    While,
}

impl<'a> Token<'a> {
    /// Returns a vector of [`Token`]s.
    #[inline]
    pub fn lex(source: &'a str) -> Vec<Self> {
        Self::lexer(source)
            .filter(|tok| !matches!(tok, &Token::Newline))
            .collect::<Vec<_>>()
    }
}

fn comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Filter<()> {
    // Let parser handle block comments
    if matches!(lex.peekable().peek(), Some(Token::OpenBracket)) {
        return Filter::Emit(());
    }

    // Lexer can eat line comments
    for tok in lex {
        if matches!(tok, Token::Newline) {
            return Filter::Skip;
        }
    }

    Filter::Skip
}

fn string_literal<'a>(lex: &mut Lexer<'a, Token<'a>>) -> &'a str {
    let slice = lex.slice();
    let len = slice.len();

    &slice[1..len - 1]
}

/// Returns a vector of [`Token`]s.
#[inline]
pub fn lex(source: &str) -> Vec<Token> {
    Token::lex(source)
}
