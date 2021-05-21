use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Str(&'static str),
    Int(i32),
}

#[derive(Debug)]
pub struct Framework {
    pub topics: RefCell<HashMap<String, Value>>,
}

pub trait ComponentInterface {
    fn publish(&self, data: Value, topic: String);
    fn subscribe(&self, topic: String) -> Value;
}

pub trait Run {
    fn run(&self);
}

impl ComponentInterface for Framework {
    fn publish(&self, data: Value, topic: String) {
        if self.topics.borrow().contains_key(&topic) {
            self.topics.borrow_mut().remove(&topic);
        }
        self.topics.borrow_mut().insert(topic, data);
    }

    fn subscribe(&self, topic: String) -> Value {
        if self.topics.borrow().contains_key(&topic) {
            return *self.topics.borrow().get(&topic).unwrap();
        } else {
            println!("{} does not exist!", topic);
        }
        return Value::Int(0);
    }
}
