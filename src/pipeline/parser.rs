use std::fmt;

use crate::{
    data::{
        expr::{
            self,
            Expression,
        },
        span::Span,
        token::{
            Token,
            TokenKind,
        },
    },
    pipeline::{
        self,
        tokenizer,
    },
};


pub struct Parser<Tokenizer> {
    tokenizer: Tokenizer,
}

impl<Tokenizer> Parser<Tokenizer> {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Parser {
            tokenizer,
        }
    }
}

impl<Tokenizer> pipeline::Stage for Parser<Tokenizer>
    where Tokenizer: pipeline::Stage<Item=Token, Error=tokenizer::Error>
{
    type Item  = Expression;
    type Error = Error;

    fn next(&mut self) -> Result<Self::Item, Self::Error> {
        let token = self.tokenizer.next()?;

        let (kind, span) = match token.kind {
            TokenKind::ListOpen => {
                let (list, span) = self.parse_list(token.span)?;
                (expr::Kind::List(expr::List::new(list)), span)
            }
            TokenKind::ListClose => {
                return Err(Error::UnexpectedToken(token));
            }
            TokenKind::Number(number) => {
                (expr::Kind::Number(expr::Number::new(number)), token.span)
            }
            TokenKind::String(string) => {
                (expr::Kind::String(expr::String::new(string)), token.span)
            }
            TokenKind::Word(word) => {
                (expr::Kind::Word(expr::Word::new(word)), token.span)
            }
        };

        Ok(
            Expression {
                kind,
                span,
            }
        )
    }
}

impl<Tokenizer> Parser<Tokenizer>
    where Tokenizer: pipeline::Stage<Item=Token, Error=tokenizer::Error>
{
    fn parse_list(&mut self, mut list_span: Span)
        -> Result<(Vec<Expression>, Span), Error>
    {
        let mut expressions = Vec::new();

        loop {
            let token = self.tokenizer.next()?;

            list_span = list_span.merge(token.span.clone());

            let (kind, span) = match token.kind {
                TokenKind::ListOpen => {
                    let (list, span) = self.parse_list(token.span)?;
                    (expr::Kind::List(expr::List::new(list)), span)
                }
                TokenKind::ListClose => {
                    return Ok((expressions, list_span));
                }
                TokenKind::Number(number) => {
                    (expr::Kind::Number(expr::Number::new(number)), token.span)
                }
                TokenKind::String(string) => {
                    (expr::Kind::String(expr::String::new(string)), token.span)
                }
                TokenKind::Word(word) => {
                    (expr::Kind::Word(expr::Word::new(word)), token.span)
                }
            };

            expressions.push(
                Expression {
                    kind,
                    span,
                }
            );
        }
    }
}


#[derive(Debug)]
pub enum Error {
    Tokenizer(tokenizer::Error),
    UnexpectedToken(Token),
    EndOfStream,
}

impl Error {
    pub fn span(self) -> Option<Span> {
        match self {
            Error::Tokenizer(_)           => None,
            Error::UnexpectedToken(token) => Some(token.span),
            Error::EndOfStream            => None,
        }
    }
}

impl From<tokenizer::Error> for Error {
    fn from(from: tokenizer::Error) -> Self {
        match from {
            tokenizer::Error::EndOfStream => Error::EndOfStream,
            error                         => Error::Tokenizer(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Tokenizer(error) => {
                write!(f, "Tokenizer error:\n{:?}", error)?;
            }
            Error::UnexpectedToken(token) => {
                write!(f, "Unexpected token: `{}`", token.kind)?;
            }
            Error::EndOfStream => {
                panic!("Error variant should not be display: {:?}", self);
            }
        }

        Ok(())
    }
}
