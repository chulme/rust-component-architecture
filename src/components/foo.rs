use crate::component::ComponentInterface;
use crate::component::Framework;
use crate::component::Run;
use crate::component::Value;

pub struct Foo<'a> {
    pub framework: &'a Framework,
}

impl Run for Foo<'_> {
    fn run(&self) {
        self.framework.publish(Value::Int(1), "/topic1".to_string());
        self.framework.publish(Value::Int(2), "/topic2".to_string());
        self.framework.publish(Value::Int(3), "/topic1".to_string());
        let res = self.framework.subscribe("/topic1".to_string());
        println!("Received {:?}!", res);
    }
}
