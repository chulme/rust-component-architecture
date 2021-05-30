use reqwest;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;

pub trait Component {
    fn run(&self);
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct Interface {}

impl Interface {
    pub fn new() -> Self {
        return Interface {};
    }

    pub fn subscribe(&self, topic: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let mut res = reqwest::get("http://127.0.0.1:8080/")?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        let topics: Value = serde_json::from_str(&body)?;
        let subscribed_data = serde_json::from_str(&topics[topic].to_string());
        return Ok(subscribed_data.unwrap());
    }
    pub fn publish(&self, data: Value, topic: &str) {
        let client = reqwest::Client::new();

        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("topic".to_string(), json!(topic));
        map.insert("data".to_string(), json!(data));

        let _res = client
            .post("http://127.0.0.1:8080/publish")
            .json(&map)
            .send();
    }
}
