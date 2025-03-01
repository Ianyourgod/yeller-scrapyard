#![allow(dead_code)]

use crate::errors;

pub struct Lexer<'a> {
    pub input: &'a str,
    line: usize,
    position: usize,
    current_char: char,
    funshun_count: u32,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

/*
funshun1 is integer_thirty_two be equal to main left_brace argc is integer_thirty_two right_brace left_parenthesis
    let variable is integer_thirty_two be equal to left bracket 1 plus 1 right bracket times 2 semicolon
right_parenthesis
*/

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(i64),
    Keyword(Keyword),
    Identifier(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Semicolon,
    Comma,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Fn,
    Let,
    Is,
    Be,
    Equal,
    To,
    I32,
    Return,
    In,
    The,
    Case,
    That,
    Do,
    Or,
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Fn => "funshun".to_string(),
            Keyword::Let => "let".to_string(),
            Keyword::Is => "is".to_string(),
            Keyword::Be => "be".to_string(),
            Keyword::Equal => "equal".to_string(),
            Keyword::To => "to".to_string(),
            Keyword::I32 => "integer thirty two".to_string(),
            Keyword::Return => "return".to_string(),
            Keyword::In => "in".to_string(),
            Keyword::The => "the".to_string(),
            Keyword::Case => "case".to_string(),
            Keyword::That => "that".to_string(),
            Keyword::Do => "do".to_string(),
            Keyword::Or => "or".to_string(),
        }
    }
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Number(n) => n.to_string(),
            TokenKind::Keyword(k) => k.to_string(),
            TokenKind::Identifier(s) => s.to_string(),
            TokenKind::LBrace => "left_brace".to_string(),
            TokenKind::RBrace => "right_brace".to_string(),
            TokenKind::LParen => "left_parenthesis".to_string(),
            TokenKind::RParen => "right_parenthesis".to_string(),
            TokenKind::LBracket => "left_bracket".to_string(),
            TokenKind::RBracket => "right_bracket".to_string(),
            TokenKind::Plus => "plus".to_string(),
            TokenKind::Minus => "minus".to_string(),
            TokenKind::Mul => "times".to_string(),
            TokenKind::Div => "div".to_string(),
            TokenKind::Mod => "mod".to_string(),
            TokenKind::Semicolon => "semicolon".to_string(),
            TokenKind::Comma => "comma".to_string(),
            TokenKind::EOF => "EOF".to_string(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            line: 1,
            position: 0,
            current_char: input.chars().next().unwrap(),
            funshun_count: 1,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = self.input.chars().nth(self.position).unwrap();
        } else {
            self.current_char = '\0';
        }
    }

    pub fn next_token(&mut self) -> Result<Token, errors::Error> {
        while self.current_char.is_whitespace() {
            if self.current_char == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        let line = self.line;

        /*
        funshun1 is integer_thirty_two be equal to main left_bracket argc is integer_thirty_two right_bracket left_parenthesis
            let variable is integer_thirty_two be equal to left brace 1 plus 1} times 2 semicolon
        right_parenthesis
         */

        let token_kind = match self.current_char {
            '0'..='9' => {
                let start = self.position;
                while self.current_char.is_digit(10) {
                    self.advance();
                }
                let end = self.position;
                let number = self.input[start..end].parse().unwrap();
                TokenKind::Number(number)
            }
            '\0' => TokenKind::EOF,
            _ => {
                let start = self.position;
                while self.current_char.is_alphanumeric() || self.current_char == '_' {
                    self.advance();
                }
                let end = self.position;
                let identifier = &self.input[start..end];

                if identifier.is_empty() {
                    return Err(errors::Error::new(
                        errors::ErrorKind::UnexpectedChar(self.current_char),
                        line,
                    ));
                }

                if identifier.len() > 6 {
                    let fn_split = identifier.split_at(7);
                    if fn_split.0 == "funshun" && fn_split.1.parse::<u32>().is_ok() {
                        let fun_count = fn_split.1.parse::<u32>().unwrap();
                        if fun_count != self.funshun_count {
                            return Err(errors::Error::new(
                                errors::ErrorKind::WrongFunshunCount {
                                    expected: self.funshun_count,
                                    found: fun_count,
                                },
                                line,
                            ));
                        }
                        self.funshun_count += 1;
                        return Ok(Token {
                            kind: TokenKind::Keyword(Keyword::Fn),
                            line,
                        });
                    }
                }

                match identifier {
                    "let" => TokenKind::Keyword(Keyword::Let),
                    "is" => TokenKind::Keyword(Keyword::Is),
                    "be" => TokenKind::Keyword(Keyword::Be),
                    "equal" => TokenKind::Keyword(Keyword::Equal),
                    "to" => TokenKind::Keyword(Keyword::To),
                    "integer_thirty_two" => TokenKind::Keyword(Keyword::I32),
                    "return" => TokenKind::Keyword(Keyword::Return),
                    "in" => TokenKind::Keyword(Keyword::In),
                    "the" => TokenKind::Keyword(Keyword::The),
                    "case" => TokenKind::Keyword(Keyword::Case),
                    "that" => TokenKind::Keyword(Keyword::That),
                    "do" => TokenKind::Keyword(Keyword::Do),
                    "or" => TokenKind::Keyword(Keyword::Or),

                    "left_bracket" => TokenKind::LBracket,
                    "right_bracket" => TokenKind::RBracket,
                    "left_brace" => TokenKind::LBrace,
                    "right_brace" => TokenKind::RBrace,
                    "left_parenthesis" => TokenKind::LParen,
                    "right_parenthesis" => TokenKind::RParen,
                    "plus" => TokenKind::Plus,
                    "minus" => TokenKind::Minus,
                    "times" => TokenKind::Mul,
                    "div" => TokenKind::Div,
                    "semicolon" => TokenKind::Semicolon,
                    "mod" => TokenKind::Mod,
                    "comma" => TokenKind::Comma,

                    _ => TokenKind::Identifier(identifier.to_string()),
                }
            }
        };

        Ok(Token {
            kind: token_kind,
            line,
        })
    }
}