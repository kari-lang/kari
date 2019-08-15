use std::fmt;

use crate::{
    data::{
        expr,
        span::Span,
        token::{
            self,
            Token,
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
    type Item  = expr::Any;
    type Error = Error;

    fn next(&mut self) -> Result<Self::Item, Self::Error> {
        let token = self.tokenizer.next()?;

        let (kind, span) = match token.kind {
            token::Kind::ListOpen => {
                let (list, span) = self.parse_list(token.span)?;
                (expr::Kind::List(list), span)
            }
            token::Kind::ListClose => {
                return Err(Error::UnexpectedToken(token));
            }
            token::Kind::Bool(value) => {
                (expr::Kind::Bool(value), token.span)
            }
            token::Kind::Number(value) => {
                (expr::Kind::Number(value), token.span)
            }
            token::Kind::String(value) => {
                (expr::Kind::String(value), token.span)
            }
            token::Kind::Symbol(value) => {
                (expr::Kind::Symbol(value), token.span)
            }
            token::Kind::Word(value) => {
                (expr::Kind::Word(value), token.span)
            }
        };

        Ok(
            expr::Any {
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
        -> Result<(Vec<expr::Any>, Span), Error>
    {
        let mut expressions = Vec::new();

        loop {
            let token = self.tokenizer.next()?;

            list_span = list_span.merge(token.span.clone());

            let (kind, span) = match token.kind {
                token::Kind::ListOpen => {
                    let (list, span) = self.parse_list(token.span)?;
                    (expr::Kind::List(list), span)
                }
                token::Kind::ListClose => {
                    return Ok((expressions, list_span));
                }
                token::Kind::Bool(value) => {
                    (expr::Kind::Bool(value), token.span)
                }
                token::Kind::Number(value) => {
                    (expr::Kind::Number(value), token.span)
                }
                token::Kind::String(value) => {
                    (expr::Kind::String(value), token.span)
                }
                token::Kind::Symbol(value) => {
                    (expr::Kind::Symbol(value), token.span)
                }
                token::Kind::Word(value) => {
                    (expr::Kind::Word(value), token.span)
                }
            };

            expressions.push(
                expr::Any {
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
    pub fn spans(self, spans: &mut Vec<Span>) {
        match self {
            Error::UnexpectedToken(token) => spans.push(token.span),

            Error::Tokenizer(_) => (),
            Error::EndOfStream  => (),
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
