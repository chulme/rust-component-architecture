use crate::component::Component;
use crate::component::Interface;
use crate::component::Value;

pub struct Foo<'a> {
    pub interface: &'a Interface,
}

impl Component for Foo<'_> {
    fn run(&self) {
        self.interface
            .publish(Value::Str("Hello from Foo!"), "/greetings".to_string());
    }

    fn name(&self) -> &str {
        return "foo";
    }
}
