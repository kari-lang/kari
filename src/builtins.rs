use std::{
    collections::HashMap,
    ops::Deref,
};

use crate::{
    functions::Functions,
    parser::{
        Expression,
        List,
        Number,
    },
    stack::{
        self,
        Stack,
    },
};


pub struct Builtins(HashMap<&'static str, Box<Builtin>>);

impl Builtins {
    pub fn new() -> Self {
        let mut b = HashMap::new();

        for builtin in builtins() {
            b.insert(builtin.name(), builtin);
        }

        Self(b)
    }

    pub fn get(&self, name: &str) -> Option<&Builtin> {
        self.0.get(name).map(|builtin| builtin.deref())
    }
}


pub trait Builtin {
    fn name(&self) -> &'static str;
    fn run(&self, _: &mut Stack, _: &mut Functions) -> Result<(), stack::Error>;
}

macro_rules! impl_builtin {
    ($($ty:ident, $name:expr, $fn:ident;)*) => {
        fn builtins() -> Vec<Box<Builtin>> {
            vec![
                $($ty::new(),)*
            ]
        }

        $(
            pub struct $ty;

            impl $ty {
                fn new() -> Box<Builtin> {
                    Box::new($ty)
                }
            }

            impl Builtin for $ty {
                fn name(&self) -> &'static str {
                    $name
                }

                fn run(&self, stack: &mut Stack, functions: &mut Functions)
                    -> Result<(), stack::Error>
                {
                    $fn(stack, functions)
                }
            }
        )*
    }
}

impl_builtin!(
    Print, "print",  print;
    Define,"define", define;

    Add, "+", add;
    Mul, "*", mul;
);

fn print(stack: &mut Stack, _: &mut Functions)
    -> Result<(), stack::Error>
{
    match stack.pop::<Expression>()? {
        Expression::Number(number) => print!("{}", number),
        Expression::List(_)        => unimplemented!(),
        Expression::String(string) => print!("{}", string),
        Expression::Word(_)        => unimplemented!(),
    }

    Ok(())
}

fn define(stack: &mut Stack, functions: &mut Functions)
    -> Result<(), stack::Error>
{
    let mut name = stack.pop::<List>()?;
    assert_eq!(name.len(), 1);
    let name = name.pop().unwrap();

    let name = match name {
        Expression::Word(word) => {
            word
        }
        expression => {
            panic!(
                "Unexpected expression: {:?}\n",
                expression,
            );
        }
    };

    let body = stack.pop::<List>()?;

    functions.define(name, body);

    Ok(())
}

fn add(stack: &mut Stack, _: &mut Functions)
    -> Result<(), stack::Error>
{
    let b = stack.pop::<Number>()?;
    let a = stack.pop::<Number>()?;

    stack.push(Expression::Number(a + b));

    Ok(())
}

fn mul(stack: &mut Stack, _: &mut Functions)
    -> Result<(), stack::Error>
{
    let b = stack.pop::<Number>()?;
    let a = stack.pop::<Number>()?;

    stack.push(Expression::Number(a * b));

    Ok(())
}
