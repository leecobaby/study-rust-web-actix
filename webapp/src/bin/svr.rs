#[path = "../mod.rs"]
mod wa;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use wa::{errors, handlers, models, routers};

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_post = env::var("HOST_PORT").expect("HOST:PORT must be set");
    println!("Listening on {}", &host_post);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_post)?
    .run()
    .await
}
