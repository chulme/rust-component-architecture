mod component;
mod components;
#[allow(unused_imports)]
use crate::component::Run;
use crate::components::*;

use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let topics = HashMap::new();
    let framework: component::Framework = component::Framework {
        topics: RefCell::new(topics),
    };

    let mut components: Vec<Box<dyn component::Run>> = vec![];
    dynamic_generation::create_components!();

    for comp in components {
        comp.as_ref().run();
    }

    for (key, value) in framework.topics.borrow().iter() {
        println!("{}: {:?}", key, value);
    }
}
