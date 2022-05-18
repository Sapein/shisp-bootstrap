use lazy_static::lazy_static;
use regex::Regex;

pub mod tokens;
use tokens::Token;

lazy_static! {
    static ref NUM: Regex = Regex::new("[1-9]+[0-9]*").unwrap();
    static ref COM: Regex = Regex::new(r";.*\n?+").unwrap();
    static ref STR: Regex = Regex::new("\"[^\"]*\"").unwrap();
    static ref WHI: Regex = Regex::new(r"\s").unwrap();
}

pub fn scan_string(input: String) -> Vec<Token> {
    let mut curr_str = String::new();
    let mut tokens = vec![];

    for (li, line) in input.lines().enumerate() {
        for (ci, c) in line.chars().enumerate() {
            let next_char = line.chars().nth(ci + 1);
            let complex_special = match next_char {
                Some(nc) => {
                    if c == ',' && nc == '@' {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            };

            if WHI.is_match(&c.to_string()) && curr_str.chars().count() > 0 {
                let offset = if ci >= curr_str.chars().count() {
                    curr_str.chars().count()
                } else {
                    0
                };

                tokens.push(Token::new((li, li), (ci - offset, ci - 1), curr_str));
                curr_str = String::new();
                continue;
            }

            if is_special_char(&curr_str, complex_special) && c != '@' {
                tokens.push(Token::new((li, li), (ci - curr_str.chars().count(), ci -1), curr_str));
                curr_str = String::new();
            }

            if is_special_char(&c.to_string(), complex_special) {
                if curr_str.chars().count() > 1 {
                    tokens.push(Token::new((li, li), (ci - curr_str.chars().count(), ci), curr_str));
                    curr_str = String::new();
                } else if curr_str.chars().count() == 1 {
                    if ci > 0 {
                        tokens.push(Token::new((li, li), (ci - 1, ci - 1), curr_str));
                    } else {
                        tokens.push(Token::new((li, li), (ci, ci), curr_str));
                    }
                    curr_str = String::new();
                }

                curr_str.push(c);
                tokens.push(Token::new((li, li), (ci, ci), curr_str));
                curr_str = String::new();
                continue;
            }

            if kinda_special_char(c) {
                if curr_str.chars().count() > 0
                    && curr_str.chars().nth(0).unwrap() != ';'
                    && curr_str.chars().nth(0).unwrap() != '"'
                {
                    let offset = if ci >= curr_str.chars().count() {
                        curr_str.chars().count()
                    } else {
                        0
                    };

                    tokens.push(Token::new((li, li), (ci - offset, ci), curr_str));
                    curr_str = String::new();
                } else if curr_str.chars().count() > 0 && curr_str.chars().nth(0).unwrap() == ';' {
                    curr_str.push(c);
                    continue;
                } else if curr_str.chars().count() > 0 && curr_str.chars().nth(0).unwrap() == '"' {
                    let offset = if ci >= curr_str.chars().count() {
                        curr_str.chars().count()
                    } else {
                        0
                    };
                    curr_str.push(c);
                    tokens.push(Token::new((li,li), (ci - offset, ci), curr_str));
                    curr_str = String::new();
                } else {
                    curr_str.push(c);
                }
                continue;
            }

            curr_str.push(c);
        }
        if curr_str.chars().count() > 0 {
            tokens.push(Token::new(
                (li, li),
                (line.chars().count() - curr_str.chars().count(), line.chars().count() - 1),
                curr_str,
            ));
            curr_str = String::new();
        }
    }
    tokens
}

fn kinda_special_char(c: char) -> bool {
    '\"' == c || ';' == c
}

fn is_special_char(c: &str, next_is_at: bool) -> bool {
    "(" == c || ")" == c || (c == "," && !next_is_at) || (c == ",@")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokens::TokenType;

    #[test]
    fn test_scan_string() {
        let input = vec![
            ",@(a b c)".to_string(),
            ",(a b c)".to_string(),
            "(a@b c)".to_string(),
            "(a,b c)".to_string(),
            "\"test\"".to_string(),
            "1234".to_string(),
            "0123".to_string(),
            ";comment".to_string(),
            "123\n345".to_string(),
            ",123".to_string(),
            ",@123".to_string(),
        ];

        let output = [
            vec![
                Token::new((0,0), (0, 1), ",@".to_string()),
                Token::new((0,0), (2, 2), "(".to_string()),
                Token::new((0,0), (3, 3), "a".to_string()),
                Token::new((0,0), (5, 5), "b".to_string()),
                Token::new((0,0), (7, 7), "c".to_string()),
                Token::new((0,0), (8, 8), ")".to_string()),
            ],
            vec![
                Token::new((0, 0), (0, 0), ",".to_string()),
                Token::new((0, 0), (1, 1), "(".to_string()),
                Token::new((0, 0), (2, 2), "a".to_string()),
                Token::new((0, 0), (4, 4), "b".to_string()),
                Token::new((0, 0), (6, 6), "c".to_string()),
                Token::new((0, 0), (7, 7), ")".to_string()),
            ],
            vec![
                Token::new((0, 0), (0, 0), "(".to_string()),
                Token::new((0, 0), (1, 3), "a@b".to_string()),
                Token::new((0, 0), (5, 5), "c".to_string()),
                Token::new((0, 0), (6, 6), ")".to_string()),
            ],
            vec![
                Token::new((0, 0), (0, 0), "(".to_string()),
                Token::new((0, 0), (1, 1), "a".to_string()),
                Token::new((0, 0), (2, 2), ",".to_string()),
                Token::new((0, 0), (3, 3), "b".to_string()),
                Token::new((0, 0), (5, 5), "c".to_string()),
                Token::new((0, 0), (6, 6), ")".to_string()),
            ],
            vec![Token::new((0, 0), (0, 5), "\"test\"".to_string())],
            vec![Token::new((0, 0), (0, 3), "1234".to_string())],
            vec![Token::new((0, 0), (0, 3), "0123".to_string())],
            vec![Token::new((0, 0), (0, 7), ";comment".to_string())],
            vec![
                Token::new((0, 0), (0, 2), "123".to_string()),
                Token::new((1, 1), (0, 2), "345".to_string()),
            ],
            vec![
                Token::new((0, 0), (0, 0), ",".to_string()),
                Token::new((0, 0), (1, 3), "123".to_string()),
            ],
            vec![
                Token::new((0, 0), (0, 1), ",@".to_string()),
                Token::new((0, 0), (2, 4), "123".to_string()),
            ],
        ];

        for (index, string) in input.iter().enumerate() {
            println!("{}", string);
            assert_eq!(scan_string(string.clone()), output[index]);
        }
    }

    #[test]
    fn test_is_special() {
        assert!(is_special_char("(", false));
        assert!(is_special_char("(", true));

        assert!(is_special_char(")", true));
        assert!(is_special_char(")", false));

        assert!(is_special_char(",", false));
        assert!(!is_special_char(",", true));

        assert!(is_special_char(",@", false));
        assert!(is_special_char(",@", true));

        assert!(!is_special_char("@", false));
        assert!(!is_special_char("@", true));

        assert!(!is_special_char(",a", false));
        assert!(!is_special_char(",a", true));
    }

    #[test]
    fn test_is_kinda_special() {
        assert!(kinda_special_char('"'));
        assert!(kinda_special_char('"'));

        assert!(kinda_special_char(';'));
        assert!(kinda_special_char(';'));
    }
}
