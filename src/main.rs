mod component;
mod components;
use crate::components::*;
#[allow(unused_imports)]
use component::Component;
use reqwest;
mod server;
use serde_json;
use serde_json::Value as JsonValue;
use std::io::Read;
use std::thread::spawn;

fn call_server_topics() -> Result<(), Box<dyn std::error::Error>> {
    let mut res = reqwest::get("http://127.0.0.1:8080/")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let v: JsonValue = serde_json::from_str(&body)?;
    println!("Final State: {}", v);
    Ok(())
}

fn main() {
    /*  Init */
    let interface = component::Interface::new();

    let mut components: Vec<Box<dyn component::Component + Send>> = vec![];
    dynamic_generation::create_components!();
    server::start_server();
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
    let _ = call_server_topics();
}
