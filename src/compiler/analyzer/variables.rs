use std::collections::HashMap;

use crate::compiler::{counter::NameCounter, errors::Location, types::Type};

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Option<Type>,
    pub key: String,
    pub location: Location,

    // pub initialized: bool
    pub mutated: bool,
    pub read: bool,
}

#[derive(Debug)]
pub struct Variables {
    counter: NameCounter,
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
}
impl Variables {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            variables: HashMap::new(),
            counter: NameCounter::new(),
        }
    }
    pub fn increment(&mut self) -> String {
        // self.count += 1;
        // return self.count.to_string();
        self.counter.increment()
    }
    pub fn insert(
        &mut self,
        name: &String,
        mutable: bool,
        data_type: Type,
        location: Location,
    ) -> Result<(), Variable> {
        let key = self.increment();
        let current_state = self.states.last_mut().unwrap();

        let result = self.variables.insert(
            name.clone(),
            Variable {
                location,
                key,
                mutable,
                data_type: Some(data_type),
                mutated: false,
                read: false,
            },
        );

        match result {
            Some(var) => return Err(var),
            None => {}
        }
        current_state.push(name.clone());

        return Ok(());
    }
    pub fn create_state(&mut self) {
        self.states.push(Vec::new());
    }
    pub fn pop_state(&mut self) -> Vec<(String, Variable)> {
        let state = self.states.pop().unwrap();
        let mut vars = Vec::new();
        for key in state {
            vars.push((key.clone(), self.variables.remove(&key).unwrap()));
        }
        return vars;
    }
    pub fn get(&self, key: &String) -> Option<&Variable> {
        return self.variables.get(key);
    }
    pub fn set_key(&mut self, name: &String, key: String) {
        let variable = self.variables.get_mut(name).unwrap();
        variable.key = key
    }
    pub fn write(&mut self, key: &String) -> bool {
        match self.variables.get_mut(key) {
            Some(var) => {
                var.mutated = true;
            }
            None => return false,
        };
        return true;
    }
    pub fn read(&mut self, key: &String) -> bool {
        match self.variables.get_mut(key) {
            Some(var) => {
                var.read = true;
            }
            None => return false,
        };
        return true;
    }
}
