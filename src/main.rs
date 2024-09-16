use actix_web::(get, post, web, App, HttpResponse, HttpServer, Responder);
use serde::(Deserialize, Serialize);

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

#[get("/")]
async fn index()-> impl Reponder {
    HttpResponse::Ok().body("Rust API Initiatied")
}
