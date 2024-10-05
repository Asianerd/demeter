use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, utils::decode_uri};

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub id: i32,
    pub name: String
}
impl Species {
    pub async fn create(db: &Pool<Sqlite>, name: String) -> Result {
        match Species::fetch_by_name(db, &name).await {
            Some(_) => {
                Result::Exists
            },
            None => {
                sqlx::query("insert into species(name) values($1)")
                    .bind(&name)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            }
        }
    }
    
    pub async fn delete(db: &Pool<Sqlite>, id: i32) -> Result {
        match Species::fetch(db, id).await {
            Some(_) => {
                sqlx::query("update dish set species = -1 where species = $1;")
                    .bind(&id)
                    .execute(db)
                    .await
                    .unwrap();

                sqlx::query("delete from species where id = $1;")
                    .bind(&id)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            },
            None => Result::DoesntExist
        }
    }

    pub async fn edit(db: &Pool<Sqlite>, id: i32, new_name: String) -> Result {
        match Species::fetch(db, id).await {
            Some(_) => {
                sqlx::query("update species set name = $1 where id = $2;")
                    .bind(new_name)
                    .bind(&id)
                    .execute(db)
                    .await
                    .unwrap();

                Result::Success
            },
            None => Result::DoesntExist
        }
    }

    pub async fn fetch(db: &Pool<Sqlite>, id: i32) -> Option<Species> {
        match sqlx::query_as("select * from species where id = $1;")
            .bind(&id)
            .fetch_one(db)
            .await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("species.rs; fetch({id}); error: {e}");
                None
            }
        }
    }

    pub async fn fetch_by_name(db: &Pool<Sqlite>, name: &String) -> Option<Species> {
        match sqlx::query_as("select * from species where name = $1;")
            .bind(name)
            .fetch_one(db)
            .await {
            Ok(s) => Some(s),
            Err(e) => {
                println!("species.rs; fetch_by_name({name}); error: {e}");
                None
            }
        }
    }

    pub async fn fetch_all(db: &Pool<Sqlite>) -> Vec<Species> {
        sqlx::query_as("select * from species;")
            .fetch_all(db)
            .await
            .unwrap()
    }
}

#[get("/<name>")]
pub async fn create(db: &State<Pool<Sqlite>>, name: String) -> String {
    Species::create(db.inner(), decode_uri(name)).await.to_string()
}

#[get("/<id>")]
pub async fn delete(db: &State<Pool<Sqlite>>, id: i32) -> String {
    Species::delete(db.inner(), id).await.to_string()
}

#[get("/<id>/<new_name>")]
pub async fn edit(db: &State<Pool<Sqlite>>, id: i32, new_name: String) -> String {
    Species::edit(db.inner(), id, decode_uri(new_name)).await.to_string()
}

#[get("/")]
pub async fn fetch_all(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Species::fetch_all(db.inner()).await).unwrap()
}

#[get("/<id>")]
pub async fn fetch(db: &State<Pool<Sqlite>>, id: i32) -> String {
    serde_json::to_string(&Species::fetch(db.inner(), id).await).unwrap()
}

#[get("/<name>")]
pub async fn fetch_by_name(db: &State<Pool<Sqlite>>, name: String) -> String {
    serde_json::to_string(&Species::fetch_by_name(db.inner(), &decode_uri(name)).await).unwrap()
}
