use actix_web::{
    middleware::Logger, web::{self}, App, HttpServer
};
use serde::Serialize;
// use serde_json;
// use serde::{Serialize, Deserialize};
use voronoi::{info::Info, Diagram};
use std::sync::Mutex;
use ftail::Ftail;
use log::LevelFilter;
use std::path::Path;
fn serialize(object: &impl Serialize) -> String {
    let msg = serde_json::to_string(object).unwrap();
    return msg;
}

pub async fn next_step(data: web::Data<Mutex<Diagram>>) -> String {
    data.lock().unwrap().put_next_point_in2();
    let points = data.lock().unwrap().convert_points();
    let lines = data.lock().unwrap().convert_lines();

    let body = Info::build(String::from("Artem"), points, lines);

    serialize(&body)
    // "".to_owned()
}
#[actix_web::main]
async fn main() -> std::io::Result<()>  {
    let path = Path::new("C:/Users/tarna/Fun Projects/puzzles/logs");
    Ftail::new()
    // .console(LevelFilter::Debug)
    .daily_file(path, LevelFilter::Error)
    .init().expect("broke((");
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
