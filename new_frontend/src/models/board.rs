
use chrono::Datelike;
use sqlx::{FromRow, MySqlPool, Error};
use rocket::serde::{Serialize, Deserialize};
use bcrypt::{hash, DEFAULT_COST};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, FromForm)]
pub struct BoardForm {

    #[validate(length(min = 3))]
    pub name: String,

    pub password: String,

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub january: Vec<bool>,
    pub february: Vec<bool>,
    pub march: Vec<bool>,
    pub april: Vec<bool>,
    pub may: Vec<bool>,
    pub june: Vec<bool>,
    pub july: Vec<bool>,
    pub august: Vec<bool>,
    pub september: Vec<bool>,
    pub october: Vec<bool>,
    pub november: Vec<bool>,
    pub december: Vec<bool>,
}

#[derive(FromRow, Serialize, Deserialize, Clone)]
pub struct TableSQL {
    pub id: String,
    pub name: String,
    pub description: String,
    pub password: String,
    pub january: i32,
    pub february: i32,
    pub march: i32,
    pub april: i32,
    pub may: i32,
    pub june: i32,
    pub july: i32,
    pub august: i32,
    pub september: i32,
    pub october: i32,
    pub november: i32,
    pub december: i32,
}

impl TableSQL {

    pub async fn check_in(&mut self, pool: &MySqlPool,) -> Result<Self, Error> {
        let now = chrono::offset::Utc::now();
        let month = now.month();
        let day = now.day() as usize;
        let day_bit = 1 << (day - 1);

        let (column, current_value) = match month {
            1 => ("january", &mut self.january),
            2 => ("february", &mut self.february),
            3 => ("march", &mut self.march),
            4 => ("april", &mut self.april),
            5 => ("may", &mut self.may),
            6 => ("june", &mut self.june),
            7 => ("july", &mut self.july),
            8 => ("august", &mut self.august),
            9 => ("september", &mut self.september),
            10 => ("october", &mut self.october),
            11 => ("november", &mut self.november),
            12 => ("december", &mut self.december),
            _ => return Err(Error::RowNotFound),
        };

        *current_value |= day_bit;

        let query = format!(
            "UPDATE jafiz SET {} = ? WHERE id = ?",
            column
        );

        let _ = sqlx::query(&query)
            .bind(*current_value)
            .bind(&self.id)
            .execute(pool)
            .await;

        sqlx::query_as!(
                TableSQL,
                "SELECT * FROM jafiz WHERE id = ?",
                self.id
            )
            .fetch_one(pool)
            .await
    }

    pub async fn update_description(pool: &MySqlPool, id: String, description: String){
        let _ = sqlx::query!(
            "UPDATE jafiz SET description = ? WHERE id = ?",
            description,
            id
        )
        .execute(pool)
        .await;
    }

    pub async fn create(
        pool: &MySqlPool, 
        id: String, 
        name: String,
        description: String,
        password: String
    ) -> Result<Self, sqlx::Error> {
        let password = hash(&password, DEFAULT_COST).unwrap();
        let new_table = TableSQL {
            id,
            name,
            password,
            description,
            january: 0,
            february: 0,
            march: 0,
            april: 0,
            may: 0,
            june: 0,
            july: 0,
            august: 0,
            september: 0,
            october: 0,
            november: 0,
            december: 0,
        };

        let _ = sqlx::query!(
            "INSERT INTO jafiz (id, name, password) VALUES (?, ?, ?)",
            new_table.id,
            new_table.name,
            new_table.password
        )
        .execute(pool)
        .await;

        Ok(new_table)
    }

    pub async fn find_by_id(pool: &MySqlPool, id: &str) -> Result<TableSQL, sqlx::Error> {
        sqlx::query_as!(
            TableSQL,
            "SELECT * FROM jafiz WHERE id = ?",
            id
        )
        .fetch_one(pool)
        .await
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }

    pub fn to_dto(&self) -> TableDTO{
        TableDTO {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            january: int_to_bool_array(self.january, 31),
            february: int_to_bool_array(self.february, 28),
            march: int_to_bool_array(self.march, 31),
            april: int_to_bool_array(self.april, 30),
            may: int_to_bool_array(self.may, 31),
            june: int_to_bool_array(self.june, 30),
            july: int_to_bool_array(self.july, 31),
            august: int_to_bool_array(self.august, 31),
            september: int_to_bool_array(self.september, 30),
            october: int_to_bool_array(self.october, 31),
            november: int_to_bool_array(self.november, 30),
            december: int_to_bool_array(self.december, 31),
        }
    }
/* 
    pub fn from_dto(table: TableDTO) -> Self {
        TableSQL {
            id: table.id,
            name: table.name,
            january: bool_array_to_int(table.january),
            february: bool_array_to_int(table.february),
            march: bool_array_to_int(table.march),
            april: bool_array_to_int(table.april),
            may: bool_array_to_int(table.may),
            june: bool_array_to_int(table.june),
            july: bool_array_to_int(table.july),
            august: bool_array_to_int(table.august),
            september: bool_array_to_int(table.september),
            october: bool_array_to_int(table.october),
            november: bool_array_to_int(table.november),
            december: bool_array_to_int(table.december),
            password: "".to_string(),
        }
    }
}

fn bool_array_to_int(bool_array: Vec<bool>) -> i32 {
    let mut num = 0;
    for i in 0..32 {
        if bool_array[i] {
            num |= 1 << i;
        }
    }
    num
} */
} // remove

fn int_to_bool_array(num: i32, limit: i8) -> Vec<bool> {
    let mut bool_array = Vec::new();
    for i in 0..limit {
        bool_array.push(num & (1 << i) != 0);
    }
    bool_array
}
