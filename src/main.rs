
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
mod blockchain;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("localhost:8000")?
    .run()
    .await
}



