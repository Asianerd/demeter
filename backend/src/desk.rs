use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, session::Session, utils::ValueInt};

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Desk {
    pub name: String,
    pub capacity: i32
}
impl Desk {
    pub async fn create(db: &Pool<Sqlite>, name: String, capacity: i32) -> Result {
        match Desk::fetch(db, &name).await {
            Some(_) => Result::Exists,
            None => {
                sqlx::query("insert into desk values($1, $2);")
                    .bind(&name)
                    .bind(capacity)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            }
        }
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

    pub async fn fetch(db: &Pool<Sqlite>, name: &String) -> Option<Desk> {
        match sqlx::query_as("select * from desk where name = $1;")
            .bind(&name)
            .fetch_one(db)
            .await {
            Ok(d) => Some(d),
            Err(e) => {
                println!("desk.rs; fetch({name}); error : {e}");
                None
            }
        }
    }

    pub async fn fetch_all(db: &Pool<Sqlite>) -> Vec<Desk> {
        sqlx::query_as("select * from desk;")
            .fetch_all(db)
            .await
            .unwrap()
    }


    pub async fn fetch_open_session(db: &Pool<Sqlite>, desk: &String) -> Option<Session> {
        match sqlx::query_as("select * from session where desk = $1 and state = 1;")
            .bind(&desk)
            .fetch_one(db)
            .await {
            Ok(s) => {
                Some(s)
            },
            Err(e) => {
                println!("desk.rs; fetch_session({desk}); error: {e}");
                None
            }
        }
    }
}

#[get("/<name>/<capacity>")]
pub async fn create(db: &State<Pool<Sqlite>>, name: String, capacity: i32) -> String {
    Desk::create(db.inner(), urlencoding::decode(&name).unwrap().to_string(), capacity).await.to_string()
}

#[get("/<name>")]
pub async fn delete(db: &State<Pool<Sqlite>>, name: String) -> String {
    Desk::delete(db.inner(), urlencoding::decode(&name).unwrap().to_string()).await.to_string()
}

#[get("/<name>")]
pub async fn fetch(db: &State<Pool<Sqlite>>, name: String) -> String {
    serde_json::to_string(&Desk::fetch(db.inner(), &urlencoding::decode(&name).unwrap().to_string()).await).unwrap()
}

#[get("/")]
pub async fn fetch_all(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Desk::fetch_all(db.inner()).await).unwrap()
}
