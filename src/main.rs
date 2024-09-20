use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// Mutex is used for thread-safe access to shared data.
use std::sync::Mutex;
mod structs;
use structs::{block::*, blockchain::*};

struct AppState {
    blockchain: Mutex<Blockchain>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialize the blockchain
    let blockchain = Blockchain::new();

    // wrap blockchain in a thread-safe Mutex to share it across multiple thread-safe
    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(blockchain),
    });
    println!("Howdy! Hold onto your butt! The Blockchain is about to start.");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/add_block", web::post().to(add_block))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

async fn add_block(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    // extract the user data from client
    let user = user.into_inner();

    // lock the blockchain to a specific thread prior to adding a block. this ensures only one
    // thread at a time can access and modify the blockchain preventing a race condition
    let mut blockchain = data.blockchain.lock().unwrap();

    // add a new block with the user data
    blockchain.add_block(user);

    HttpResponse::Ok().json(&blockchain.blocks)
}
