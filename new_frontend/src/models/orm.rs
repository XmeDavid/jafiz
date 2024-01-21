
use rocket::http::Status;
use sqlx::MySqlPool;

use crate::models::board::{TableSQL, TableDTO};

pub struct BoardORM;

impl BoardORM {
    pub async fn find_by_id(pool: &MySqlPool, id: &str) -> Result<TableDTO, sqlx::Error> {
        let sql_result: Result<TableSQL, sqlx::Error> = TableSQL::find_by_id(&pool, id).await;
        match sql_result {
            Ok(table) => {
                Ok(table.to_dto())
            },
            Err(e) => Err(e),
            
        }
    }

    pub async fn create(pool: &MySqlPool, id: String, name: String, password: String) -> Result<TableDTO, sqlx::Error> {
        let sql_result: Result<TableSQL, sqlx::Error> = TableSQL::create(&pool, id, name, "I am trying to achieve...".to_string(), password).await;
        match sql_result {
            Ok(table) => {
                Ok(table.to_dto())
            },
            Err(e) => Err(e),
            
        }
    }

    pub async fn check_in(pool: &MySqlPool, id: &str) -> Result<TableDTO, (Status, String)> {
        let table = TableSQL::find_by_id(pool, id).await
            .map_err(|_| Status::NotFound);
        
        match table {
            Ok(mut table) => {
                let result = table.check_in(pool).await;
                match result {
                    Ok(table) => Ok(table.to_dto()),
                    Err(e) => Err((Status::InternalServerError, format!("Failed to check in: {}", e)))
                }
            },
            Err(e) => return Err((Status::NotFound, format!("Table doesn't exist: {}", e)))
        }
    }

    pub async fn change_description(pool: &MySqlPool, id: &str, description: &str) -> Result<TableDTO, (Status, String)> {
        TableSQL::update_description(pool, id.to_string(), description.to_string()).await;
        let table = TableSQL::find_by_id(pool, id).await
            .map_err(|_| Status::NotFound);
        match table {
            Ok(table) => Ok(table.to_dto()),
            Err(e) => Err((Status::NotFound, format!("Unable to get updated board: {}", e)))
        }
    }
}