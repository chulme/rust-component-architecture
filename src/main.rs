mod component;
mod components;
use crate::components::*;
#[allow(unused_imports)]
use component::Component;

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    /*  Init */
    let topics = HashMap::new();
    let interface: Arc<Mutex<component::Interface>> = Arc::new(Mutex::new(component::Interface {
        topics: RefCell::new(topics),
    }));

    let mut components: Vec<Box<dyn component::Component + Send>> = vec![];
    dynamic_generation::create_components!();

    /*  Launch each component on it's own thread */
    let mut threads = Vec::new();
    for component in components {
        threads.push(spawn(move || {
            println!("Running {}.", component.name());
            component.run();
        }));
    }

    /*  Wait for all the threads to finish */
    for handle in threads {
        handle.join().expect("Panic occurred in thread!");
    }

    /*  Display topics final state */
    for (key, value) in interface.lock().unwrap().topics.borrow().iter() {
        println!("{}: {:?}", key, value);
    }
}
