use std::collections::HashMap;

use crate::{
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


pub struct Functions(HashMap<String, Function>);

impl Functions {
    pub fn new() -> Self {
        let mut functions = HashMap::new();

        macro_rules! insert {
            ($name:expr, $builtin:expr) => {
                functions.insert(
                    String::from($name),
                    Function::Builtin(&$builtin)
                );
            }
        }

        insert!("print",  print);
        insert!("define", define);

        insert!("+", add);
        insert!("*", mul);

        Self(functions)
    }

    pub fn define(&mut self, name: String, body: List) {
        self.0.insert(name, Function::List(body));
    }

    pub fn get(&self, name: &str) -> Option<Function> {
        self.0
            .get(name)
            .cloned()
    }
}


#[derive(Clone)]
pub enum Function {
    Builtin(&'static Builtin),
    List(List),
}


pub type Builtin = Fn(&mut Stack, &mut Functions) -> Result<(), stack::Error>;


pub fn print(stack: &mut Stack, _: &mut Functions) -> Result<(), stack::Error> {
    match stack.pop::<Expression>()? {
        Expression::Number(number) => print!("{}", number),
        Expression::List(_)        => unimplemented!(),
        Expression::String(string) => print!("{}", string),
        Expression::Word(_)        => unimplemented!(),
    }

    Ok(())
}

pub fn define(stack: &mut Stack, functions: &mut Functions)
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

pub fn add(stack: &mut Stack, _: &mut Functions) -> Result<(), stack::Error> {
    let b = stack.pop::<Number>()?;
    let a = stack.pop::<Number>()?;

    stack.push(Expression::Number(a + b));

    Ok(())
}

pub fn mul(stack: &mut Stack, _: &mut Functions) -> Result<(), stack::Error> {
    let b = stack.pop::<Number>()?;
    let a = stack.pop::<Number>()?;

    stack.push(Expression::Number(a * b));

    Ok(())
}
