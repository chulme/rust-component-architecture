use actix_web::{middleware, rt::System, web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::serde_as;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread; // 1.0.101

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct TopicsMap {
    #[serde_as(as = "Vec<(_, _)>")]
    topics: HashMap<String, i32>,
}

impl TopicsMap {
    fn new() -> Self {
        return TopicsMap {
            topics: HashMap::new(),
        };
    }
}

/// Get topics
async fn get_topics(topics: web::Data<Mutex<TopicsMap>>, req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    let counter = 0;
    // Increment the counters
    topics
        .lock()
        .unwrap()
        .topics
        .insert("/counter".to_string(), counter);
    topics
        .lock()
        .unwrap()
        .topics
        .insert("/counter1".to_string(), counter + 1);
    let body = serde_json::to_string_pretty(&topics.lock().unwrap().topics).unwrap();
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
            let topics: web::Data<Mutex<TopicsMap>> = web::Data::new(Mutex::new(TopicsMap::new()));

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
