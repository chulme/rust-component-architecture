use crate::component::Component;
use crate::component::Interface;
use crate::component::Value;
use std::sync::{Arc, Mutex};

pub struct Bar {
    pub interface: Arc<Mutex<Interface>>,
}

impl Component for Bar {
    fn run(&self) {
        let res = self
            .interface
            .lock()
            .unwrap()
            .subscribe("/greetings".to_string());
        println!("Bar received: \n\t\"{:?}\"!", res);

        self.interface.lock().unwrap().publish(
            Value::Str("Bar received your message!"),
            "/response".to_string(),
        );
    }

    fn name(&self) -> &str {
        return "bar";
    }
}
