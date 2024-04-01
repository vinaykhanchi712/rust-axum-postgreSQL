mod route;
mod handler;
mod schema;
mod model;

use std::env;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};


use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::route::create_router;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main(){
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL is not set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool)=>{
            println!("✅Connection to the database is successful!");
            pool
        }

        Err(err)=>{
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET , Method::POST , Method::PATCH , Method::DELETE])
        .allow_credentials(true)
        .allow_headers([ACCEPT,AUTHORIZATION,CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db : pool.clone() })).layer(cors);

    println!("🚀 Server started successfully");
    let listener= tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    axum::serve(listener,app).await.unwrap();


}