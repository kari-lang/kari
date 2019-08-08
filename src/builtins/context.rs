use std::fmt;

use crate::data::{
    expression::{
        Expression,
        List,
    },
    span::Span,
    stack::{
        self,
        Stack,
    },
};


pub trait Context {
    fn stack(&mut self) -> &mut Stack;
    fn define(&mut self, name: String, body: List);
    fn evaluate(&mut self,
        operator:    Option<Span>,
        expressions: &mut Iterator<Item=Expression>,
    )
        -> Result<(), Error>;
}


#[derive(Debug)]
pub enum Error {
    Failure { operator: Span },
    UnknownFunction { name: String, span: Span },
    Stack(stack::Error),
}

impl Error {
    pub fn span(&self) -> Option<Span> {
        match self {
            Error::Failure { operator }         => Some(*operator),
            Error::UnknownFunction { span, .. } => Some(*span),
            Error::Stack(error)                 => error.span(),
        }
    }
}

impl From<stack::Error> for Error {
    fn from(from: stack::Error) -> Self {
        Error::Stack(from)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Failure { .. } => {
                write!(f, "Explicit failure")?;
            }
            Error::UnknownFunction { name, .. } => {
                write!(f, "Unknown function: `{}`", name)?;
            }
            Error::Stack(error) => {
                write!(f, "{}", error)?;
            }
        }

        Ok(())
    }
}