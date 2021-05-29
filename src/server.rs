use actix_web::{middleware, rt::System, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger;
use serde_json::{json, Map, Value};
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread; // 1.0.101

/// Get topics
async fn get_topics(
    topics: web::Data<Mutex<Map<String, Value>>>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", req);

    let mut map: Map<String, Value> = topics.lock().unwrap().clone();
    map.insert("/int".to_string(), json!(2));
    map.insert("/str".to_string(), json!("hello"));
    map.insert("/dec".to_string(), json!(9.22));

    let body = serde_json::to_string_pretty(&map).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
pub async fn start_server() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let (tx, _) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");

        let srv = HttpServer::new(move || {
            let topics: web::Data<Mutex<Map<String, Value>>> =
                web::Data::new(Mutex::new(Map::new()));

            App::new()
                .app_data(topics.clone()) // add shared state
                // enable logger
                .wrap(middleware::Logger::default())
                // register simple handler
                .service(web::resource("/").to(get_topics))
        })
        .bind("127.0.0.1:8080")?
        .run();

        let _ = tx.send(srv);
        sys.run()
    });
}
