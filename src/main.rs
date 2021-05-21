mod component;
mod components;
use crate::component::Run;
use crate::components::*;

use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let topics = HashMap::new();
    let framework: component::Framework = component::Framework {
        topics: RefCell::new(topics),
    };

    let component_a = foo::Foo {
        framework: &framework,
    };

    let component_b = bar::Bar {
        framework: &framework,
    };

    component_a.run();
    component_b.run();

    for (key, value) in framework.topics.borrow().iter() {
        println!("{}: {:?}", key, value);
    }
}
