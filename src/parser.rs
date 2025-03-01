use crate::lexer::{Token, TokenKind, Keyword, Lexer};
use crate::errors;

pub mod nodes;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, errors::Error> {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token()?;
        Ok(Self { lexer, current_token })
    }

    pub fn parse_program(&mut self) -> Result<nodes::Program, errors::Error> {
        let mut functions = Vec::new();
        while self.current_token.kind != TokenKind::EOF {
            functions.push(self.parse_function_definition()?);
        }
        Ok(nodes::Program { functions })
    }

    fn next(&mut self) -> Result<(), errors::Error> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), errors::Error> {
        if self.current_token.kind == kind {
            self.next()
        } else {
            Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: kind.to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line))
        }
    }

    fn expect_keyword(&mut self, kind: Keyword) -> Result<(), errors::Error> {
        self.expect(TokenKind::Keyword(kind))
    }

    fn parse_type(&mut self) -> Result<nodes::Type, errors::Error> {
        match self.current_token.kind {
            TokenKind::Keyword(Keyword::I32) => {
                self.next()?;
                Ok(nodes::Type::I32)
            }
            _ => Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: "a type".to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line)),
        }
    }

    fn parse_param(&mut self) -> Result<(String, nodes::Type), errors::Error> {
        let name = if let TokenKind::Identifier(name) = &self.current_token.kind {
            name.clone()
        } else {
            return Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: "an identifier".to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line));
        };
        self.next()?;
        self.expect_keyword(Keyword::Is)?;
        Ok((name, self.parse_type()?))
    }

    fn parse_function_definition(&mut self) -> Result<nodes::FunctionDefinition, errors::Error> {
        let line_started = self.current_token.line;
        self.expect_keyword(Keyword::Fn)?;
        self.expect_keyword(Keyword::Is)?;
        let return_type = self.parse_type()?;
        self.expect_keyword(Keyword::Be)?;
        self.expect_keyword(Keyword::Equal)?;
        self.expect_keyword(Keyword::To)?;
        let name = if let TokenKind::Identifier(name) = &self.current_token.kind {
            name.clone()
        } else {
            return Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: "an identifier".to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line));
        };
        self.next()?;
        self.expect(TokenKind::LBracket)?;
        let mut params = Vec::new();
        if self.current_token.kind != TokenKind::RBracket {
            params.push(self.parse_param()?);
            while self.current_token.kind == TokenKind::Comma {
                self.next()?;
                params.push(self.parse_param()?);
            }
        }
        self.expect(TokenKind::RBracket)?;
        let body = self.parse_block()?;
        Ok(nodes::FunctionDefinition {
            name,
            params,
            return_type,
            body,
            line_started,
        })
    }

    fn parse_block(&mut self) -> Result<nodes::Block, errors::Error> {
        let line_started = self.current_token.line;
        self.expect(TokenKind::LParen)?;
        let mut items = Vec::new();
        while self.current_token.kind != TokenKind::RParen {
            items.push(self.parse_block_item()?);
        }
        self.expect(TokenKind::RParen)?;
        Ok(nodes::Block { items, line_started })
    }

    fn parse_block_item(&mut self) -> Result<nodes::BlockItem, errors::Error> {
        match self.current_token.kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_declaration().map(nodes::BlockItem::Declaration),
            _ => self.parse_statement().map(nodes::BlockItem::Statement),
        }
    }

    fn parse_declaration(&mut self) -> Result<nodes::Declaration, errors::Error> {
        let line_started = self.current_token.line;
        self.expect_keyword(Keyword::Let)?;
        let name = if let TokenKind::Identifier(name) = &self.current_token.kind {
            name.clone()
        } else {
            return Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: "an identifier".to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line));
        };
        self.next()?;
        self.expect_keyword(Keyword::Is)?;
        let ty = self.parse_type()?;
        self.expect_keyword(Keyword::Be)?;
        self.expect_keyword(Keyword::Equal)?;
        self.expect_keyword(Keyword::To)?;
        let value = self.parse_expression(0)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(nodes::Declaration { name, ty, value, line_started })
    }

    fn parse_statement(&mut self) -> Result<nodes::Statement, errors::Error> {
        let line_started = self.current_token.line;
        Ok(match self.current_token.kind {
            TokenKind::Keyword(Keyword::Return) => {
                self.next()?;
                let expr = self.parse_expression(0)?;
                self.expect(TokenKind::Semicolon)?;
                nodes::Statement { kind: nodes::StatementKind::Return(expr), line_started }
            }
            TokenKind::Keyword(Keyword::In) => {
                self.next()?;
                self.expect_keyword(Keyword::The)?;
                self.expect_keyword(Keyword::Case)?;
                self.expect_keyword(Keyword::That)?;
                let cond = self.parse_expression(0)?;
                self.expect_keyword(Keyword::Do)?;
                let block = Box::new(self.parse_statement()?);
                let else_block = if self.current_token.kind == TokenKind::Keyword(Keyword::Or) {
                    self.next()?;
                    self.expect_keyword(Keyword::Do)?;
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                nodes::Statement { kind: nodes::StatementKind::If(cond, block, else_block), line_started }
            }
            TokenKind::LParen => {
                let block = self.parse_block()?;
                nodes::Statement { kind: nodes::StatementKind::Block(block), line_started }
            }
            _ => {
                let expr = self.parse_expression(0)?;
                self.expect(TokenKind::Semicolon)?;
                nodes::Statement { kind: nodes::StatementKind::Expression(expr), line_started }
            }
        })
    }

    fn get_prec(&self, kind: &TokenKind) -> i8 {
        match kind {
            TokenKind::Mul | TokenKind::Div | TokenKind::Mod => 50,
            TokenKind::Plus | TokenKind::Minus => 45,
            _ => -1,
        }
    }

    fn token_to_binop(&self, kind: &TokenKind) -> nodes::Binop {
        match kind {
            TokenKind::Plus => nodes::Binop::Add,
            TokenKind::Minus => nodes::Binop::Sub,
            TokenKind::Mul => nodes::Binop::Mul,
            TokenKind::Div => nodes::Binop::Div,
            TokenKind::Mod => nodes::Binop::Mod,
            _ => unreachable!(),
        }
    }

    fn parse_expression(&mut self, min_prec: i8) -> Result<nodes::Expression, errors::Error> {
        let mut left = self.parse_factor()?;

        let mut prec = self.get_prec(&self.current_token.kind);
        while prec >= min_prec {
            let op = self.token_to_binop(&self.current_token.kind);
            self.next()?;

            let right = self.parse_expression(prec + 1)?;

            let line_started = left.line_started;
            left = nodes::Expression {
                kind: nodes::ExpressionKind::Binary(op, Box::new(left), Box::new(right)),
                line_started,
            };

            prec = self.get_prec(&self.current_token.kind);
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<nodes::Expression, errors::Error> {
        match self.current_token.kind {
            TokenKind::Number(n) => {
                let line_started = self.current_token.line;
                self.next()?;
                Ok(nodes::Expression { kind: nodes::ExpressionKind::Number(n), line_started })
            }
            TokenKind::LBrace => {
                self.next()?;
                let expr = self.parse_expression(0)?;
                self.expect(TokenKind::RBrace)?;
                Ok(expr)
            }
            TokenKind::Identifier(ref name) => {
                let name = name.clone();
                let line_started = self.current_token.line;
                self.next()?;
                Ok(nodes::Expression { kind: nodes::ExpressionKind::Variable(name), line_started })
            }
            _ => Err(errors::Error::new(errors::ErrorKind::UnexpectedToken {
                expected: "a factor".to_string(),
                found: self.current_token.kind.to_string(),
            }, self.current_token.line)),
        }
    }
}