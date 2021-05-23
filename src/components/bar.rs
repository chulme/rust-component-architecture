use crate::component::Component;
use crate::component::Interface;

pub struct Bar<'a> {
    pub interface: &'a Interface,
}

impl Component for Bar<'_> {
    fn run(&self) {
        let res = self.interface.subscribe("/greetings".to_string());
        println!("Bar received: \n\t\"{:?}\"!", res);
    }

    fn name(&self) -> &str {
        return "bar";
    }
}
