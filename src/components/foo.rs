use crate::component::ComponentInterface;
use crate::component::Framework;
use crate::component::Run;
use crate::component::Value;

pub struct Foo<'a> {
    pub framework: &'a Framework,
}

impl Run for Foo<'_> {
    fn run(&self) {
        self.framework
            .publish(Value::Str("Hello from Foo!"), "/greetings".to_string());
    }
}
