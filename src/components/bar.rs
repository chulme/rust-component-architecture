use crate::component::ComponentInterface;
use crate::component::Framework;
use crate::component::Run;

pub struct Bar<'a> {
    pub framework: &'a Framework,
}

impl Run for Bar<'_> {
    fn run(&self) {
        let res = self.framework.subscribe("/greetings".to_string());
        println!("Bar received: \n\t\"{:?}\"!", res);
    }
}
