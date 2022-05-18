#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Composite Tokens
    Atom(String), Number(u128), Str(String),
    True(bool), False(bool), UnquoteSplice,

    // Single Characters
    LeftParen, RightParen, Comma, At,
    Backquote, SingleQuote,
    Comment(String),

    // Whitespace Characters
    Newline, Tab,
    Whitespace(String),

    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    row: (usize, usize),
    col: (usize, usize),

    _type: TokenType,
}

impl Token {
    pub fn new(row: (usize, usize), col: (usize, usize), raw_characters: String) -> Token {
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
                _ if super::NUM.is_match(raw_characters.as_str()) => {
                    TokenType::Number(raw_characters.parse::<u128>().unwrap())
                }
                _ if super::COM.is_match(raw_characters.as_str()) => {
                    TokenType::Comment(raw_characters)
                }
                _ if super::WHI.is_match(raw_characters.as_str()) => {
                    TokenType::Whitespace(raw_characters)
                }
                _ => TokenType::Atom(raw_characters),
            },
        }
    }

    pub fn break_apart(self) -> ((usize, usize), (usize, usize), TokenType) {
        (self.row, self.col, self._type)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_breakapart() {
        let token = Token::new((0, 0), (0, 0), "(".to_string());
        let token_str = Token::new((0, 0), (0, 3) , "asdf".to_string());
        let token_num = Token::new((0, 0), (0, 2), "123".to_string());
        assert_eq!(token.break_apart(), ((0,0), (0,0), TokenType::LeftParen));
        assert_eq!(token_str.break_apart(), ((0, 0), (0,3), TokenType::Atom("asdf".to_string())));
        assert_eq!(token_num.break_apart(), ((0,0), (0,2), TokenType::Number(123)))
    }

}
