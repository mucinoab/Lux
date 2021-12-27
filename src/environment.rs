use crate::{errors::CompileError, expr::Value, token::Token};

use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Environment {
    // TODO try making this a <&str, Value>
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_with_enclosing(env: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(env)),
        }
    }

    pub fn define(&mut self, name: &Token, value: Value) {
        self.values.insert(name.lexeme.clone(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), CompileError> {
        if let Some(e) = self.values.get_mut(&name.lexeme) {
            *e = value;
            return Ok(());
        }

        // Recurse down the scopes
        if let Some(enclosing) = self.enclosing.as_mut() {
            return enclosing.assign(name, value);
        }

        Err(CompileError::Interpreter(
            name.place.0,
            name.place.1,
            "Undefined variable", // TODO put the name of the variable not found
        ))
    }

    pub fn get(&self, name: &Token) -> Result<Value, CompileError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        // Recurse down the scopes
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(CompileError::Interpreter(
            name.place.0,
            name.place.1,
            "Undefined variable", // TODO put the name of the variable not found
        ))
    }
}
