mod component;
mod components;
use crate::components::*;
#[allow(unused_imports)]
use component::Component;

use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let topics = HashMap::new();
    let interface: component::Interface = component::Interface {
        topics: RefCell::new(topics),
    };

    let mut components: Vec<Box<dyn component::Component>> = vec![];
    dynamic_generation::create_components!();

    for comp in components {
        println!("Running {}.", comp.as_ref().name());
        comp.as_ref().run();
    }

    for (key, value) in interface.topics.borrow().iter() {
        println!("{}: {:?}", key, value);
    }
}
