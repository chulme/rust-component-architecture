use crate::component::Component;
use crate::component::Interface;
use serde_json::json;

pub struct Bar {
    pub interface: Interface,
}

impl Component for Bar {
    fn run(&self) {
        let topic = "/hello";
        let data = "boss";
        self.interface.publish(json!(data), topic);
        println!("[ {} ]: Published `{}` to {}.", self.name(), data, topic);
    }

    fn name(&self) -> &str {
        return "bar";
    }
}
