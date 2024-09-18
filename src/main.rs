
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Mutex;
mod blockchain;
use serde::Deserialize

#[derive(Serialize, Deserialize)]
struct Score {
    username: String,
    score: u32
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(submit_score)
            .service(get_scores)
    })
    .bind("localhost:8000")?
    .run()
    .await
}



