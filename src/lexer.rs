use regex::Regex;
use lazy_static::lazy_static;

pub mod tokens;
use tokens::Token;

lazy_static! {
    static ref NUM: Regex = Regex::new("[0-9]*").unwrap();
    static ref COM: Regex = Regex::new(r";.*\n").unwrap();
    static ref STR: Regex = Regex::new("\"[^\"]\"").unwrap();
    static ref WHI: Regex = Regex::new(r"\s").unwrap();
}

pub fn scan_string(input: String){
    let mut curr_str = String::new();
    let mut tokens = vec![];
    for (li, line) in input.lines().enumerate() {
        for (ci, c) in line.chars().enumerate() {
            if !WHI.is_match(&c.to_string()) || '"' != c || ';' != c{
                curr_str.push(c);
            } else {
                tokens.push(Token::new(li, ci, c.to_string()));
            }

        }
    }
}
