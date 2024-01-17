use rocket::Rocket;
use rocket::Build;
use rocket::fs::FileServer;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::mysql::MySqlPoolOptions;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions, Cors};

mod models {
    pub mod table;
}
mod routes {
    pub mod tables;
}


fn cors_fairing() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Options,
            // Include other methods as needed
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            // Include other headers as needed
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration to be valid")
}


#[rocket::launch]
async fn rocket() -> Rocket<Build> {

    dotenv::dotenv().ok();

    //let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let database_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");


    let connection_options = MySqlConnectOptions::new()
        .host(&database_host)
        .username(&database_user)
        .password(&database_password)
        .database(&database_name);

    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect_with(connection_options)
        .await
        .expect("Failed to create pool.");

    rocket::build()
        .attach(cors_fairing())
        .manage(pool)
        .mount("/", FileServer::from("./dist"))
        .mount("/api", rocket::routes![
            routes::tables::check_in,
            routes::tables::auth,
            routes::tables::get,
            routes::tables::validate_token,
            routes::tables::description,
        ])
    
}