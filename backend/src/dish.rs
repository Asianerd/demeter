use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{callback_result::Result, utils::ValueInt};

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Dish {
    pub id: i32,
    pub name: String,
    pub variants: String,
    pub sizes: String,
    pub species: i32,
}
impl Dish {
    pub async fn create(db: &Pool<Sqlite>, name: String, variants: String, sizes: String, species: i32) -> Result {
        let result = sqlx::query_as::<_, ValueInt>("select count(*) from dish where name = $1;")
            .bind(&name)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 > 0 {
            return Result::Exists;
        }

        // CREATE TABLE dish(id integer primary key autoincrement, name varchar, variants varchar, sizes varchar, species int);

        sqlx::query("insert into dish(name, variants, sizes, species) values($1, $2, $3, $4);")
            .bind(name)
            .bind(variants)
            .bind(sizes)
            .bind(species)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }

    pub async fn edit(db: &Pool<Sqlite>, id: i32, name: String, variants: String, sizes: String, species: i32) -> Result {
        let result = sqlx::query_as::<_, ValueInt>("select count(*) from dish where name = $1;")
            .bind(&name)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 <= 0 {
            return Result::DoesntExist;
        }

        sqlx::query("update dish set name = $1, variants = $2, sizes = $3, species = $4 where id = $5;")
            .bind(name)
            .bind(variants)
            .bind(sizes)
            .bind(species)
            .bind(id)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }

    pub async fn delete(db: &Pool<Sqlite>, id: i32) -> Result {
        let result = sqlx::query_as::<_, ValueInt>("select count(*) from dish where id = $1;")
            .bind(&id)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 <= 0 {
            return Result::DoesntExist;
        }

        sqlx::query("delete from dish where id = $1;")
            .bind(id)
            .execute(db)
            .await
            .unwrap();

        Result::Success
    }

    pub async fn fetch(db: &Pool<Sqlite>, id: i32) -> Option<Dish> {
        let result = sqlx::query_as::<_, ValueInt>("select count(*) from dish where id = $1;")
            .bind(&id)
            .fetch_one(db)
            .await
            .unwrap();

        if result.0 <= 0 {
            return None;
        }

        Some(
            sqlx::query_as("select * from dish where id = $1;")
                .bind(id)
                .fetch_one(db)
                .await
                .unwrap()
        )
    }

    pub async fn fetch_all(db: &Pool<Sqlite>) -> Vec<Dish> {
        sqlx::query_as("select * from dish;")
            .fetch_all(db)
            .await
            .unwrap()
    }
}

#[get("/<name>/<variants>/<sizes>/<species>")]
pub async fn create(db: &State<Pool<Sqlite>>, name: String, variants: String, sizes: String, species: i32) -> String {
    Dish::create(db.inner(), name, variants, sizes, species).await.to_string()
}

#[get("/<id>/<name>/<variants>/<sizes>/<species>")]
pub async fn edit(db: &State<Pool<Sqlite>>, id: i32, name: String, variants: String, sizes: String, species: i32) -> String {
    Dish::edit(db.inner(), id, name, variants, sizes, species).await.to_string()
}

#[get("/<id>")]
pub async fn delete(db: &State<Pool<Sqlite>>, id: i32) -> String {
    Dish::delete(db.inner(), id).await.to_string()
}

#[get("/<id>")]
pub async fn fetch(db: &State<Pool<Sqlite>>, id: i32) -> String {
    serde_json::to_string(&Dish::fetch(db.inner(), id).await).unwrap()
}

#[get("/")]
pub async fn fetch_all(db: &State<Pool<Sqlite>>) -> String {
    serde_json::to_string(&Dish::fetch_all(db.inner()).await).unwrap()
}
