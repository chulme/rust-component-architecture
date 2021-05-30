use crate::component::Component;
use crate::component::Interface;

pub struct Foo {
    pub interface: Interface,
}

impl Component for Foo {
    fn run(&self) {
        // let topic = "/hello";
        // let res = self.interface.subscribe(topic);
        // println!(
        //     "{} is subscribed to {}:\n\tRes:{:?}",
        //     self.name(),
        //     topic,
        //     res
        // );
    }

    fn name(&self) -> &str {
        return "foo";
    }
}
