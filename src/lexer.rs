#![allow(dead_code)]

use crate::errors;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub input: &'a str,
    line: usize,
    position: usize,
    current_char: char,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(u64),
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
    Is,
    Numbered,
    Shall,
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
    I,
    Am,
    Declaring,
    A,
    Variable,
    Named,
    Now,
    Period,
    During,
    Not,
    Zero,
    Invoke,
    And,
    It,
    Take,
    Parameters,
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Fn => "function",
            Keyword::Is => "is",
            Keyword::Numbered => "numbered",
            Keyword::Shall => "shall",
            Keyword::Be => "be",
            Keyword::Equal => "equal",
            Keyword::To => "to",
            Keyword::I32 => "integer meaning whole in latin with exactly thirty two bits",
            Keyword::Return => "return",
            Keyword::In => "in",
            Keyword::The => "the",
            Keyword::Case => "case",
            Keyword::That => "that",
            Keyword::Do => "do",
            Keyword::Or => "or",
            Keyword::I => "i",
            Keyword::Am => "am",
            Keyword::Declaring => "declaring",
            Keyword::A => "a",
            Keyword::Variable => "variable",
            Keyword::Named => "named",
            Keyword::Now => "now",
            Keyword::Period => "period",
            Keyword::During => "during",
            Keyword::Not => "not",
            Keyword::Zero => "zero",
            Keyword::Invoke => "invoke",
            Keyword::And => "and",
            Keyword::It => "it",
            Keyword::Take => "take",
            Keyword::Parameters => "parameters",
        }.to_string()
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
            current_char: if let Some(c) = input.chars().nth(0) { c } else { '\0' },
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

                match identifier {
                    "function" => TokenKind::Keyword(Keyword::Fn),
                    "is" => TokenKind::Keyword(Keyword::Is),
                    "shall" => TokenKind::Keyword(Keyword::Shall),
                    "be" => TokenKind::Keyword(Keyword::Be),
                    "equal" => TokenKind::Keyword(Keyword::Equal),
                    "to" => TokenKind::Keyword(Keyword::To),
                    "integer_meaning_whole_in_latin_with_exactly_thirty_two_bits" => TokenKind::Keyword(Keyword::I32),
                    "return" => TokenKind::Keyword(Keyword::Return),
                    "in" => TokenKind::Keyword(Keyword::In),
                    "the" => TokenKind::Keyword(Keyword::The),
                    "case" => TokenKind::Keyword(Keyword::Case),
                    "that" => TokenKind::Keyword(Keyword::That),
                    "do" => TokenKind::Keyword(Keyword::Do),
                    "or" => TokenKind::Keyword(Keyword::Or),
                    "numbered" => TokenKind::Keyword(Keyword::Numbered),
                    "i" => TokenKind::Keyword(Keyword::I),
                    "am" => TokenKind::Keyword(Keyword::Am),
                    "declaring" => TokenKind::Keyword(Keyword::Declaring),
                    "a" => TokenKind::Keyword(Keyword::A),
                    "variable" => TokenKind::Keyword(Keyword::Variable),
                    "named" => TokenKind::Keyword(Keyword::Named),
                    "period" => TokenKind::Keyword(Keyword::Period),
                    "now" => TokenKind::Keyword(Keyword::Now),
                    "during" => TokenKind::Keyword(Keyword::During),
                    "not" => TokenKind::Keyword(Keyword::Not),
                    "zero" => TokenKind::Keyword(Keyword::Zero),
                    "invoke" => TokenKind::Keyword(Keyword::Invoke),
                    "and" => TokenKind::Keyword(Keyword::And),
                    "it" => TokenKind::Keyword(Keyword::It),
                    "take" => TokenKind::Keyword(Keyword::Take),
                    "parameters" => TokenKind::Keyword(Keyword::Parameters),

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

    pub fn peek_token(&self) -> Result<Token, errors::Error> {
        let mut lexer = self.clone();
        lexer.next_token()
    }
}