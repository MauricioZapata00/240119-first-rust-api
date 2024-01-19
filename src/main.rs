use actix_web::{App, HttpServer};

mod controller;

use controller::songs_controller::get_songs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // println!("Hello, world!");
    
    HttpServer::new(move || {
        App::new()
        .service(get_songs)
    })
    .bind(("127.0.0.1", 8102))?
    .run()
    .await
}