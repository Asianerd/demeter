use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, utils::ValueInt};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Desk {
    pub name: String,
    pub capacity: i32
}
impl Desk {
    pub async fn create(db: &Pool<Sqlite>, name: String, capacity: i32) -> Result {
        let result = sqlx::query_as::<_, ValueInt>("select * from desk where name = $1;")
            .bind(&name)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 >= 1 {
            return Result::Exists;
        }

        sqlx::query("insert into desk values($1, $2);")
            .bind(name)
            .bind(capacity)
            .execute(db)
            .await
            .unwrap();

            Result::Success
    }

    pub async fn delete(db: &Pool<Sqlite>, name: String) -> Result {
        let result = sqlx::query_as::<_, ValueInt>("select * from desk where name = $1;")
            .bind(&name)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 <= 0 {
            return Result::DoesntExist;
        }

        sqlx::query("delete from desk where name = $1;")
            .bind(name)
            .execute(db)
            .await
            .unwrap();
        
        Result::Success
    }

    pub async fn fetch(db: &Pool<Sqlite>) -> Vec<Desk> {
        sqlx::query_as("select * from desk;")
            .fetch_all(db)
            .await
            .unwrap()
    }
}

#[get("/<name>/<capacity>")]
pub async fn create(db: &State<Pool<Sqlite>>, name: String, capacity: i32) -> String {
    Desk::create(db.inner(), name, capacity).await.to_string()
}

#[get("/<name>")]
pub async fn delete(db: &State<Pool<Sqlite>>, name: String) -> String {
    Desk::delete(db.inner(), name).await.to_string()
}

#[get("/")]
pub async fn fetch(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Desk::fetch(db.inner()).await).unwrap()
}
