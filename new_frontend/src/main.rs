#[macro_use] extern crate rocket;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions, Cors};
use rocket::http::Method;
use sqlx::mysql::{MySqlPoolOptions, MySqlConnectOptions};

mod htmx;
mod models;

fn cors_fairing() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration to be valid")
}


#[rocket::launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect_with(MySqlConnectOptions::new()
            .host(&database_host)
            .username(&database_user)
            .password(&database_password)
            .database(&database_name))
        .await
        .expect("Failed to create pool.");

    rocket::build()
        .attach(cors_fairing())
        .manage(pool)
        .mount("/", rocket::routes![
            htmx::index::home,
            htmx::index::board,
            htmx::index::board_auth,
            htmx::operations::check_in,
        ])
    
}