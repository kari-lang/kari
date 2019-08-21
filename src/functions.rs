use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use crate::{
    context::{
        self,
        Context,
    },
    data::{
        span::Span,
        stack::Stack,
        types::{
            Type,
            Typed,
        },
    },
};

pub struct Functions<T: Copy> {
    functions:  HashMap<Signature, T>,
    signatures: HashMap<String, usize>,
}

impl<T> Functions<T> where T: Copy {
    pub fn new() -> Self {
        Self {
            functions:  HashMap::new(),
            signatures: HashMap::new(),
        }
    }

    pub fn with(&mut self,
        name:     String,
        args:     Vec<&'static dyn Type>,
        function: T,
    )
        -> &mut Self
    {
        let args_len = args.len();
        self.functions.insert(Signature { name: name.clone(), args }, function);

        self.signatures
            .entry(name)
            .and_modify(|num| *num = usize::max(*num, args_len))
            .or_insert(args_len);

        self
    }

    pub fn get(&self, name: &str, stack: &Stack) -> Option<T> {
        for n in 0 ..= self.signatures.get(name).map(|n| *n).unwrap_or(0) {
            let mut args: Vec<&'static dyn Type> = stack
                .peek()
                .take(n)
                .map(|expr| expr.get_type())
                .collect();
            args.reverse();

            let function = self.functions
                .get(&Signature { name: String::from(name), args })
                .map(|function| *function);

            if function.is_some() {
                return function;
            }
        }

        None
    }
}


pub struct Signature {
    pub name: String,
    pub args: Vec<&'static dyn Type>,
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        let args_are_equal = self.args
            .iter()
            .zip(other.args.iter())
            .fold(true, |p, (&a1, &a2)| p && a1.eq(a2));

        self.name == other.name
            && args_are_equal
    }
}

impl Eq for Signature {}

impl Hash for Signature {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.name.hash(state);
        // Arguments can't be part of hash, as types can have different names,
        // but still be equal (when one of them is "any").
    }
}


pub type Builtin =
    fn(&mut dyn Context, Span) -> Result<(), context::Error>;
pub type Extension<Host> =
    fn(Rc<RefCell<Host>>, &mut dyn Context, Span) -> Result<(), context::Error>;
