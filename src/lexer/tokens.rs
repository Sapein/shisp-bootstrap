#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Composite Tokens
    Atom(String), Number(u128), Str(String), True(bool), False(bool), UnquoteSplice,

    // Single Characters
    LeftParen, RightParen, Comma, At,
    Backquote, SingleQuote, DoubleQuote,
    Comment(String),

    // Whitespace Characters
    Newline, Tab,

    EOF
}

#[derive(Debug, PartialEq)]
pub struct Token {
    row: usize,
    col: usize,

    _type: TokenType,
}

impl Token {
    pub fn new(row: usize, col: usize, raw_characters: String) -> Token {
        Token {
            row: row,
            col: col,

            _type: match raw_characters.as_str() {
                "(" => TokenType::LeftParen,
                ")" => TokenType::RightParen,
                ",@" => TokenType::UnquoteSplice,
                "," => TokenType::Comma,
                "@" => TokenType::At,
                "`" => TokenType::Backquote,
                "'" => TokenType::SingleQuote,
                "#t" => TokenType::True(true),
                "#f" => TokenType::False(false),
                "\n" => TokenType::Newline,
                "\t" => TokenType::Tab,
                _ if super::STR.is_match(raw_characters.as_str()) => TokenType::Str(raw_characters),
                _ if super::NUM.is_match(raw_characters.as_str()) => TokenType::Number(raw_characters.parse::<u128>().unwrap()),
                _ if super::COM.is_match(raw_characters.as_str()) => TokenType::Comment(raw_characters),
                _ => TokenType::Atom(raw_characters),
            },

        }
    }
}
