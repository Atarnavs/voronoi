use actix_web::{
    middleware::Logger, web::{self}, App, HttpServer
};
use serde::Serialize;
// use serde_json;
// use serde::{Serialize, Deserialize};
use voronoi::{info::Info, Diagram};
use std::sync::Mutex;

fn serialize(object: &impl Serialize) -> String {
    let msg = serde_json::to_string(object).unwrap();
    return msg;
}

pub async fn next_step(data: web::Data<Mutex<Diagram>>) -> String {
    data.lock().unwrap().put_next_point_in();
    let points = data.lock().unwrap().convert_points();
    let lines = data.lock().unwrap().convert_lines();

    let body = Info::build(String::from("Artem"), points, lines);

    serialize(&body)
    // "".to_owned()
}
#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    let diagram = web::Data::new(Mutex::new(Diagram::build(10)));
    println!("diagram is: {diagram:?}");
    HttpServer::new( move || {
        let app = App::new()
        .app_data(diagram.clone())
        .route("/next_step", web::get().to(next_step))
        .wrap(Logger::default());
        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
