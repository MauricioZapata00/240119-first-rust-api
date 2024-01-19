use actix_web::{get, Responder, HttpResponse};

#[get("/songs")]
pub async fn get_songs() -> impl Responder{
    HttpResponse::Ok().body("These are songs")
}