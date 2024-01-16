use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::request::{self, Request, FromRequest};
use sqlx::MySqlPool;
use validator::Validate;
use std::env;
use jsonwebtoken::{encode,decode, Header, EncodingKey, DecodingKey, Validation, errors::ErrorKind};
use chrono::{Utc, Duration};

use crate::models::table::{TableSQL, TableData, TableDTO};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct BearerToken{
    pub token: String,
}

fn token_valid(token: &str) -> Result<Claims, Status> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| match *err.kind() {
        ErrorKind::InvalidToken => Status::Unauthorized, // Token is invalid
        ErrorKind::ExpiredSignature => Status::Unauthorized, // Token has expired
        _ => Status::Unauthorized, // Some other error
    })
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.first() {
            Some(key) if key.starts_with("Bearer ") => {
                request::Outcome::Success(BearerToken{ token: key[7..].to_string() })
            }
            _ => request::Outcome::Error((Status::Unauthorized, ()))
        }
    }
}



#[rocket::post("/check_in")]
pub async fn check_in(bearer: BearerToken,  db_pool: &State<MySqlPool>) -> Result<Json<TableDTO>, (Status, String)> {
    let claims = token_valid(bearer.token.as_str());
    return match claims {
        Ok(claims) => {
            let table = TableSQL::find_by_id(&db_pool, &claims.sub).await
            .map_err(|_| Status::Unauthorized);
            
            
            match table {
                Ok(mut table) => {
                    let result = table.check_in(&db_pool).await;
                    match result {
                        Ok(table) => Ok(Json(table.to_dto())),
                        Err(e) => Err((Status::InternalServerError, format!("Failed to check in: {}", e)))
                    }
                },
                Err(e) => return Err((Status::InternalServerError, format!("Table doesn't exist: {}", e)))
            }
        },
        Err(e) => return Err((Status::Unauthorized, format!("Invalid token: {}", e)))   
    }
    
}

#[rocket::get("/auth")]
pub async fn validate_token(bearer: BearerToken, db_pool: &State<MySqlPool>) -> Result<(), (Status, String)> {
    let claims = token_valid(bearer.token.as_str());

    return match claims {
        Ok(claims) => {
            let _ = TableSQL::find_by_id(&db_pool, &claims.sub).await
            .map_err(|_| (Status::Unauthorized, String::from("Unauthorized")));
            Ok(())
        },
        Err(e) => return Err((Status::Unauthorized, format!("Invalid token: {}", e)))
    }
    
}


#[rocket::get("/<id>")]
pub async fn get(id: &str, db_pool: &State<MySqlPool>) -> Result<Json<TableDTO>, (Status, String)> {

    let table: Result<TableSQL, sqlx::Error> = TableSQL::find_by_id(&db_pool, id).await;
    
    return match table {
        Ok(table) => {
            Ok(Json(table.to_dto()))
        },
        Err(e) => Err((Status::InternalServerError, format!("Table doesn't exist: {}", e)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct DescriptionData {
    pub description: String,
}

#[rocket::post("/description", data = "<form_data>", format = "json")]
pub async fn description(bearer: BearerToken,  form_data: Json<DescriptionData>, db_pool: &State<MySqlPool>) -> Result<(), (Status, String)> {

    let claims = token_valid(bearer.token.as_str());

    let description_data: DescriptionData = form_data.into_inner();

    return match claims {
        Ok(claims) => {
            TableSQL::update_description(&db_pool, claims.sub, description_data.description).await;
            Ok(())
        },
        Err(e) => return Err((Status::Unauthorized, format!("Invalid token: {}", e)))
    }

    
}


#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub data: TableSQL,
}


fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // Token expires in 24 hours
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_ref()))
}

#[rocket::post("/auth", format = "json", data = "<form_data>")]
pub async fn auth(form_data: Json<TableData>, db_pool: &State<MySqlPool>) -> Result<Json<AuthResponse>, (Status, String)> {
    let table_data = form_data.into_inner();

    // Validate the input data
    match table_data.validate() {
        Ok(_) => (),
        Err(e) => return Err((Status::BadRequest, format!("{}", e))),
    }

    let table: Result<TableSQL, sqlx::Error> = TableSQL::find_by_id(&db_pool, table_data.id.as_str()).await;

    match table {
        Ok(table) => {
            if !table.verify_password(table_data.password.as_str()) {
                return Err((Status::Unauthorized, "Invalid password".to_string()));
            }
            match generate_token(table.id.as_str()) {
                Ok(token) => Ok(Json(AuthResponse {
                    token,
                    data: table,
                })),
                Err(e) => Err((Status::InternalServerError, format!("Failed to generate token: {}", e))),
                
            }
        },
        Err(_) => {
            match TableSQL::create(&db_pool, table_data.id, table_data.name, table_data.description, table_data.password).await {
                Ok(table) => {
                    match generate_token(table.id.as_str()) {
                        Ok(token) => Ok(Json(AuthResponse {
                            token,
                            data: table,
                        })),
                        Err(e) => Err((Status::InternalServerError, format!("Failed to generate token: {}", e))),
                    }
                },
                Err(e) => Err((Status::InternalServerError, format!("Failed to create user: {}", e))),
            }
        },
    }

}