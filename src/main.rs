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

    dynamic_generation::start_components!();
    for (key, value) in framework.topics.borrow().iter() {
        println!("{}: {:?}", key, value);
    }
}
