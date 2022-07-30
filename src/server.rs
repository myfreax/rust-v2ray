use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt; // for read_to_end()

#[derive(Deserialize)]
struct Client {
    id: String,
    alterId: u8,
    email: String,
}

async fn read_config() -> Vec<u8> {
    let f = File::open("foo.txt").await;
    let mut contents = vec![];
    let _f = match f {
        Ok(mut file) => file.read_to_end(&mut contents).await,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    return contents;
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

#[get("/config/full")]
async fn full() -> impl Responder {
    HttpResponse::Ok().body("hello world!")
}

// add inbounds client add v2ray config
#[post("/config/inbounds/clients")]
async fn add(client: web::Json<Client>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Welcome id: {} client: {} alterId: {}!",
        client.email, client.id, client.alterId
    ))
}

#[delete("/config/inbounds/clients/{id}")]
async fn remove(paht: web::Path<String>) -> impl Responder {
    let id = paht.into_inner();
    HttpResponse::Ok().body(id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(add).service(remove))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
