use crate::errors::MyError;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../errors.rs"]
mod errors;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let shared_data = web::Data::new(AppState {
        health_check_response: String::from("I'm healthy"),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let error_data = web::JsonConfig::default().error_handler(|_err, _req| {
        MyError::InvalidInput("Please provide a valid input".to_string()).into()
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(error_data.clone())
            .configure(course_routes)
            .configure(general_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
