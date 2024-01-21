use rocket::response::content::RawHtml;
use rocket::{State, Response, Request, response};
use rocket::http::Status;
use rocket::form::Form;
use sqlx::MySqlPool;
use tokio::{io::AsyncReadExt, fs::File};
use validator::Validate;
use crate::models::board::BoardForm;
use crate::models::orm::BoardORM;
use crate::htmx;
use sha2::{Digest, Sha256};
use rocket::response::Responder;



pub struct HTMXReplace{
    body: String,
    header: String,
}

impl<'r> Responder<'r, 'static> for HTMXReplace {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .sized_body(self.body.len(), std::io::Cursor::new(self.body))
            .raw_header("HX-Replace-Url", self.header)
            .ok()
    }
}


fn get_id(name: String, password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&format!("{}{}", name, password).as_bytes());
    return format!("{:x}", hasher.finalize());
}

#[rocket::get("/")]
pub async fn home() -> Result<RawHtml<String>, (Status, String)> {
    let mut template_file = File::open("./public/template.html").await.map_err(|_| (Status::NotFound, "   >> File './public/template.html' not found".to_string()))?;
    let mut homepage_file = File::open("./public/home.html").await.map_err(|_| (Status::NotFound, "   >> File './public/home.html' not found".to_string()))?;
    
    let mut template = String::new();
    let mut home = String::new();

    template_file.read_to_string(&mut template).await.map_err(|_| (Status::InternalServerError, "   >> Failed to read from './public/template.html'".to_string()))?;
    homepage_file.read_to_string(&mut home).await.map_err(|_| (Status::InternalServerError, "   >> Failed to read from './public/home.html'".to_string()))?;

    Ok(RawHtml(template.replace("{{ body }}", home.as_str())))
}


#[rocket::post("/board", data = "<form_data>")]
pub async fn board_auth(form_data: Form<BoardForm>, pool: &State<MySqlPool>) -> Result<HTMXReplace, (Status, String)> {
    let board_data: BoardForm = form_data.into_inner();

    match board_data.validate() {
        Ok(_) => (),
        Err(e) => return Err((Status::BadRequest, format!("{}", e))),
    }

    let board_id = get_id(board_data.name.clone(), board_data.password.clone());

    let sql_result = BoardORM::find_by_id(&pool, board_id.as_str()).await;

    let table_result = match sql_result {
        Ok(table) => Ok(table),
        Err(_) => BoardORM::create(&pool, board_id.clone(), board_data.name.clone(), board_data.password.clone()).await
    };

    match table_result {
        Ok(table) => {
            match htmx::board::board_page(table.clone(), true).await {
                Ok(body) => {
                    return Ok(HTMXReplace{
                        body: body.0,
                        header: format!("/{}", table.id)
                    });
                },
                Err(e) => return Err((Status::BadRequest, format!("Error generating the page: {}", e.1)))
            }
        },
        Err(e) => Err((Status::BadRequest, format!("Unable to get a board!: {}", e)))
    }
}


#[rocket::get("/<id>")]
pub async fn board(id: &str, pool: &State<MySqlPool>) -> Result<RawHtml<String>, (Status, String)> {

    let sql_result = BoardORM::find_by_id(&pool, id).await;

    match sql_result {
        Ok(table) => {
            let mut template_file = File::open("./public/template.html").await.map_err(|_| (Status::NotFound, "   >> File './public/template.html' not found".to_string()))?;
            let mut template = String::new();
            
            template_file.read_to_string(&mut template).await.map_err(|_| (Status::InternalServerError, "   >> Failed to read from './public/template.html'".to_string()))?;
            
            match htmx::board::board_page(table.clone(), false).await {
                Ok(body) => Ok(RawHtml(template.replace("{{ body }}", body.0.as_str()))),
                Err(e) => return Err((Status::BadRequest, format!("Error generating the page: {}", e.1)))
            }
        }
        Err(_) =>  Err((Status::BadRequest, format!("Unable to get a board!")))
    }

}