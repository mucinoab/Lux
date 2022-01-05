use crate::{errors::CompileError, expr::Value, token::Token};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Default, Clone, Debug)]
pub struct Environment {
    // TODO try making this a <&str, Value>
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing.clone()),
            values: HashMap::new(),
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
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(CompileError::Interpreter(
            name.place,
            format!("Undefined variable: {}.", name.lexeme),
        ))
    }

    pub fn get(&self, name: &Token) -> Result<Value, CompileError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        // Recurse down the scopes
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().get(name);
        }

        Err(CompileError::Interpreter(
            name.place,
            format!("Undefined variable or function: {}.", name.lexeme),
        ))
    }
}
