use crate::component::ComponentInterface;
use crate::component::Framework;
use crate::component::Run;
use crate::component::Value;

pub struct Bar<'a> {
    pub framework: &'a Framework,
}

impl Run for Bar<'_> {
    fn run(&self) {
        self.framework
            .publish(Value::Str("Hello"), "/hello".to_string());
        self.framework
            .publish(Value::Str("From Component B!"), "/world".to_string());
    }
}
