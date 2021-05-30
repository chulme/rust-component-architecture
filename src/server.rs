use actix_web::{rt::System, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Info {
    topic: String,
    data: Value,
}

// Get all topics
async fn get_topics(
    topics: web::Data<Mutex<Map<String, Value>>>,
    _req: HttpRequest,
) -> HttpResponse {
    let map = topics.lock().unwrap().clone();
    let body = serde_json::to_string_pretty(&map).unwrap();
    return HttpResponse::Ok().body(body);
}

// Publish to topic (creating new topic if none of the same name exists, else update the data)
async fn publish_topic(
    topics: web::Data<Mutex<Map<String, Value>>>,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let result = serde_json::from_str(std::str::from_utf8(&body).unwrap());
    let publish_req: Info = match result {
        Ok(v) => v,
        Err(e) => Info {
            topic: "_".to_string(),
            data: json!(e.to_string()),
        },
    };
    //println!("[ SERVER ]: POST Req: {:?}", publish_req);
    topics
        .lock()
        .unwrap()
        .insert(publish_req.topic, publish_req.data);
    let map = topics.lock().unwrap().clone();
    let body = serde_json::to_string_pretty(&map).unwrap();
    return Ok(HttpResponse::Ok().json(body));
}

#[actix_web::main]
pub async fn start_server() {
    let topics: web::Data<Mutex<Map<String, Value>>> = web::Data::new(Mutex::new(Map::new()));

    let (tx, _) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(move || {
            App::new()
                .app_data(topics.clone()) // add shared state
                .service(web::resource("/").route(web::get().to(get_topics)))
                .service(web::resource("/publish").route(web::post().to(publish_topic)))
        })
        .bind("127.0.0.1:8080")?
        .run();

        let _ = tx.send(srv);
        sys.run()
    });
}
