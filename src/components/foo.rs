use crate::component::Component;
use crate::component::Interface;
use crate::component::Value;
use std::sync::{Arc, Mutex};

pub struct Foo {
    pub interface: Arc<Mutex<Interface>>,
}

impl Component for Foo {
    fn run(&self) {
        println!("Publishing a message from foo.");
        self.interface
            .lock()
            .unwrap()
            .publish(Value::Str("Hello from Foo!"), "/greetings".to_string());
    }

    fn name(&self) -> &str {
        return "foo";
    }
}
