use crate::{errors::CompileError, expr::Value, token::Token};

use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    // TODO try making this a <&str, Value>
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), CompileError> {
        if let Some(e) = self.values.get_mut(name) {
            *e = value;
            Ok(())
        } else {
            Err(CompileError::Interpreter(
                0,
                0,
                "Undefined variable", // TODO put the name of the variable not found
            ))
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, CompileError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            Err(CompileError::Interpreter(
                name.place.0,
                name.place.1,
                "Undefined variable", // TODO put the name of the variable not found
            ))
        }
    }
}
