use rocket::{response::content::RawHtml, http::Status, State};
use sqlx::MySqlPool;

use crate::models::orm::BoardORM;
use crate::htmx::board;


#[post("/check-in/<id>")]
pub async fn check_in(id: &str, pool: &State<MySqlPool>) -> Result<RawHtml<String>, (Status, String)> {
    let result = BoardORM::check_in(pool, id).await;
    match result {
        Ok(table) => {
            let template = board::months(table);
            Ok(RawHtml(template))
        },
        Err((status, message)) => Err((status, message))
    }
}

#[post("/description/<id>")]
pub async fn change_description(id: &str, pool: &State<MySqlPool>) -> Result<RawHtml<String>, (Status, String)> {
    let result = BoardORM::change_description(pool, id, "New Description").await;
    match result {
        Ok(table) => {
            let template = board::months(table);
            Ok(RawHtml(template))
        },
        Err((status, message)) => Err((status, message))
    }
}