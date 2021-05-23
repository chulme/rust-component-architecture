use std::cell::RefCell;
use std::collections::HashMap;

pub trait Component {
    fn run(&self);
    fn name(&self) -> &str;
}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Str(&'static str),
    Int(i32),
}

#[derive(Debug)]
pub struct Interface {
    pub topics: RefCell<HashMap<String, Value>>,
}

impl Interface {
    pub fn publish(&self, data: Value, topic: String) {
        if self.topics.borrow().contains_key(&topic) {
            self.topics.borrow_mut().remove(&topic);
        }
        self.topics.borrow_mut().insert(topic, data);
    }

    pub fn subscribe(&self, topic: String) -> Value {
        if self.topics.borrow().contains_key(&topic) {
            return *self.topics.borrow().get(&topic).unwrap();
        } else {
            println!("{} does not exist!", topic);
        }
        return Value::Int(0);
    }
}
